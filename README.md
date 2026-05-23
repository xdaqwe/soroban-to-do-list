# 📝 OnChain Todos — Stellar Soroban dApp

Aplikasi To-Do List berbasis blockchain yang dibangun di atas **Stellar Network** menggunakan **Soroban Smart Contract**. Setiap task yang ditambahkan, diselesaikan, atau dihapus tersimpan langsung on-chain secara transparan dan tidak bisa dimanipulasi.

> 🎓 Dibuat sebagai tugas akhir program **Stellar RiseIn Bootcamp**

---

## 🌟 Fitur

- ✅ **Tambah task** — tersimpan permanen di blockchain
- ✅ **Toggle selesai** — tandai task selesai/belum selesai
- ✅ **Hapus task** — remove task dari on-chain storage
- ✅ **Connect wallet** — integrasi dengan Freighter Wallet
- ✅ **Stats real-time** — total, selesai, dan pending tasks
- ✅ **Per-user storage** — setiap wallet punya data task sendiri

---

## 🛠️ Tech Stack

| Layer | Teknologi |
|---|---|
| Smart Contract | Rust + Soroban SDK |
| Network | Stellar Testnet |
| Frontend | HTML, CSS, Vanilla JS |
| Wallet | Freighter Browser Extension |
| SDK | stellar-base (CDN) |
| Deploy Contract | Soroban Studio |

---

## 📁 Struktur Project

```
todo-dapp/
├── contract/
│   └── src/
│       └── lib.rs        # Smart contract Soroban (Rust)
│   └── Cargo.toml        # Rust dependencies
├── frontend/
│   └── index.html        # Frontend lengkap (HTML + CSS + JS)
└── README.md
```

---

## ⚙️ Cara Deploy & Menjalankan

### Prasyarat

Sebelum mulai, pastikan sudah punya:

- [ ] Browser (Chrome / Brave / Firefox)
- [ ] Ekstensi **Freighter Wallet** → [freighter.app](https://freighter.app)
- [ ] Akun Freighter sudah di-set ke **Testnet**
- [ ] XLM Testnet (gratis via Friendbot)

---

### Step 1 — Deploy Smart Contract di Soroban Studio

1. Buka [soroban.studio](https://soroban.studio)
2. Buat project baru
3. Copy-paste isi file `contract/src/lib.rs` ke editor
4. Klik **Build** — tunggu sampai berhasil compile
5. Klik **Deploy to Testnet**
6. Setelah deploy berhasil, **copy Contract ID** yang muncul

> ⚠️ Simpan Contract ID ini, nanti dipakai di frontend!

---

### Step 2 — Dapatkan XLM Testnet Gratis

Buka link berikut di browser (ganti `<PUBLIC_KEY>` dengan public key Freighter kamu):

```
https://friendbot.stellar.org?addr=<PUBLIC_KEY>
```

Atau bisa lewat [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test)

---

### Step 3 — Jalankan Frontend

1. Buka file `frontend/index.html` langsung di browser (double-click)
2. Klik tombol **Connect Freighter** → approve di popup Freighter
3. Paste **Contract ID** dari Step 1 ke kolom yang tersedia
4. Mulai tambah task! 🎉

---

## 🔗 Contract yang Sudah Di-deploy

| Info | Detail |
|---|---|
| Network | Stellar Testnet |
| Contract ID | `CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX` |
| Deploy oleh | `GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX` |

> Ganti tabel di atas dengan Contract ID dan Public Key kamu setelah deploy!

---

## 📐 Arsitektur Aplikasi

```
User (Browser)
     │
     │  Connect Wallet
     ▼
Freighter Wallet ──── sign transaksi
     │
     │  XDR Transaction
     ▼
Soroban RPC (Testnet)
     │
     │  invoke contract
     ▼
Smart Contract (on-chain)
  ┌─────────────────────┐
  │  add_task()         │
  │  get_tasks()        │
  │  complete_task()    │
  │  delete_task()      │
  └─────────────────────┘
     │
     │  result
     ▼
Frontend Update UI
```

---

## 📜 Fungsi Smart Contract

```rust
// Tambah task baru
add_task(env, user: Address, title: String) -> u32

// Ambil semua tasks milik user
get_tasks(env, user: Address) -> Vec<Task>

// Toggle task selesai / belum
complete_task(env, user: Address, task_id: u32) -> bool

// Hapus task
delete_task(env, user: Address, task_id: u32) -> bool

// Hapus semua tasks
clear_tasks(env, user: Address)
```

### Struktur Data Task

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
```

---

## 🧪 Menjalankan Tests

Jika ingin menjalankan unit test contract secara lokal:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Tambah target WASM
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli

# Masuk ke folder contract
cd contract

# Jalankan tests
cargo test
```

---

## 🖼️ Tampilan Aplikasi

```
┌─────────────────────────────────────┐
│  ● STELLAR TESTNET                  │
│                                     │
│  OnChain                            │
│  Todos                              │
│  // POWERED BY SOROBAN              │
│                                     │
│  ● G1234...5678    [✅ Connected]   │
│  CONTRACT ID: [CXXX...XXX_______]   │
│  ─────────────────────────────────  │
│  [Tulis task baru...      ] [+ Add] │
│                                     │
│  — TASKS ON-CHAIN                   │
│  ┌───────────────────────────────┐  │
│  │ #01  Belajar Soroban      ○ 🗑│  │
│  │ #02  Deploy contract     ✅ 🗑│  │
│  │ #03  Submit tugas         ○ 🗑│  │
│  └───────────────────────────────┘  │
│                                     │
│  [  3  ]   [  1  ]   [  2  ]       │
│   Total     Selesai   Pending       │
└─────────────────────────────────────┘
```

---

## 🔍 Cara Verifikasi Transaksi

Setiap aksi di app menghasilkan transaksi on-chain yang bisa dicek di:

**Stellar Explorer (Testnet):**
```
https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>
```

---

## ❓ Troubleshooting

**Freighter tidak terdeteksi**
→ Pastikan ekstensi Freighter sudah terinstall dan aktif di browser

**"Contract ID belum diisi"**
→ Paste Contract ID dari Soroban Studio ke kolom yang tersedia

**Transaksi gagal / timeout**
→ Pastikan saldo XLM Testnet cukup (minimal 1 XLM), gunakan Friendbot jika habis

**Task tidak muncul setelah ditambah**
→ Tunggu beberapa detik, transaksi on-chain butuh waktu konfirmasi

---

## 👤 Author

**[Nama Kamu]**
- GitHub: [@username](https://github.com/username)
- Stellar Public Key: `GXXXXXXX...`

---

## 📄 Lisensi

MIT License — bebas digunakan untuk keperluan belajar.

---

> Dibuat dengan ❤️ selama program **Stellar RiseIn Bootcamp**