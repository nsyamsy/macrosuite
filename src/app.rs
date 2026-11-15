use std::path::{Path, PathBuf};
use eframe::egui::{self, Align, Color32, Layout, RichText, ScrollArea, Stroke, TextEdit, Ui};
use arboard::Clipboard;

use crate::excel_engine;
use crate::fs_engine;
use crate::models::{ActiveTab, DeleteTargetType, ItemStatus, LogLevel, LogMessage, RenameItem, SheetItem, TabCategory};
use crate::theme::{self, COLOR_ERROR, COLOR_SUCCESS, COLOR_TEAL, COLOR_WARNING};

pub struct MacroSuiteApp {
    // Theme
    pub dark_mode: bool,

    // Navigation
    pub current_category: TabCategory,
    pub current_tab: ActiveTab,

    // File & Folder paths
    pub fs_base_dir: String,
    pub fs_extension_filter: String,
    pub folder_names_input: String,
    pub delete_names_input: String,
    pub delete_type: DeleteTargetType,
    pub delete_confirmed: bool,

    // Rename state
    pub rename_folder_items: Vec<RenameItem>,
    pub rename_file_items: Vec<RenameItem>,
    pub transform_prefix: String,
    pub transform_suffix: String,
    pub transform_find: String,
    pub transform_replace: String,

    // Excel state
    pub excel_file_path: String,
    pub excel_sheets: Vec<SheetItem>,
    pub clean_pictures: bool,
    pub clean_cond_formatting: bool,
    pub combine_include_source_col: bool,
    pub pdf_output_folder: String,

    // Activity Log
    pub logs: Vec<LogMessage>,
    pub show_log_panel: bool,
}

impl Default for MacroSuiteApp {
    fn default() -> Self {
        let mut app = Self {
            dark_mode: true,
            current_category: TabCategory::FilesAndFolders,
            current_tab: ActiveTab::RenameFiles,

            fs_base_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            fs_extension_filter: "*.*".to_string(),
            folder_names_input: String::new(),
            delete_names_input: String::new(),
            delete_type: DeleteTargetType::FilesOnly,
            delete_confirmed: false,

            rename_folder_items: Vec::new(),
            rename_file_items: Vec::new(),
            transform_prefix: String::new(),
            transform_suffix: String::new(),
            transform_find: String::new(),
            transform_replace: String::new(),

            excel_file_path: "All Macros.xlsm".to_string(),
            excel_sheets: Vec::new(),
            clean_pictures: true,
            clean_cond_formatting: true,
            combine_include_source_col: true,
            pdf_output_folder: String::new(),

            logs: Vec::new(),
            show_log_panel: true,
        };

        app.log(LogLevel::Info, "⚡ Office Macro Suite initialized and ready.");
        if Path::new(&app.excel_file_path).exists() {
            app.load_excel_sheets();
        }
        app
    }
}

impl MacroSuiteApp {
    pub fn log(&mut self, level: LogLevel, text: &str) {
        let now = chrono_lite_time();
        self.logs.push(LogMessage {
            timestamp: now,
            level,
            text: text.to_string(),
        });
        if self.logs.len() > 300 {
            self.logs.remove(0);
        }
    }

    pub fn copy_to_clipboard(&self, text: &str) {
        if let Ok(mut clip) = Clipboard::new() {
            if clip.set_text(text).is_ok() {
                // copied successfully
            }
        }
    }

    fn load_excel_sheets(&mut self) {
        let path = Path::new(&self.excel_file_path);
        if !path.exists() {
            self.log(LogLevel::Error, &format!("File not found: {}", self.excel_file_path));
            return;
        }

        match excel_engine::read_sheet_info(path) {
            Ok(sheets) => {
                let active_count = sheets.iter().filter(|s| !s.is_hidden).count();
                let hidden_count = sheets.iter().filter(|s| s.is_hidden).count();
                self.log(
                    LogLevel::Success,
                    &format!(
                        "Loaded {} sheets from '{}' (Active: {}, Hidden: {})",
                        sheets.len(),
                        path.file_name().unwrap_or_default().to_string_lossy(),
                        active_count,
                        hidden_count
                    ),
                );
                self.excel_sheets = sheets;
            }
            Err(e) => {
                self.log(LogLevel::Error, &format!("Failed to read Excel sheets: {e}"));
            }
        }
    }
}

fn chrono_lite_time() -> String {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let total_secs = duration.as_secs();
    let hours = (total_secs / 3600) % 24;
    let mins = (total_secs / 60) % 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

impl eframe::App for MacroSuiteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let bg = theme::bg_main(self.dark_mode);
        let border = theme::border_color(self.dark_mode);

        // Top Header and Navigation Bar
        egui::TopBottomPanel::top("top_header_panel")
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(16, 12),
                fill: bg,
                stroke: Stroke::new(1.0, border),
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.render_header(ctx, ui);
            });

        // Bottom Activity Log Panel
        if self.show_log_panel {
            egui::TopBottomPanel::bottom("bottom_log_panel")
                .resizable(true)
                .default_height(140.0)
                .min_height(70.0)
                .max_height(300.0)
                .frame(egui::Frame {
                    inner_margin: egui::Margin::symmetric(14, 8),
                    fill: bg,
                    stroke: Stroke::new(1.0, border),
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    self.render_log_panel(ui);
                });
        }

        // Central Content Area
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(16),
                fill: bg,
                ..Default::default()
            })
            .show(ctx, |ui| {
                match self.current_tab {
                    ActiveTab::CreateFolders => self.render_create_folders_tab(ui),
                    ActiveTab::DeleteItems => self.render_delete_items_tab(ui),
                    ActiveTab::RenameFolders => self.render_rename_folders_tab(ui),
                    ActiveTab::RenameFiles => self.render_rename_files_tab(ui),
                    ActiveTab::ExcelRenameSheets => self.render_excel_rename_sheets_tab(ui),
                    ActiveTab::ExcelCleanWorkbook => self.render_excel_clean_tab(ui),
                    ActiveTab::ExcelCombineSheets => self.render_excel_combine_tab(ui),
                    ActiveTab::ExcelExportPdf => self.render_excel_pdf_tab(ui),
                }
            });
    }
}

