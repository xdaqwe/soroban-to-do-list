# 📝 OnChain Todos — Stellar Soroban dApp

A blockchain-based To-Do List application built on the **Stellar Network** using **Soroban Smart Contracts**. Every task you add, complete, or delete is stored directly on-chain — transparent and tamper-proof.

> 🎓 Built as a final project for the **Stellar RiseIn Bootcamp**

---

## 🌟 Features

- ✅ **Add tasks** — stored permanently on the blockchain
- ✅ **Toggle complete** — mark tasks as done or undone
- ✅ **Delete tasks** — remove tasks from on-chain storage
- ✅ **Wallet connect** — integrated with Freighter Wallet
- ✅ **Real-time stats** — total, completed, and pending tasks
- ✅ **Per-user storage** — each wallet has its own task data

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Smart Contract | Rust + Soroban SDK |
| Network | Stellar Testnet |
| Frontend | HTML, CSS, Vanilla JS |
| Wallet | Freighter Browser Extension |
| SDK | stellar-base (CDN) |
| Contract Deploy | Soroban Studio |

---

## 📁 Project Structure

```
todo-dapp/
├── contract/
│   └── src/
│       └── lib.rs        # Soroban smart contract (Rust)
│   └── Cargo.toml        # Rust dependencies
├── frontend/
│   └── index.html        # Full frontend (HTML + CSS + JS)
└── README.md
```

---

## ⚙️ Setup & Running the App

### Prerequisites

Make sure you have the following ready:

- [ ] A browser (Chrome / Brave / Firefox)
- [ ] **Freighter Wallet** extension → [freighter.app](https://freighter.app)
- [ ] Freighter account set to **Testnet**
- [ ] Testnet XLM (free via Friendbot)

---

### Step 1 — Deploy Smart Contract on Soroban Studio

1. Open [soroban.studio](https://soroban.studio)
2. Create a new project
3. Copy and paste the contents of `contract/src/lib.rs` into the editor
4. Click **Build** — wait until it compiles successfully
5. Click **Deploy to Testnet**
6. Once deployed, **copy the Contract ID** that appears

> ⚠️ Save this Contract ID — you'll need it for the frontend!

---

### Step 2 — Get Free Testnet XLM

Open the following URL in your browser (replace `<PUBLIC_KEY>` with your Freighter public key):

```
https://friendbot.stellar.org?addr=<PUBLIC_KEY>
```

Or use the [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test)

---

### Step 3 — Run the Frontend

1. Open `frontend/index.html` directly in your browser (double-click the file)
2. Click **Connect Freighter** → approve in the Freighter popup
3. Paste the **Contract ID** from Step 1 into the input field
4. Start adding tasks! 🎉

---

## 🔗 Deployed Contract

| Info | Detail |
|---|---|
| Network | Stellar Testnet |
| Contract ID | `CBUCTRWPHD3QSJIOODLZ4XXUD6ZSZJVYYFSQQWGGYMQ6GPWWC737BDRP` |
| Deployed by | `XDAQWE` |

> Replace the table above with your actual Contract ID and Public Key after deploying!

---

## 📐 Application Architecture

```
User (Browser)
     │
     │  Connect Wallet
     ▼
Freighter Wallet ──── sign transaction
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
Frontend updates UI
```

---

## 📜 Smart Contract Functions

```rust
// Add a new task
add_task(env, user: Address, title: String) -> u32

// Get all tasks for a user
get_tasks(env, user: Address) -> Vec<Task>

// Toggle task complete / incomplete
complete_task(env, user: Address, task_id: u32) -> bool

// Delete a task
delete_task(env, user: Address, task_id: u32) -> bool

// Clear all tasks
clear_tasks(env, user: Address)
```

### Task Data Structure

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
```

---

## 🧪 Running Tests Locally

If you want to run the contract unit tests locally:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli

# Go to the contract folder
cd contract

# Run tests
cargo test
```

---

## 🖼️ App Preview

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
│  [Write a new task...     ] [+ Add] │
│                                     │
│  — TASKS ON-CHAIN                   │
│  ┌───────────────────────────────┐  │
│  │ #01  Learn Soroban        ○ 🗑│  │
│  │ #02  Deploy contract     ✅ 🗑│  │
│  │ #03  Submit assignment    ○ 🗑│  │
│  └───────────────────────────────┘  │
│                                     │
│  [  3  ]   [  1  ]   [  2  ]       │
│   Total    Done      Pending        │
└─────────────────────────────────────┘
```

---

## 🔍 Verifying Transactions

Every action in the app creates an on-chain transaction. You can verify it at:

**Stellar Explorer (Testnet):**
```
https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>
```

---

## ❓ Troubleshooting

**Freighter not detected**
→ Make sure the Freighter extension is installed and enabled in your browser

**"Contract ID not filled"**
→ Paste the Contract ID from Soroban Studio into the input field

**Transaction failed / timeout**
→ Make sure your Testnet XLM balance is sufficient (at least 1 XLM); use Friendbot to top up

**Tasks not showing after adding**
→ Wait a few seconds — on-chain transactions need time to confirm

---

## 👤 Author

**[XDAQWE]**
- GitHub: [@xdaqwe](https://github.com/xdaqwe)
- Stellar Public Key: `GXXXXXXX...`

---

## 📄 License

MIT License — free to use for learning purposes.

---

> Built with ❤️ during the **Stellar RiseIn Bootcamp**