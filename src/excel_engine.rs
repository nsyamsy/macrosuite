use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};
use calamine::{open_workbook_auto, Reader, Data};
use rust_xlsxwriter::{Workbook as XlsxWriterWorkbook, Format};

use crate::models::{ItemStatus, SheetItem};

/// Reads sheet information from .xlsx or .xlsm file,
/// accurately detecting sheet visibility (visible vs hidden) from xl/workbook.xml.
pub fn read_sheet_info(excel_path: &Path) -> Result<Vec<SheetItem>, String> {
    if !excel_path.exists() {
        return Err("Excel file not found".to_string());
    }

    // 1. Read visibility from xl/workbook.xml inside the zip
    let file = File::open(excel_path).map_err(|e| format!("Cannot open file: {e}"))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Not a valid Excel zip package: {e}"))?;

    let mut hidden_map = std::collections::HashMap::new();
    let mut sheet_order = Vec::new();

    if let Ok(mut wb_xml) = zip.by_name("xl/workbook.xml") {
        let mut content = String::new();
        if wb_xml.read_to_string(&mut content).is_ok() {
            // Parse <sheet ... name="..." ... state="hidden" ... />
            let re_sheet = regex_lite_sheets(&content);
            for (name, is_hidden) in re_sheet {
                hidden_map.insert(name.clone(), is_hidden);
                sheet_order.push(name);
            }
        }
    }

    // 2. Open with calamine to get sheets and row counts
    let mut workbook = open_workbook_auto(excel_path)
        .map_err(|e| format!("Failed to read Excel workbook: {e}"))?;

    let calamine_sheets = workbook.sheet_names().to_vec();

    let mut items = Vec::new();
    let sheets_to_iterate = if !sheet_order.is_empty() {
        sheet_order
    } else {
        calamine_sheets
    };

    for (idx, name) in sheets_to_iterate.into_iter().enumerate() {
        let is_hidden = hidden_map.get(&name).copied().unwrap_or(false);
        let mut row_count = 0;

        if let Ok(range) = workbook.worksheet_range(&name) {
            row_count = range.get_size().0;
        }

        items.push(SheetItem {
            index: idx + 1,
            original_name: name.clone(),
            new_name: name,
            is_hidden,
            is_selected: !is_hidden, // Select active/visible sheets by default
            row_count,
            status: ItemStatus::Ready,
        });
    }

    Ok(items)
}

fn regex_lite_sheets(xml: &str) -> Vec<(String, bool)> {
    let mut sheets = Vec::new();
    let mut pos = 0;
    while let Some(start) = xml[pos..].find("<sheet ") {
        let abs_start = pos + start;
        if let Some(end) = xml[abs_start..].find("/>") {
            let tag = &xml[abs_start..abs_start + end + 2];
            let name = extract_attr(tag, "name");
            let state = extract_attr(tag, "state");
            let is_hidden = state.map(|s| s == "hidden" || s == "veryHidden").unwrap_or(false);
            if let Some(n) = name {
                sheets.push((n, is_hidden));
            }
            pos = abs_start + end + 2;
        } else {
            break;
        }
    }
    sheets
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    if let Some(start) = tag.find(&needle) {
        let val_start = start + needle.len();
        if let Some(end) = tag[val_start..].find('"') {
            return Some(tag[val_start..val_start + end].to_string());
        }
    }
    None
}