// ---------------- UI Render Subsections ----------------
impl MacroSuiteApp {
    fn render_header(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let accent = theme::accent_color(self.dark_mode);
        let txt_muted = theme::text_muted(self.dark_mode);

        ui.horizontal(|ui| {
            // Logo / Title
            ui.label(
                RichText::new("⚡ MacroSuite Studio")
                    .color(accent)
                    .size(20.0)
                    .strong(),
            );
            ui.label(
                RichText::new("Native High-Performance Office Automation")
                    .color(txt_muted)
                    .size(12.0),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Dark / Light Mode Toggle Button
                let theme_btn_text = if self.dark_mode { "☀️ Light Mode" } else { "🌙 Dark Mode" };
                if ui.button(RichText::new(theme_btn_text).strong()).clicked() {
                    self.dark_mode = !self.dark_mode;
                    theme::apply_custom_theme(ctx, self.dark_mode);
                }

                ui.add_space(8.0);

                // Show/Hide Log Panel
                let log_text = if self.show_log_panel { "▼ Hide Log" } else { "▲ Show Log" };
                if ui.button(log_text).clicked() {
                    self.show_log_panel = !self.show_log_panel;
                }
            });
        });

        ui.add_space(8.0);

        // Category Switcher Pills
        ui.horizontal(|ui| {
            let cat_ff_active = self.current_category == TabCategory::FilesAndFolders;
            let cat_ex_active = self.current_category == TabCategory::ExcelSuite;

            if ui.selectable_label(cat_ff_active, RichText::new("📁 File & Directory Tools").strong().size(14.0)).clicked() {
                self.current_category = TabCategory::FilesAndFolders;
                self.current_tab = ActiveTab::RenameFiles;
            }

            ui.add_space(6.0);

            if ui.selectable_label(cat_ex_active, RichText::new("📊 Excel Automation Suite").strong().size(14.0)).clicked() {
                self.current_category = TabCategory::ExcelSuite;
                self.current_tab = ActiveTab::ExcelRenameSheets;
            }
        });

        ui.add_space(6.0);

        // Sub-Tab Switcher
        ui.horizontal(|ui| {
            match self.current_category {
                TabCategory::FilesAndFolders => {
                    self.tab_button(ui, "📄 Batch Rename Files", ActiveTab::RenameFiles);
                    self.tab_button(ui, "🗂️ Batch Rename Folders", ActiveTab::RenameFolders);
                    self.tab_button(ui, "➕ Create Folders", ActiveTab::CreateFolders);
                    self.tab_button(ui, "🗑️ Delete Items", ActiveTab::DeleteItems);
                }
                TabCategory::ExcelSuite => {
                    self.tab_button(ui, "📑 Rename Sheets", ActiveTab::ExcelRenameSheets);
                    self.tab_button(ui, "🧹 Clean Shapes & Formatting", ActiveTab::ExcelCleanWorkbook);
                    self.tab_button(ui, "🔗 Combine Sheets", ActiveTab::ExcelCombineSheets);
                    self.tab_button(ui, "📕 Export Active Sheets to PDF", ActiveTab::ExcelExportPdf);
                }
            }
        });
    }

    fn tab_button(&mut self, ui: &mut Ui, label: &str, tab: ActiveTab) {
        let is_selected = self.current_tab == tab;
        let rich = if is_selected {
            RichText::new(label).color(if self.dark_mode { Color32::WHITE } else { Color32::BLACK }).strong()
        } else {
            RichText::new(label).color(theme::text_muted(self.dark_mode))
        };
        if ui.selectable_label(is_selected, rich).clicked() {
            self.current_tab = tab;
        }
    }

    fn render_log_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📋 Activity & Status Log").strong().size(12.0).color(theme::text_primary(self.dark_mode)));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("Clear Log").clicked() {
                    self.logs.clear();
                }
                if ui.button("Copy All").clicked() {
                    let full_log = self.logs.iter()
                        .map(|l| format!("[{}] {:?}", l.timestamp, l.text))
                        .collect::<Vec<_>>()
                        .join("\n");
                    self.copy_to_clipboard(&full_log);
                }
            });
        });

        ui.add_space(4.0);

        theme::log_panel_frame(self.dark_mode).show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    if self.logs.is_empty() {
                        ui.label(RichText::new("No activity yet.").color(theme::text_dimmed(self.dark_mode)).italics());
                    } else {
                        for log in &self.logs {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&format!("[{}]", log.timestamp)).color(theme::text_dimmed(self.dark_mode)).size(11.0));
                                let color = theme::log_level_color(&log.level, self.dark_mode);
                                ui.label(RichText::new(&log.text).color(color).size(12.0));
                            });
                        }
                    }
                });
        });
    }
}

