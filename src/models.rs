#[derive(Clone, Debug, PartialEq)]
pub enum ActiveTab {
    CreateFolders,
    DeleteItems,
    RenameFolders,
    RenameFiles,
    ExcelRenameSheets,
    ExcelCleanWorkbook,
    ExcelCombineSheets,
    ExcelExportPdf,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabCategory {
    FilesAndFolders,
    ExcelSuite,
}

#[derive(Clone, Debug)]
pub struct RenameItem {
    pub original_name: String,
    pub new_name: String,
    pub status: ItemStatus,
}

#[derive(Clone, Debug)]
pub struct SheetItem {
    pub index: usize,
    pub original_name: String,
    pub new_name: String,
    pub is_hidden: bool,
    pub is_selected: bool,
    pub row_count: usize,
    pub status: ItemStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemStatus {
    Ready,
    Modified,
    Success(String),
    Warning(String),
    Error(String),
}

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub timestamp: String,
    pub level: LogLevel,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeleteTargetType {
    FilesOnly,
    FoldersOnly,
    Both,
}
