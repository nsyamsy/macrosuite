# ⚡ MacroSuite Studio (Rust + GUI)

Aplikasi desktop native berperforma tinggi yang dibangun menggunakan **Rust** dan antarmuka modern **egui/eframe** sebagai pengganti rangkaian macro VBA di `All Macros.xlsm`.

Aplikasi ini mandiri (standalone), portabel, dan langsung dapat dijalankan melalui file executable tunggal: `MacroSuite.exe`.

---

## 🚀 Cara Menjalankan

1. **Jalankan langsung executable:**
   Double-click file **`MacroSuite.exe`** yang ada di direktori ini.
   
2. **Atau jalankan melalui terminal / Cargo:**
   ```bash
   cargo run --release
   ```

---

## 🛠️ Fitur & Modul yang Tersedia

### 1. 📁 File & Directory Tools
* **Batch Rename Files:**
  * Memindai seluruh berkas di folder dengan filter ekstensi opsional (misal: `*.*`, `*.pdf`, `*.xlsx`).
  * Preview tabel interaktif: *Original Name* vs *New Name*.
  * Batch tools terintegrasi: *Prefix*, *Suffix*, *Find & Replace*, *UPPERCASE / lowercase / Trim*.
  * Dukungan paste 1 kolom nama baru langsung dari Excel / Clipboard.
  * Eksekusi aman dengan indikator status per file.
* **Batch Rename Folders:**
  * Memindai subfolder dalam direktori induk secara instan.
  * Penataan nama baru dengan batch transformers.
  * Validasi nama folder sebelum proses rename dilakukan.
* **Create Folders:**
  * Membuat banyak folder sekaligus berdasarkan daftar teks (satu nama per baris).
  * Tombol *Paste from Clipboard* untuk paste dari kolom Excel.
* **Delete Items:**
  * Menghapus file atau folder massal berdasarkan daftar nama.
  * Opsi lingkup: *Files Only*, *Folders Only*, atau *Both*.
  * Proteksi checkbox konfirmasi untuk mencegah kesalahan hapus yang tidak disengaja.

---

### 2. 📊 Excel Automation Suite
* **Rename Sheets:**
  * Otomatis membedakan sheet **ACTIVE (Visible)** dan **HIDDEN**.
  * Sheet tersembunyi (hidden) tidak akan disentuh/diubah sembarangan sesuai preferensi kerja.
  * Kolom nama baru dapat diedit langsung atau di-paste dari clipboard.
  * Disimpan ke file baru secara aman tanpa merusak workbook asli.
* **Clean Shapes & Formatting:**
  * **Delete Pictures / Shapes:** Menghapus seluruh drawing, gambar, dan shape floating dari seluruh sheet (rekonstruksi dari macro `DeletePictures`).
  * **Remove Conditional Formatting:** Menghapus seluruh aturan format bersyarat dari semua sheet (rekonstruksi dari macro `removecond`).
* **Combine Sheets (Merge):**
  * Memilih dan menggabungkan beberapa sheet aktif menjadi satu sheet konsolidasi (`Combined Data`).
  * Opsi menyertakan kolom `Source Sheet` untuk penanda asal data.
* **Export Active Sheets to PDF:**
  * Mengonversi sheet yang aktif menjadi file PDF terpisah ke folder output.
  * Otomatis mengabaikan sheet yang di-hide (hanya sheet yang berstatus *Visible* yang diekspor).

---

### 📋 Fitur Pendukung UI (Quality of Life)
* **Pilihan Light Mode & Dark Mode:** Tersedia tombol toggle di pojok kanan atas (`☀️ Light Mode` / `🌙 Dark Mode`) untuk mengganti tema antarmuka secara instan sesuai kenyamanan visual Anda.
* **Fitur Copy Nama (Sheet, Folder, File):**
  * **Copy Massal:** Tombol `📋 Copy Active Sheet Names`, `📋 Copy All Sheet Names`, `📋 Copy All Folder Names`, dan `📋 Copy All File Names` untuk menyalin seluruh daftar nama (dipisah baris baru) sehingga siap di-paste langsung ke Excel atau Notepad.
  * **Copy Per Baris:** Ikon `📋` kecil di samping setiap nama sheet, folder, dan file pada tabel preview untuk menyalin nama spesifik dalam satu klik.
* **Activity & Status Log Panel:** Panel log aktivitas di bagian bawah dengan timestamp, level status (Info, Success, Warning, Error), serta tombol *Copy All* dan *Clear*.
* **Native Dialogs:** Terintegrasi langsung dengan file dialog bawaan Windows untuk pemilihan file dan folder.