// ---------------- TAB 1: BATCH RENAME FILES ----------------
impl MacroSuiteApp {
    fn render_rename_files_tab(&mut self, ui: &mut Ui) {
        let accent = theme::accent_color(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Folder Path:").strong());
                ui.add(TextEdit::singleline(&mut self.fs_base_dir).desired_width(400.0));
                if ui.button("📁 Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.fs_base_dir = folder.to_string_lossy().to_string();
                    }
                }

                ui.add_space(12.0);
                ui.label("Filter:");
                ui.add(TextEdit::singleline(&mut self.fs_extension_filter).desired_width(60.0));

                if ui.button(RichText::new("🔄 Scan Files").strong().color(accent)).clicked() {
                    self.scan_files();
                }
            });
        });

        ui.add_space(8.0);
        self.render_transformer_bar(ui, false);
        ui.add_space(8.0);

        // Table
        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Loaded Files: {}", self.rename_file_items.len())).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("🚀 Apply Rename All").strong().color(COLOR_SUCCESS)).clicked() {
                        self.apply_rename_files();
                    }
                    if ui.button("📋 Paste New Names Column").clicked() {
                        self.paste_new_names_column(false);
                    }
                    if ui.button("📋 Copy All File Names").clicked() {
                        self.copy_all_file_names();
                    }
                    if ui.button("Reset Names").clicked() {
                        for item in &mut self.rename_file_items {
                            item.new_name = item.original_name.clone();
                            item.status = ItemStatus::Ready;
                        }
                    }
                });
            });

            ui.add_space(6.0);

            ScrollArea::vertical()
                .max_height(350.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("files_rename_grid")
                        .striped(true)
                        .min_col_width(80.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("#").strong());
                            ui.label(RichText::new("Current File Name").strong());
                            ui.label(RichText::new("New File Name (Editable)").strong());
                            ui.label(RichText::new("Status").strong());
                            ui.end_row();

                            let mut copied_name: Option<String> = None;

                            for (idx, item) in self.rename_file_items.iter_mut().enumerate() {
                                ui.label(format!("{}", idx + 1));

                                ui.horizontal(|ui| {
                                    if ui.small_button("📋").on_hover_text("Copy file name to clipboard").clicked() {
                                        copied_name = Some(item.original_name.clone());
                                    }
                                    ui.label(&item.original_name);
                                });

                                ui.add(TextEdit::singleline(&mut item.new_name).desired_width(280.0));
                                theme::render_status_badge(ui, &item.status);
                                ui.end_row();
                            }

                            if let Some(name) = copied_name {
                                self.copy_to_clipboard(&name);
                                self.log(LogLevel::Info, &format!("Copied file name: '{name}'"));
                            }
                        });
                });
        });
    }

    fn copy_all_file_names(&mut self) {
        let names: Vec<String> = self.rename_file_items.iter().map(|i| i.original_name.clone()).collect();
        if !names.is_empty() {
            let joined = names.join("\n");
            self.copy_to_clipboard(&joined);
            self.log(LogLevel::Success, &format!("Copied all {} file names to clipboard", names.len()));
        } else {
            self.log(LogLevel::Warning, "No files loaded to copy");
        }
    }

    fn scan_files(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        match fs_engine::scan_files(base_path, &self.fs_extension_filter) {
            Ok(files) => {
                let count = files.len();
                self.rename_file_items = files
                    .into_iter()
                    .map(|f| RenameItem {
                        original_name: f.clone(),
                        new_name: f,
                        status: ItemStatus::Ready,
                    })
                    .collect();
                self.log(LogLevel::Success, &format!("Scanned {count} files matching '{}'", self.fs_extension_filter));
            }
            Err(e) => {
                self.log(LogLevel::Error, &format!("Scan files failed: {e}"));
            }
        }
    }

    fn apply_rename_files(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        self.rename_file_items = fs_engine::execute_rename_batch(base_path, &self.rename_file_items);
        let success_count = self.rename_file_items.iter().filter(|i| matches!(i.status, ItemStatus::Success(_))).count();
        self.log(LogLevel::Success, &format!("File rename batch executed. Successfully renamed: {success_count}"));
    }
}

