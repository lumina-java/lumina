# ⚡ Welcome to Your Lumina Backend Application: lumina_clean

Selamat! Anda telah berhasil menginisialisasi project baru dengan **Lumina Framework** — Web & RESTful API framework modern berbasis Rust (Axum 0.7 & Tokio Async) yang memadukan kecepatan komputasi sistem Rust dengan elegansi dan ergonomi sintaks **Laravel** dan **Lumen**.

---

## 🚀 Menjalankan Server Lokal (Quick Start)

Secara default, project ini menggunakan database **SQLite** (file lokal). Anda dapat langsung menjalankan server tanpa perlu setup database external:

```bash
cargo run
```

Akses di browser atau API client Anda:
- 🌐 **Web Dashboard (Laravel Style)**: [http://127.0.0.1:8000](http://127.0.0.1:8000)
- ⚡ **Lumen-Style Headless JSON Spec**:
  ```bash
  curl -H "Accept: application/json" http://127.0.0.1:8000/
  ```
- 🩺 **Health Check Endpoint**: [http://127.0.0.1:8000/health](http://127.0.0.1:8000/health)
- 🛣️ **REST API Base Path**: [http://127.0.0.1:8000/api](http://127.0.0.1:8000/api)

---

## 🗄️ Beralih ke PostgreSQL atau MySQL

Lumina mendukung multi-driver database secara native. Cukup sesuaikan variabel di `.env`:

### Contoh PostgreSQL:
```env
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=nama_db_kamu
DB_USERNAME=postgres
DB_PASSWORD=secret
DATABASE_URL=postgres://postgres:secret@127.0.0.1:5432/nama_db_kamu
```

### Jalankan Migrasi Database:
```bash
lumina migrate
```

---

## 📁 Struktur Folder Proyek

```text
├── Cargo.toml                  # Konfigurasi dependensi (Axum, Tokio, SQLx, Lumina)
├── .env                        # Variabel environment (Database, App Secret, Port)
├── src/
│   ├── main.rs                 # Application entrypoint & HTTP server bootstrap
│   ├── lib.rs                  # Module exports (app, routes) untuk modular testing
│   ├── routes/
│   │   ├── web.rs              # Route frontend / HTML views
│   │   └── api.rs              # Route RESTful API (prefix /api)
│   └── app/
│       ├── controllers/        # Request handling & responses
│       ├── models/             # Active Record models (#[derive(LuminaModel)])
│       └── services/           # Business logic layer
├── resources/views/            # Blade template views (.blade.rs)
├── database/migrations/        # Migrasi skema database SQL
└── tests/                      # Automated integration tests
```

---

## 🦀 Selamat Berkarya dengan Lumina Framework!
Dokumentasi lengkap dan update framework: [github.com/lumina-java/framework](https://github.com/lumina-java/framework)