/// Renames sheets in xl/workbook.xml and produces a new workbook
pub fn rename_sheets(
    excel_path: &Path,
    output_path: &Path,
    renames: &[(String, String)],
) -> Result<usize, String> {
    let src_file = File::open(excel_path).map_err(|e| e.to_string())?;
    let mut src_zip = ZipArchive::new(src_file).map_err(|e| e.to_string())?;

    let dst_file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut dst_zip = ZipWriter::new(dst_file);

    let mut renamed_count = 0;

    for i in 0..src_zip.len() {
        let mut entry = src_zip.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();

        let options = SimpleFileOptions::default()
            .compression_method(entry.compression());

        dst_zip.start_file(&name, options).map_err(|e| e.to_string())?;

        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if name == "xl/workbook.xml" {
            let mut content = String::from_utf8_lossy(&buffer).to_string();
            for (old_name, new_name) in renames {
                if old_name != new_name && !new_name.trim().is_empty() {
                    let pattern = format!("name=\"{old_name}\"");
                    let replacement = format!("name=\"{new_name}\"");
                    if content.contains(&pattern) {
                        content = content.replace(&pattern, &replacement);
                        renamed_count += 1;
                    }
                }
            }
            dst_zip.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        } else {
            dst_zip.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }

    dst_zip.finish().map_err(|e| e.to_string())?;
    Ok(renamed_count)
}

/// Removes all drawing/picture references from worksheets
pub fn clean_pictures_and_shapes(excel_path: &Path, output_path: &Path) -> Result<usize, String> {
    let src_file = File::open(excel_path).map_err(|e| e.to_string())?;
    let mut src_zip = ZipArchive::new(src_file).map_err(|e| e.to_string())?;

    let dst_file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut dst_zip = ZipWriter::new(dst_file);

    let mut removed_count = 0;

    for i in 0..src_zip.len() {
        let mut entry = src_zip.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();

        // Skip standalone drawing files
        if name.starts_with("xl/drawings/drawing") || name.starts_with("xl/media/") {
            removed_count += 1;
            continue;
        }

        let options = SimpleFileOptions::default()
            .compression_method(entry.compression());

        dst_zip.start_file(&name, options).map_err(|e| e.to_string())?;

        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if name.starts_with("xl/worksheets/sheet") && name.ends_with(".xml") {
            let mut content = String::from_utf8_lossy(&buffer).to_string();
            // Remove <drawing r:id="..."/> and <legacyDrawing r:id="..."/>
            content = remove_tag_pattern(&content, "<drawing ", "/>");
            content = remove_tag_pattern(&content, "<legacyDrawing ", "/>");
            dst_zip.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        } else if name.starts_with("xl/worksheets/_rels/sheet") && name.ends_with(".xml.rels") {
            let mut content = String::from_utf8_lossy(&buffer).to_string();
            content = remove_drawing_rels(&content);
            dst_zip.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        } else {
            dst_zip.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }

    dst_zip.finish().map_err(|e| e.to_string())?;
    Ok(removed_count)
}

/// Removes all conditional formatting blocks from all worksheets
pub fn clean_conditional_formatting(excel_path: &Path, output_path: &Path) -> Result<usize, String> {
    let src_file = File::open(excel_path).map_err(|e| e.to_string())?;
    let mut src_zip = ZipArchive::new(src_file).map_err(|e| e.to_string())?;

    let dst_file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut dst_zip = ZipWriter::new(dst_file);

    let mut rules_removed = 0;

    for i in 0..src_zip.len() {
        let mut entry = src_zip.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();

        let options = SimpleFileOptions::default()
            .compression_method(entry.compression());

        dst_zip.start_file(&name, options).map_err(|e| e.to_string())?;

        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        if name.starts_with("xl/worksheets/sheet") && name.ends_with(".xml") {
            let content = String::from_utf8_lossy(&buffer).to_string();
            let (cleaned, count) = remove_tag_block(&content, "<conditionalFormatting", "</conditionalFormatting>");
            rules_removed += count;
            dst_zip.write_all(cleaned.as_bytes()).map_err(|e| e.to_string())?;
        } else {
            dst_zip.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }

    dst_zip.finish().map_err(|e| e.to_string())?;
    Ok(rules_removed)
}

fn remove_tag_pattern(xml: &str, start_tag: &str, end_tag: &str) -> String {
    let mut res = xml.to_string();
    while let Some(start) = res.find(start_tag) {
        if let Some(end) = res[start..].find(end_tag) {
            let abs_end = start + end + end_tag.len();
            res.replace_range(start..abs_end, "");
        } else {
            break;
        }
    }
    res
}

fn remove_tag_block(xml: &str, start_tag: &str, end_tag: &str) -> (String, usize) {
    let mut res = xml.to_string();
    let mut count = 0;
    while let Some(start) = res.find(start_tag) {
        if let Some(end) = res[start..].find(end_tag) {
            let abs_end = start + end + end_tag.len();
            res.replace_range(start..abs_end, "");
            count += 1;
        } else {
            break;
        }
    }
    (res, count)
}

fn remove_drawing_rels(xml: &str) -> String {
    let mut res = xml.to_string();
    while let Some(start) = res.find("<Relationship ") {
        if let Some(end) = res[start..].find("/>") {
            let abs_end = start + end + 2;
            let tag = &res[start..abs_end];
            if tag.contains("drawing") || tag.contains("printerSettings") {
                res.replace_range(start..abs_end, "");
                continue;
            }
        }
        break;
    }
    res
}

/// Combines selected sheets into a consolidated single worksheet
pub fn combine_sheets(
    excel_path: &Path,
    output_path: &Path,
    sheets_to_combine: &[String],
    include_source_column: bool,
) -> Result<usize, String> {
    let mut reader = open_workbook_auto(excel_path)
        .map_err(|e| format!("Cannot read source workbook: {e}"))?;

    let mut new_wb = XlsxWriterWorkbook::new();
    let worksheet = new_wb.add_worksheet();
    worksheet.set_name("Combined Data").map_err(|e| e.to_string())?;

    let header_fmt = Format::new().set_bold();
    let mut current_row: u32 = 0;
    let mut total_rows_combined = 0;
    let mut is_first_sheet = true;

    for sheet_name in sheets_to_combine {
        if let Ok(range) = reader.worksheet_range(sheet_name) {
            let (height, width) = range.get_size();
            if height == 0 || width == 0 {
                continue;
            }

            let start_r = if is_first_sheet { 0 } else { 1 }; // skip header on subsequent sheets

            for r in start_r..height {
                let mut col_offset: u16 = 0;
                if include_source_column {
                    if current_row == 0 {
                        worksheet.write_string_with_format(0, 0, "Source Sheet", &header_fmt).ok();
                    } else {
                        worksheet.write_string(current_row, 0, sheet_name).ok();
                    }
                    col_offset = 1;
                }

                for c in 0..width {
                    let col_idx = col_offset + c as u16;
                    if let Some(val) = range.get((r, c)) {
                        match val {
                            Data::Empty => {}
                            Data::String(s) => {
                                if current_row == 0 {
                                    worksheet.write_string_with_format(current_row, col_idx, s, &header_fmt).ok();
                                } else {
                                    worksheet.write_string(current_row, col_idx, s).ok();
                                }
                            }
                            Data::Float(f) => {
                                worksheet.write_number(current_row, col_idx, *f).ok();
                            }
                            Data::Int(i) => {
                                worksheet.write_number(current_row, col_idx, *i as f64).ok();
                            }
                            Data::Bool(b) => {
                                worksheet.write_boolean(current_row, col_idx, *b).ok();
                            }
                            Data::DateTime(dt) => {
                                worksheet.write_number(current_row, col_idx, dt.as_f64()).ok();
                            }
                            Data::Error(err) => {
                                worksheet.write_string(current_row, col_idx, format!("{err:?}")).ok();
                            }
                            _ => {}
                        }
                    }
                }
                current_row += 1;
                total_rows_combined += 1;
            }
            is_first_sheet = false;
        }
    }

    new_wb.save(output_path).map_err(|e| format!("Failed to save combined workbook: {e}"))?;
    Ok(total_rows_combined)
}

/// Exports only ACTIVE/VISIBLE sheets to PDF using PowerShell and Excel Automation.
/// Hidden sheets are strictly excluded.
pub fn export_visible_sheets_to_pdf(
    excel_path: &Path,
    output_folder: &Path,
) -> Result<Vec<String>, String> {
    if !excel_path.exists() {
        return Err("Excel file does not exist".to_string());
    }
    if !output_folder.exists() {
        fs::create_dir_all(output_folder).map_err(|e| e.to_string())?;
    }

    let excel_abs = fs::canonicalize(excel_path)
        .map_err(|e| format!("Path error: {e}"))?
        .to_string_lossy()
        .replace("\\\\?\\", "");

    let out_abs = fs::canonicalize(output_folder)
        .map_err(|e| format!("Path error: {e}"))?
        .to_string_lossy()
        .replace("\\\\?\\", "");

    // PowerShell script that opens Excel headless, iterates ONLY Visible sheets ($_.Visible -eq -1),
    // and exports them as PDF.
    let ps_script = format!(
        r#"
$ErrorActionPreference = 'Stop'
$excel = New-Object -ComObject Excel.Application
$excel.Visible = $false
$excel.DisplayAlerts = $false
try {{
    $wb = $excel.Workbooks.Open('{excel_abs}')
    $exported = @()
    foreach ($sh in $wb.Sheets) {{
        # xlSheetVisible = -1 (Excel.XlSheetVisibility)
        if ($sh.Visible -eq -1) {{
            $safeName = $sh.Name -replace '[\\/:*?"<>|]', '_'
            $pdfPath = Join-Path '{out_abs}' "$safeName.pdf"
            # 0 = xlTypePDF
            $sh.ExportAsFixedFormat(0, $pdfPath)
            $exported += $sh.Name
            Write-Output "EXPORTED:$($sh.Name)"
        }}
    }}
    $wb.Close($false)
}} finally {{
    $excel.Quit()
    [System.Runtime.Interopservices.Marshal]::ReleaseComObject($excel) | Out-Null
}}
"#
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_script])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {e}"))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PDF export failed (is Microsoft Excel installed?): {err_msg}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut exported_sheets = Vec::new();
    for line in stdout.lines() {
        if let Some(sheet_name) = line.strip_prefix("EXPORTED:") {
            exported_sheets.push(sheet_name.trim().to_string());
        }
    }

    Ok(exported_sheets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_all_macros_sheet_info() {
        let path = Path::new("All Macros.xlsm");
        if path.exists() {
            let sheets = read_sheet_info(path).expect("Should parse All Macros.xlsm");
            assert_eq!(sheets.len(), 10);
            
            // Verify that Getfilenames is detected as hidden
            let getfilenames = sheets.iter().find(|s| s.original_name == "Getfilenames").expect("Getfilenames sheet should exist");
            assert!(getfilenames.is_hidden, "Getfilenames sheet must be identified as hidden");

            // Verify active sheets
            let create_folders = sheets.iter().find(|s| s.original_name == "Create Folders").expect("Create Folders should exist");
            assert!(!create_folders.is_hidden, "Create Folders must be active");
        }
    }
}