// ---------------- TAB 2: BATCH RENAME FOLDERS ----------------
impl MacroSuiteApp {
    fn render_rename_folders_tab(&mut self, ui: &mut Ui) {
        let accent = theme::accent_color(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Parent Folder:").strong());
                ui.add(TextEdit::singleline(&mut self.fs_base_dir).desired_width(450.0));
                if ui.button("📁 Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.fs_base_dir = folder.to_string_lossy().to_string();
                    }
                }

                if ui.button(RichText::new("🔄 Scan Subfolders").strong().color(accent)).clicked() {
                    self.scan_subfolders();
                }
            });
        });

        ui.add_space(8.0);
        self.render_transformer_bar(ui, true);
        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Loaded Folders: {}", self.rename_folder_items.len())).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("🚀 Apply Rename All").strong().color(COLOR_SUCCESS)).clicked() {
                        self.apply_rename_folders();
                    }
                    if ui.button("📋 Paste New Names Column").clicked() {
                        self.paste_new_names_column(true);
                    }
                    if ui.button("📋 Copy All Folder Names").clicked() {
                        self.copy_all_folder_names();
                    }
                    if ui.button("Reset Names").clicked() {
                        for item in &mut self.rename_folder_items {
                            item.new_name = item.original_name.clone();
                            item.status = ItemStatus::Ready;
                        }
                    }
                });
            });

            ui.add_space(6.0);

            ScrollArea::vertical()
                .max_height(350.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("folders_rename_grid")
                        .striped(true)
                        .min_col_width(80.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("#").strong());
                            ui.label(RichText::new("Current Folder Name").strong());
                            ui.label(RichText::new("New Folder Name (Editable)").strong());
                            ui.label(RichText::new("Status").strong());
                            ui.end_row();

                            let mut copied_name: Option<String> = None;

                            for (idx, item) in self.rename_folder_items.iter_mut().enumerate() {
                                ui.label(format!("{}", idx + 1));

                                ui.horizontal(|ui| {
                                    if ui.small_button("📋").on_hover_text("Copy folder name to clipboard").clicked() {
                                        copied_name = Some(item.original_name.clone());
                                    }
                                    ui.label(&item.original_name);
                                });

                                ui.add(TextEdit::singleline(&mut item.new_name).desired_width(280.0));
                                theme::render_status_badge(ui, &item.status);
                                ui.end_row();
                            }

                            if let Some(name) = copied_name {
                                self.copy_to_clipboard(&name);
                                self.log(LogLevel::Info, &format!("Copied folder name: '{name}'"));
                            }
                        });
                });
        });
    }

    fn copy_all_folder_names(&mut self) {
        let names: Vec<String> = self.rename_folder_items.iter().map(|i| i.original_name.clone()).collect();
        if !names.is_empty() {
            let joined = names.join("\n");
            self.copy_to_clipboard(&joined);
            self.log(LogLevel::Success, &format!("Copied all {} folder names to clipboard", names.len()));
        } else {
            self.log(LogLevel::Warning, "No folders loaded to copy");
        }
    }

    fn scan_subfolders(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        match fs_engine::scan_subfolders(base_path) {
            Ok(folders) => {
                let count = folders.len();
                self.rename_folder_items = folders
                    .into_iter()
                    .map(|f| RenameItem {
                        original_name: f.clone(),
                        new_name: f,
                        status: ItemStatus::Ready,
                    })
                    .collect();
                self.log(LogLevel::Success, &format!("Scanned {count} subfolders"));
            }
            Err(e) => {
                self.log(LogLevel::Error, &format!("Scan subfolders failed: {e}"));
            }
        }
    }

    fn apply_rename_folders(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        self.rename_folder_items = fs_engine::execute_rename_batch(base_path, &self.rename_folder_items);
        let success_count = self.rename_folder_items.iter().filter(|i| matches!(i.status, ItemStatus::Success(_))).count();
        self.log(LogLevel::Success, &format!("Folder rename batch executed. Successfully renamed: {success_count}"));
    }
}

// ---------------- TRANSFORMER TOOLS BAR (PREFIX, SUFFIX, REPLACE) ----------------
impl MacroSuiteApp {
    fn render_transformer_bar(&mut self, ui: &mut Ui, is_folders: bool) {
        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Batch Tools:").strong());

                ui.label("Prefix:");
                ui.add(TextEdit::singleline(&mut self.transform_prefix).desired_width(80.0));
                if ui.button("Add Prefix").clicked() {
                    let prefix = self.transform_prefix.clone();
                    self.apply_transform(is_folders, |name| format!("{prefix}{name}"));
                }

                ui.add_space(8.0);
                ui.label("Suffix:");
                ui.add(TextEdit::singleline(&mut self.transform_suffix).desired_width(80.0));
                if ui.button("Add Suffix").clicked() {
                    let suffix = self.transform_suffix.clone();
                    self.apply_transform(is_folders, |name| {
                        if !is_folders {
                            if let Some(pos) = name.rfind('.') {
                                let (stem, ext) = name.split_at(pos);
                                format!("{stem}{suffix}{ext}")
                            } else {
                                format!("{name}{suffix}")
                            }
                        } else {
                            format!("{name}{suffix}")
                        }
                    });
                }

                ui.add_space(8.0);
                ui.label("Find:");
                ui.add(TextEdit::singleline(&mut self.transform_find).desired_width(80.0));
                ui.label("Replace:");
                ui.add(TextEdit::singleline(&mut self.transform_replace).desired_width(80.0));
                if ui.button("Replace").clicked() {
                    let find = self.transform_find.clone();
                    let rep = self.transform_replace.clone();
                    if !find.is_empty() {
                        self.apply_transform(is_folders, |name| name.replace(&find, &rep));
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Quick Cases:");
                if ui.button("UPPERCASE").clicked() {
                    self.apply_transform(is_folders, |n| n.to_uppercase());
                }
                if ui.button("lowercase").clicked() {
                    self.apply_transform(is_folders, |n| n.to_lowercase());
                }
                if ui.button("Trim Spaces").clicked() {
                    self.apply_transform(is_folders, |n| n.trim().to_string());
                }
            });
        });
    }

    fn apply_transform<F>(&mut self, is_folders: bool, transform: F)
    where
        F: Fn(&str) -> String,
    {
        let items = if is_folders {
            &mut self.rename_folder_items
        } else {
            &mut self.rename_file_items
        };

        for item in items.iter_mut() {
            let transformed = transform(&item.new_name);
            if transformed != item.new_name {
                item.new_name = transformed;
                item.status = ItemStatus::Modified;
            }
        }
        self.log(LogLevel::Info, "Batch transformation applied to table preview");
    }

    fn paste_new_names_column(&mut self, is_folders: bool) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                let lines: Vec<&str> = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
                let items = if is_folders {
                    &mut self.rename_folder_items
                } else {
                    &mut self.rename_file_items
                };

                let mut updated = 0;
                for (idx, line) in lines.iter().enumerate() {
                    if idx < items.len() {
                        items[idx].new_name = line.to_string();
                        items[idx].status = ItemStatus::Modified;
                        updated += 1;
                    }
                }
                self.log(LogLevel::Success, &format!("Pasted {updated} new names from clipboard into table"));
            }
        }
    }
}

