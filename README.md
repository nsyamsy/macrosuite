<div align="center">

# ⚡ MacroSuite Studio

**High-Performance Native Office & Batch File Automation**  
*Re-engineering Excel VBA Macros into a Blazing Fast, Standalone Rust Desktop App.*

[![Rust](https://img.shields.io/badge/Language-Rust_1.80+-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui%20%2F%20eframe-blue.svg?style=for-the-badge)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078D6.svg?style=for-the-badge&logo=windows)](https://github.com/nsyamsy/macrosuite)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

[**English**](#-english) | [**Bahasa Indonesia**](#-bahasa-indonesia)

</div>

---

## 🌐 English

### 📖 Overview

**MacroSuite Studio** is a modern, lightweight, and super-fast desktop utility built in Rust that modernizes repetitive Excel VBA macro routines. It decouples vital file and spreadsheet operations from bloated, single-threaded Excel workbooks into an autonomous, safe, and visually appealing GUI application.

### 🌟 Key Highlights

* 🚀 **Blazing Fast Native Performance:** Written in 100% Rust. Scans and processes thousands of files, directories, and rows in milliseconds without UI freezes.
* 📦 **Single Portable Executable:** Runs out-of-the-box (~8.7 MB) with zero external runtime or framework requirements.
* 🛡️ **Safety First & Live Previews:** Review original vs. new names in an interactive data grid before committing batch renames or deletes.
* 🧠 **Smart Active vs. Hidden Sheet Handling:** Intelligently inspects OpenXML structures to protect hidden sheets while operating strictly on visible/active sheets.
* 🎨 **Bilingual UI with Dark & Light Themes:** Toggle seamlessly between dark slate and clean light modes with a single click.
* 📋 **One-Click Clipboard Helpers:** Dedicated bulk and row-level copy buttons for sheet names, folder paths, and filenames.

---

### ⚡ Feature Comparison: VBA vs. MacroSuite

| Feature | Legacy VBA Macro (`.xlsm`) | MacroSuite Studio (Rust) |
| :--- | :--- | :--- |
| **Execution Speed** | Slow & single-threaded (Excel freezes) | **Instant, multithreaded native code** |
| **Application Dependency** | Requires Microsoft Office installed & open | **Standalone binary (runs independently)** |
| **UI Experience** | Basic Excel cells & modal message boxes | **Modern, reactive GUI (egui / eframe)** |
| **Error Handling** | Hidden silently via `On Error Resume Next` | **Detailed live timestamped activity log** |
| **Preview Before Action**| None (direct execution) | **Full grid preview with conflict checks** |
| **Hidden Sheets Safety**| Prone to accidental overwrites | **Automatic detection & protection** |

---

### 🛠️ Core Modules

#### 1. 📁 File & Directory Suite
* **Batch Rename Files:** Scan directories with wildcard extension filters (`*.pdf`, `*.xlsx`, etc.), apply prefixes, suffixes, text search-and-replace, and case transformations (UPPERCASE, lowercase, Trim).
* **Batch Rename Folders:** Scan nested subdirectories instantly and apply structured rename rules.
* **Bulk Folder Creator:** Create hundreds of categorized folders simultaneously from multiline lists or copied Excel tables.
* **Batch Deleter:** Delete targeted files or folders safely with confirmation guards and scope selectors (*Files Only*, *Folders Only*, or *Both*).

#### 2. 📊 Excel Automation Suite
* **Intelligent Sheet Renamer:** Loads sheet names from `.xlsx` / `.xlsm` workbooks, marks them as `ACTIVE` or `HIDDEN`, and updates OpenXML sheet definitions safely without corrupting formulas.
* **Shapes & Bloat Cleaner:** Strips floating drawings, pictures, charts, and embedded shape objects across all sheets (replaces VBA `DeletePictures`).
* **Conditional Formatting Cleaner:** Removes all conditional formatting rules across sheets to minimize workbook size and speed up calculation engines (replaces VBA `removecond`).
* **Combine Worksheets (Consolidator):** Merges data from selected visible worksheets into one centralized master sheet with an optional `Source Sheet` tracking column.
* **Export Active Sheets to PDF:** Automates headless PDF exports for active worksheets, automatically skipping hidden sheets.

---

### 🖥️ Interface Preview

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ ⚡ MacroSuite Studio — Native Office Automation                 [☀️ Light] [▲ Hide Log]  │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ [ 📁 File & Directory Tools ]   [ 📊 Excel Automation Suite ]                           │
│                                                                                         │
│  [📄 Batch Rename Files] [🗂️ Batch Rename Folders] [➕ Create Folders] [🗑️ Delete Items] │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ Folder Path: [ D:\Project\Application\macros          ] [📁 Browse...] Filter: [*.*   ] │
│ Batch Tools: [Prefix: _______] [Suffix: _______] [Find: _______] [Replace: _______]     │
│ Quick Cases: [ UPPERCASE ] [ lowercase ] [ Trim Spaces ]                                │
│                                                                                         │
│ ┌───┬──────────────────────────────┬──────────────────────────────┬───────────────────┐ │
│ │ # │ Current File Name            │ New File Name                │ Status            │ │
│ ├───┼──────────────────────────────┼──────────────────────────────┼───────────────────┤ │
│ │ 1 │ 📋 Invoice_001.pdf           │ 2026_Q1_Invoice_001.pdf      │ [Modified]        │ │
│ │ 2 │ 📋 Report_Draft.xlsx         │ 2026_Q1_Report_Draft.xlsx    │ [Ready]           │ │
│ └───┴──────────────────────────────┴──────────────────────────────┴───────────────────┘ │
│ [ 🔄 Scan Files ] [ 📋 Copy All Names ] [ 📋 Paste Column ] [ 🚀 Apply Rename All ]      │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│ 📋 Activity & Status Log                                              [Copy All] [Clear]│
│ [16:32:15] [INFO] Loaded 10 sheets from 'All Macros.xlsm' (Active: 9, Hidden: 1)        │
│ [16:32:16] [SUCCESS] Copied 9 active sheet names to clipboard                           │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### 🔧 Build & Run from Source

#### Prerequisites
* [Rust & Cargo](https://rustup.rs/) (version 1.80 or newer recommended)
* Windows 10/11 (MSVC toolchain)

```bash
# Clone the repository
git clone https://github.com/nsyamsy/macrosuite.git
cd macrosuite

# Build and run with release optimizations
cargo run --release

# The compiled portable binary will be available at:
# target/release/macro_suite.exe
```

---

<br/>

## 🇮🇩 Bahasa Indonesia

### 📖 Ringkasan

**MacroSuite Studio** adalah aplikasi desktop modern, ringan, dan berkecepatan tinggi yang dibangun menggunakan bahasa pemrograman Rust untuk memodernisasi tugas-tugas macro Excel VBA. Aplikasi ini memisahkan operasi manajemen file dan pemrosesan lembar kerja Excel dari workbook yang lambat menjadi aplikasi GUI mandiri (*standalone*), aman, dan nyaman digunakan.

### 🌟 Keunggulan Utama

* 🚀 **Performa Tinggi & Tanpa Macet:** 100% ditulis dalam Rust. Memindai ribuan berkas, folder, dan baris data hanya dalam hitungan milidetik tanpa membuat jendela aplikasi *freeze*.
* 📦 **Satu File Executable Portabel:** Siap dijalankan langsung (~8.7 MB) tanpa perlu instalasi runtime atau dependensi tambahan.
* 🛡️ **Aman dengan Preview Interaktif:** Tinjau nama lama vs. nama baru dalam tabel sebelum mengeksekusi operasi penggantian nama atau penghapusan berkas.
* 🧠 **Proteksi Sheet Aktif vs. Tersembunyi:** Secara cerdas membaca struktur OpenXML untuk memproteksi sheet yang di-*hide* dan hanya memproses sheet yang aktif/visible.
* 🎨 **Tema Gelap & Terang (Dark/Light Mode):** Beralih tema antarmuka secara instan melalui tombol toggle di sudut kanan atas.
* 📋 **Kemudahan Salin Nama (Clipboard):** Dilengkapi tombol salin nama massal (*bulk copy*) maupun tombol salin per baris untuk mempermudah transfer data kembali ke Excel.

---

### 🛠️ Fitur & Modul Tersedia

#### 1. 📁 Manajemen File & Direktori
* **Batch Rename Files:** Memindai file dalam folder dengan filter ekstensi (`*.pdf`, `*.xlsx`, dll.), menambahkan awalan (*prefix*), akhiran (*suffix*), cari & ganti teks (*find & replace*), serta format huruf besar/kecil.
* **Batch Rename Folders:** Memindai subfolder secara instan dan mengubah nama folder massal dengan validasi konflik nama.
* **Create Folders (Pembuat Folder Massal):** Membuat ratusan folder sekaligus berdasarkan daftar teks (bisa langsung di-*paste* dari kolom Excel).
* **Delete Items (Penghapus Massal):** Menghapus daftar file atau folder secara aman dengan verifikasi persetujuan (*safety confirmation*).

#### 2. 📊 Otomasi Excel
* **Rename Sheets:** Membaca nama lembar kerja dari file `.xlsx` / `.xlsm`, mendeteksi status `ACTIVE` atau `HIDDEN`, dan memperbarui nama sheet tanpa merusak formula.
* **Clean Shapes & Bloat:** Menghapus seluruh gambar, shape, dan objek drawing di workbook untuk meringankan ukuran file (pengganti macro `DeletePictures`).
* **Remove Conditional Formatting:** Membersihkan seluruh aturan pemformatan bersyarat untuk mempercepat kalkulasi workbook (pengganti macro `removecond`).
* **Combine Worksheets (Konsolidasi):** Menggabungkan data dari beberapa sheet aktif ke dalam satu sheet master (`Combined Data`).
* **Export Active Sheets to PDF:** Mengekspor masing-masing sheet yang aktif menjadi file PDF terpisah, dengan otomatis mengabaikan sheet yang tersembunyi.

---

### 🚀 Cara Kompilasi & Menjalankan

```bash
# Clone repository
git clone https://github.com/nsyamsy/macrosuite.git
cd macrosuite

# Jalankan dalam mode release
cargo run --release

# File executable hasil kompilasi akan berada di:
# target/release/macro_suite.exe
```

---

## 📄 License

Proyek ini dilisensikan di bawah lisensi [MIT License](LICENSE).
Bebas digunakan, dimodifikasi, dan didistribusikan untuk kebutuhan personal maupun profesional.
