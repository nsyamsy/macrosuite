use std::fs;
use std::path::Path;
use crate::models::{DeleteTargetType, ItemStatus, RenameItem};

pub fn create_folders(base_dir: &Path, names: &[String]) -> Vec<(String, Result<(), String>)> {
    let mut results = Vec::new();
    for name in names {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            continue;
        }
        let target_path = base_dir.join(trimmed);
        if target_path.exists() {
            results.push((trimmed.to_string(), Err("Folder already exists".to_string())));
        } else {
            match fs::create_dir_all(&target_path) {
                Ok(_) => results.push((trimmed.to_string(), Ok(()))),
                Err(e) => results.push((trimmed.to_string(), Err(e.to_string()))),
            }
        }
    }
    results
}

pub fn delete_items(
    base_dir: &Path,
    names: &[String],
    target_type: DeleteTargetType,
) -> Vec<(String, Result<(), String>)> {
    let mut results = Vec::new();
    for name in names {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            continue;
        }
        let target_path = base_dir.join(trimmed);
        if !target_path.exists() {
            results.push((trimmed.to_string(), Err("Item not found".to_string())));
            continue;
        }

        let is_dir = target_path.is_dir();
        match target_type {
            DeleteTargetType::FilesOnly => {
                if is_dir {
                    results.push((trimmed.to_string(), Err("Skipped (is a directory)".to_string())));
                } else {
                    match fs::remove_file(&target_path) {
                        Ok(_) => results.push((trimmed.to_string(), Ok(()))),
                        Err(e) => results.push((trimmed.to_string(), Err(e.to_string()))),
                    }
                }
            }
            DeleteTargetType::FoldersOnly => {
                if !is_dir {
                    results.push((trimmed.to_string(), Err("Skipped (is a file)".to_string())));
                } else {
                    match fs::remove_dir_all(&target_path) {
                        Ok(_) => results.push((trimmed.to_string(), Ok(()))),
                        Err(e) => results.push((trimmed.to_string(), Err(e.to_string()))),
                    }
                }
            }
            DeleteTargetType::Both => {
                let res = if is_dir {
                    fs::remove_dir_all(&target_path)
                } else {
                    fs::remove_file(&target_path)
                };
                match res {
                    Ok(_) => results.push((trimmed.to_string(), Ok(()))),
                    Err(e) => results.push((trimmed.to_string(), Err(e.to_string()))),
                }
            }
        }
    }
    results
}

pub fn scan_subfolders(base_dir: &Path) -> Result<Vec<String>, String> {
    if !base_dir.exists() || !base_dir.is_dir() {
        return Err("Directory does not exist or is not a folder".to_string());
    }

    let mut folders = Vec::new();
    let entries = fs::read_dir(base_dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    folders.push(name.to_string());
                }
            }
        }
    }
    folders.sort_by_key(|a| a.to_lowercase());
    Ok(folders)
}

pub fn scan_files(base_dir: &Path, extension_filter: &str) -> Result<Vec<String>, String> {
    if !base_dir.exists() || !base_dir.is_dir() {
        return Err("Directory does not exist or is not a folder".to_string());
    }

    let filter = extension_filter.trim().to_lowercase();
    let filter = filter.trim_start_matches('*').trim_start_matches('.');

    let mut files = Vec::new();
    let entries = fs::read_dir(base_dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if filter.is_empty() || filter == "*" {
                        files.push(name.to_string());
                    } else if let Some(ext) = Path::new(name).extension().and_then(|e| e.to_str()) {
                        if ext.to_lowercase() == filter {
                            files.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    files.sort_by_key(|a| a.to_lowercase());
    Ok(files)
}

pub fn execute_rename_batch(base_dir: &Path, items: &[RenameItem]) -> Vec<RenameItem> {
    let mut updated = Vec::new();
    for item in items {
        let mut cur = item.clone();
        let orig = item.original_name.trim();
        let new_name = item.new_name.trim();

        if orig.is_empty() || new_name.is_empty() {
            cur.status = ItemStatus::Warning("Empty name".to_string());
            updated.push(cur);
            continue;
        }

        if orig == new_name {
            cur.status = ItemStatus::Ready;
            updated.push(cur);
            continue;
        }

        let from_path = base_dir.join(orig);
        let to_path = base_dir.join(new_name);

        if !from_path.exists() {
            cur.status = ItemStatus::Error("Source file not found".to_string());
            updated.push(cur);
            continue;
        }

        if to_path.exists() {
            cur.status = ItemStatus::Error("Target name already exists".to_string());
            updated.push(cur);
            continue;
        }

        match fs::rename(&from_path, &to_path) {
            Ok(_) => {
                cur.status = ItemStatus::Success("Renamed successfully".to_string());
            }
            Err(e) => {
                cur.status = ItemStatus::Error(e.to_string());
            }
        }
        updated.push(cur);
    }
    updated
}