// ---------------- TAB 3: CREATE FOLDERS ----------------
impl MacroSuiteApp {
    fn render_create_folders_tab(&mut self, ui: &mut Ui) {
        let txt_muted = theme::text_muted(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Target Parent Folder:").strong());
                ui.add(TextEdit::singleline(&mut self.fs_base_dir).desired_width(450.0));
                if ui.button("📁 Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.fs_base_dir = folder.to_string_lossy().to_string();
                    }
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Folder Names to Create (One per line):").strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Clear").clicked() {
                        self.folder_names_input.clear();
                    }
                    if ui.button("📋 Copy Current List").clicked() {
                        if !self.folder_names_input.is_empty() {
                            self.copy_to_clipboard(&self.folder_names_input);
                            self.log(LogLevel::Info, "Copied folder names list to clipboard");
                        }
                    }
                    if ui.button("📋 Paste from Clipboard").clicked() {
                        if let Ok(mut clip) = Clipboard::new() {
                            if let Ok(text) = clip.get_text() {
                                self.folder_names_input = text;
                            }
                        }
                    }
                });
            });

            ui.add_space(6.0);
            ui.add(
                TextEdit::multiline(&mut self.folder_names_input)
                    .desired_rows(12)
                    .desired_width(f32::INFINITY),
            );

            let count = self.folder_names_input.lines().filter(|l| !l.trim().is_empty()).count();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Total Folders to Create: {count}")).color(txt_muted));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("🚀 Create Folders Now").strong().color(COLOR_SUCCESS)).clicked() {
                        self.execute_create_folders();
                    }
                });
            });
        });
    }

    fn execute_create_folders(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        let names: Vec<String> = self.folder_names_input
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if names.is_empty() {
            self.log(LogLevel::Warning, "No folder names provided to create");
            return;
        }

        let results = fs_engine::create_folders(base_path, &names);
        let success = results.iter().filter(|(_, r)| r.is_ok()).count();
        let failed = results.len() - success;

        self.log(LogLevel::Success, &format!("Create Folders completed. Created: {success}, Skipped/Failed: {failed}"));
    }
}

// ---------------- TAB 4: DELETE ITEMS ----------------
impl MacroSuiteApp {
    fn render_delete_items_tab(&mut self, ui: &mut Ui) {
        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Target Parent Folder:").strong());
                ui.add(TextEdit::singleline(&mut self.fs_base_dir).desired_width(450.0));
                if ui.button("📁 Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.fs_base_dir = folder.to_string_lossy().to_string();
                    }
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Items to Delete (One per line):").strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Clear").clicked() {
                        self.delete_names_input.clear();
                    }
                    if ui.button("📋 Paste from Clipboard").clicked() {
                        if let Ok(mut clip) = Clipboard::new() {
                            if let Ok(text) = clip.get_text() {
                                self.delete_names_input = text;
                            }
                        }
                    }
                });
            });

            ui.add_space(6.0);
            ui.add(
                TextEdit::multiline(&mut self.delete_names_input)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Delete Scope:");
                ui.radio_value(&mut self.delete_type, DeleteTargetType::FilesOnly, "Files Only");
                ui.radio_value(&mut self.delete_type, DeleteTargetType::FoldersOnly, "Folders Only");
                ui.radio_value(&mut self.delete_type, DeleteTargetType::Both, "Both (Files & Folders)");
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.delete_confirmed, RichText::new("I understand this permanently deletes matched files/folders").color(COLOR_WARNING));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let btn = egui::Button::new(RichText::new("🗑️ Delete Items").strong().color(COLOR_ERROR));
                    if ui.add_enabled(self.delete_confirmed, btn).clicked() {
                        self.execute_delete_items();
                    }
                });
            });
        });
    }

    fn execute_delete_items(&mut self) {
        let base_path = Path::new(&self.fs_base_dir);
        let names: Vec<String> = self.delete_names_input
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if names.is_empty() {
            self.log(LogLevel::Warning, "No items specified to delete");
            return;
        }

        let results = fs_engine::delete_items(base_path, &names, self.delete_type);
        let deleted = results.iter().filter(|(_, r)| r.is_ok()).count();
        let not_found = results.iter().filter(|(_, r)| r.is_err()).count();

        self.log(LogLevel::Success, &format!("Batch Delete completed. Deleted: {deleted}, Not Found / Errors: {not_found}"));
    }
}

// ---------------- TAB 5: EXCEL RENAME SHEETS ----------------
impl MacroSuiteApp {
    fn render_excel_rename_sheets_tab(&mut self, ui: &mut Ui) {
        let accent = theme::accent_color(self.dark_mode);
        let txt_muted = theme::text_muted(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Excel File (.xlsx / .xlsm):").strong());
                ui.add(TextEdit::singleline(&mut self.excel_file_path).desired_width(420.0));
                if ui.button("📄 Select File...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Excel Workbooks", &["xlsx", "xlsm"])
                        .pick_file()
                    {
                        self.excel_file_path = file.to_string_lossy().to_string();
                        self.load_excel_sheets();
                    }
                }

                if ui.button(RichText::new("🔄 Reload Sheets").strong().color(accent)).clicked() {
                    self.load_excel_sheets();
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                let active = self.excel_sheets.iter().filter(|s| !s.is_hidden).count();
                let hidden = self.excel_sheets.iter().filter(|s| s.is_hidden).count();
                ui.label(RichText::new(format!("Worksheet Count: {} (Active: {}, Hidden: {})", self.excel_sheets.len(), active, hidden)).strong());

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("🚀 Save Renamed Workbook").strong().color(COLOR_SUCCESS)).clicked() {
                        self.execute_rename_sheets();
                    }
                    if ui.button("📋 Paste New Sheet Names").clicked() {
                        self.paste_sheet_names();
                    }
                    if ui.button("📋 Copy Active Sheet Names").clicked() {
                        self.copy_sheet_names(true);
                    }
                    if ui.button("📋 Copy All Sheet Names").clicked() {
                        self.copy_sheet_names(false);
                    }
                    if ui.button("Reset Names").clicked() {
                        for s in &mut self.excel_sheets {
                            s.new_name = s.original_name.clone();
                            s.status = ItemStatus::Ready;
                        }
                    }
                });
            });

            ui.add_space(6.0);

            ScrollArea::vertical()
                .max_height(350.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("excel_sheets_grid")
                        .striped(true)
                        .min_col_width(70.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("#").strong());
                            ui.label(RichText::new("Visibility").strong());
                            ui.label(RichText::new("Current Sheet Name").strong());
                            ui.label(RichText::new("New Sheet Name (Editable)").strong());
                            ui.label(RichText::new("Rows").strong());
                            ui.label(RichText::new("Status").strong());
                            ui.end_row();

                            let mut copied_name: Option<String> = None;

                            for s in &mut self.excel_sheets {
                                ui.label(format!("{}", s.index));

                                if s.is_hidden {
                                    theme::render_badge(ui, "HIDDEN", Color32::from_rgb(55, 65, 81), Color32::from_rgb(209, 213, 219));
                                } else {
                                    theme::render_badge(ui, "ACTIVE", Color32::from_rgb(6, 78, 59), COLOR_SUCCESS);
                                }

                                ui.horizontal(|ui| {
                                    if ui.small_button("📋").on_hover_text("Copy sheet name to clipboard").clicked() {
                                        copied_name = Some(s.original_name.clone());
                                    }
                                    ui.label(&s.original_name);
                                });

                                if s.is_hidden {
                                    ui.label(RichText::new(&s.original_name).color(txt_muted));
                                } else {
                                    if ui.add(TextEdit::singleline(&mut s.new_name).desired_width(220.0)).changed() {
                                        s.status = ItemStatus::Modified;
                                    }
                                }

                                ui.label(format!("{}", s.row_count));
                                theme::render_status_badge(ui, &s.status);
                                ui.end_row();
                            }

                            if let Some(name) = copied_name {
                                self.copy_to_clipboard(&name);
                                self.log(LogLevel::Info, &format!("Copied sheet name: '{name}'"));
                            }
                        });
                });
        });
    }

    fn copy_sheet_names(&mut self, active_only: bool) {
        let names: Vec<String> = self.excel_sheets
            .iter()
            .filter(|s| !active_only || !s.is_hidden)
            .map(|s| s.original_name.clone())
            .collect();

        if !names.is_empty() {
            let joined = names.join("\n");
            self.copy_to_clipboard(&joined);
            let desc = if active_only { "active" } else { "all" };
            self.log(LogLevel::Success, &format!("Copied {} {desc} sheet names to clipboard", names.len()));
        } else {
            self.log(LogLevel::Warning, "No sheets loaded to copy");
        }
    }

    fn paste_sheet_names(&mut self) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                let lines: Vec<&str> = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
                let mut line_idx = 0;
                let mut updated = 0;

                for sheet in &mut self.excel_sheets {
                    if !sheet.is_hidden && line_idx < lines.len() {
                        sheet.new_name = lines[line_idx].to_string();
                        sheet.status = ItemStatus::Modified;
                        line_idx += 1;
                        updated += 1;
                    }
                }
                self.log(LogLevel::Success, &format!("Pasted {updated} new sheet names into visible sheets"));
            }
        }
    }

    fn execute_rename_sheets(&mut self) {
        let src = Path::new(&self.excel_file_path);
        if !src.exists() {
            self.log(LogLevel::Error, "Source Excel file does not exist");
            return;
        }

        let _parent = src.parent().unwrap_or(Path::new(""));
        let stem = src.file_stem().unwrap_or_default().to_string_lossy();
        let ext = src.extension().unwrap_or_default().to_string_lossy();

        if let Some(out_path) = rfd::FileDialog::new()
            .set_file_name(&format!("{stem}_renamed.{ext}"))
            .save_file()
        {
            let renames: Vec<(String, String)> = self.excel_sheets
                .iter()
                .filter(|s| !s.is_hidden && s.original_name != s.new_name)
                .map(|s| (s.original_name.clone(), s.new_name.clone()))
                .collect();

            if renames.is_empty() {
                self.log(LogLevel::Warning, "No sheet names were changed");
                return;
            }

            match excel_engine::rename_sheets(src, &out_path, &renames) {
                Ok(count) => {
                    self.log(LogLevel::Success, &format!("Successfully renamed {count} sheets! Saved to: {}", out_path.display()));
                }
                Err(e) => {
                    self.log(LogLevel::Error, &format!("Failed to rename sheets: {e}"));
                }
            }
        }
    }
}

// ---------------- TAB 6: EXCEL CLEAN WORKBOOK ----------------
impl MacroSuiteApp {
    fn render_excel_clean_tab(&mut self, ui: &mut Ui) {
        let txt_muted = theme::text_muted(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Target Excel File:").strong());
                ui.add(TextEdit::singleline(&mut self.excel_file_path).desired_width(420.0));
                if ui.button("📄 Select File...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Excel Workbooks", &["xlsx", "xlsm"])
                        .pick_file()
                    {
                        self.excel_file_path = file.to_string_lossy().to_string();
                    }
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.label(RichText::new("Cleaning Actions").strong().size(15.0));
            ui.label(RichText::new("Strips bloat, embedded drawings, shapes, and rules from the workbook without altering cell data.").color(txt_muted));

            ui.add_space(10.0);
            ui.checkbox(&mut self.clean_pictures, RichText::new("Delete All Pictures, Drawings & Shapes (Replaces VBA DeletePictures)").strong());
            ui.label(RichText::new("Removes all floating images, icons, and drawn shapes from all sheets.").color(txt_muted).size(11.0));

            ui.add_space(8.0);
            ui.checkbox(&mut self.clean_cond_formatting, RichText::new("Remove All Conditional Formatting (Replaces VBA removecond)").strong());
            ui.label(RichText::new("Clears all conditional formatting rules across all worksheets.").color(txt_muted).size(11.0));

            ui.add_space(14.0);
            if ui.button(RichText::new("🧹 Clean Workbook & Save As...").strong().color(COLOR_SUCCESS)).clicked() {
                self.execute_clean_workbook();
            }
        });
    }

    fn execute_clean_workbook(&mut self) {
        let src = Path::new(&self.excel_file_path);
        if !src.exists() {
            self.log(LogLevel::Error, "Source Excel file does not exist");
            return;
        }

        let stem = src.file_stem().unwrap_or_default().to_string_lossy();
        let ext = src.extension().unwrap_or_default().to_string_lossy();

        if let Some(out_path) = rfd::FileDialog::new()
            .set_file_name(&format!("{stem}_cleaned.{ext}"))
            .save_file()
        {
            let current_temp = out_path.clone();

            if self.clean_pictures {
                match excel_engine::clean_pictures_and_shapes(src, &current_temp) {
                    Ok(count) => {
                        self.log(LogLevel::Success, &format!("Stripped {count} drawing/picture elements from workbook"));
                    }
                    Err(e) => {
                        self.log(LogLevel::Error, &format!("Failed to clean shapes: {e}"));
                        return;
                    }
                }
            }

            if self.clean_cond_formatting {
                let temp_in = current_temp.clone();
                match excel_engine::clean_conditional_formatting(&temp_in, &out_path) {
                    Ok(count) => {
                        self.log(LogLevel::Success, &format!("Stripped {count} conditional formatting rules from workbook"));
                    }
                    Err(e) => {
                        self.log(LogLevel::Error, &format!("Failed to clean conditional formatting: {e}"));
                    }
                }
            }

            self.log(LogLevel::Success, &format!("Cleaned workbook saved to: {}", out_path.display()));
        }
    }
}

// ---------------- TAB 7: EXCEL COMBINE SHEETS ----------------
impl MacroSuiteApp {
    fn render_excel_combine_tab(&mut self, ui: &mut Ui) {
        let txt_muted = theme::text_muted(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Source Excel File:").strong());
                ui.add(TextEdit::singleline(&mut self.excel_file_path).desired_width(420.0));
                if ui.button("📄 Select File...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Excel Workbooks", &["xlsx", "xlsm"])
                        .pick_file()
                    {
                        self.excel_file_path = file.to_string_lossy().to_string();
                        self.load_excel_sheets();
                    }
                }
                if ui.button("🔄 Reload").clicked() {
                    self.load_excel_sheets();
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Select Visible Sheets to Combine into One").strong().size(14.0));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("🔗 Combine Selected Sheets").strong().color(COLOR_SUCCESS)).clicked() {
                        self.execute_combine_sheets();
                    }
                    if ui.button("📋 Copy Selected Sheet Names").clicked() {
                        self.copy_selected_sheet_names();
                    }
                    if ui.button("Select All Visible").clicked() {
                        for s in &mut self.excel_sheets {
                            if !s.is_hidden {
                                s.is_selected = true;
                            }
                        }
                    }
                    if ui.button("Deselect All").clicked() {
                        for s in &mut self.excel_sheets {
                            s.is_selected = false;
                        }
                    }
                });
            });

            ui.add_space(6.0);
            ui.checkbox(&mut self.combine_include_source_col, "Include 'Source Sheet' column in combined result");

            ui.add_space(6.0);

            ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("combine_sheets_grid")
                        .striped(true)
                        .min_col_width(80.0)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Select").strong());
                            ui.label(RichText::new("Sheet Name").strong());
                            ui.label(RichText::new("Rows").strong());
                            ui.label(RichText::new("Visibility").strong());
                            ui.end_row();

                            let mut copied_name: Option<String> = None;

                            for s in &mut self.excel_sheets {
                                if s.is_hidden {
                                    ui.label("-");
                                    ui.label(RichText::new(&s.original_name).color(txt_muted));
                                    ui.label(format!("{}", s.row_count));
                                    theme::render_badge(ui, "HIDDEN (Skipped)", Color32::from_rgb(55, 65, 81), Color32::from_rgb(209, 213, 219));
                                } else {
                                    ui.checkbox(&mut s.is_selected, "");
                                    ui.horizontal(|ui| {
                                        if ui.small_button("📋").on_hover_text("Copy sheet name to clipboard").clicked() {
                                            copied_name = Some(s.original_name.clone());
                                        }
                                        ui.label(RichText::new(&s.original_name).strong());
                                    });
                                    ui.label(format!("{}", s.row_count));
                                    theme::render_badge(ui, "ACTIVE", Color32::from_rgb(6, 78, 59), COLOR_SUCCESS);
                                }
                                ui.end_row();
                            }

                            if let Some(name) = copied_name {
                                self.copy_to_clipboard(&name);
                                self.log(LogLevel::Info, &format!("Copied sheet name: '{name}'"));
                            }
                        });
                });
        });
    }

    fn copy_selected_sheet_names(&mut self) {
        let names: Vec<String> = self.excel_sheets
            .iter()
            .filter(|s| !s.is_hidden && s.is_selected)
            .map(|s| s.original_name.clone())
            .collect();

        if !names.is_empty() {
            let joined = names.join("\n");
            self.copy_to_clipboard(&joined);
            self.log(LogLevel::Success, &format!("Copied {} selected sheet names to clipboard", names.len()));
        } else {
            self.log(LogLevel::Warning, "No visible sheets currently selected");
        }
    }

    fn execute_combine_sheets(&mut self) {
        let src = Path::new(&self.excel_file_path);
        if !src.exists() {
            self.log(LogLevel::Error, "Source Excel file does not exist");
            return;
        }

        let selected: Vec<String> = self.excel_sheets
            .iter()
            .filter(|s| !s.is_hidden && s.is_selected)
            .map(|s| s.original_name.clone())
            .collect();

        if selected.is_empty() {
            self.log(LogLevel::Warning, "No visible sheets selected to combine");
            return;
        }

        if let Some(out_path) = rfd::FileDialog::new()
            .set_file_name("Combined_Result.xlsx")
            .add_filter("Excel Workbook", &["xlsx"])
            .save_file()
        {
            match excel_engine::combine_sheets(src, &out_path, &selected, self.combine_include_source_col) {
                Ok(total_rows) => {
                    self.log(LogLevel::Success, &format!("Combined {} sheets (total {} rows) into: {}", selected.len(), total_rows, out_path.display()));
                }
                Err(e) => {
                    self.log(LogLevel::Error, &format!("Failed to combine sheets: {e}"));
                }
            }
        }
    }
}

// ---------------- TAB 8: EXCEL EXPORT TO PDF ----------------
impl MacroSuiteApp {
    fn render_excel_pdf_tab(&mut self, ui: &mut Ui) {
        let txt_muted = theme::text_muted(self.dark_mode);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Source Excel File:").strong());
                ui.add(TextEdit::singleline(&mut self.excel_file_path).desired_width(420.0));
                if ui.button("📄 Select File...").clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Excel Workbooks", &["xlsx", "xlsm"])
                        .pick_file()
                    {
                        self.excel_file_path = file.to_string_lossy().to_string();
                        self.load_excel_sheets();
                    }
                }
            });

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Output Folder for PDFs:").strong());
                ui.add(TextEdit::singleline(&mut self.pdf_output_folder).desired_width(420.0));
                if ui.button("📁 Browse Folder...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.pdf_output_folder = folder.to_string_lossy().to_string();
                    }
                }
            });
        });

        ui.add_space(8.0);

        theme::card_frame(self.dark_mode).show(ui, |ui| {
            ui.label(RichText::new("Export Active/Visible Sheets to Separate PDFs").strong().size(14.0));
            ui.label(
                RichText::new("Replaces VBA ExportAsPDF. Automatically skips all hidden sheets so only valid active worksheets are exported.")
                    .color(txt_muted),
            );

            ui.add_space(8.0);
            let active_sheets: Vec<&SheetItem> = self.excel_sheets.iter().filter(|s| !s.is_hidden).collect();
            ui.label(RichText::new(format!("Active Sheets to be Exported: {}", active_sheets.len())).strong().color(COLOR_TEAL));

            for s in active_sheets.iter().take(8) {
                ui.label(format!("  • {}.pdf", s.original_name));
            }
            if active_sheets.len() > 8 {
                ui.label(format!("  ... and {} more", active_sheets.len() - 8));
            }

            ui.add_space(14.0);
            if ui.button(RichText::new("📕 Export Active Sheets to PDF").strong().color(COLOR_SUCCESS)).clicked() {
                self.execute_export_pdf();
            }
        });
    }

    fn execute_export_pdf(&mut self) {
        let src = PathBuf::from(&self.excel_file_path);
        if !src.exists() {
            self.log(LogLevel::Error, "Source Excel file does not exist");
            return;
        }

        let out_dir = if self.pdf_output_folder.trim().is_empty() {
            src.parent().unwrap_or(Path::new("")).to_path_buf()
        } else {
            PathBuf::from(&self.pdf_output_folder)
        };

        self.log(LogLevel::Info, "Exporting active sheets to PDF using background Excel COM automation...");

        match excel_engine::export_visible_sheets_to_pdf(&src, &out_dir) {
            Ok(exported) => {
                self.log(
                    LogLevel::Success,
                    &format!("Export complete! Generated {} PDF files in: {}", exported.len(), out_dir.display()),
                );
            }
            Err(e) => {
                self.log(LogLevel::Error, &format!("PDF Export error: {e}"));
            }
        }
    }
}
