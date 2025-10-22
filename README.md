<h1 align="center">🚀 GraBbit</h1>
<p align="center">
  <b>A cross-device clipboard and file sharing app for secure, seamless, local transfers.</b><br>
  <i>Minimal. Encrypted. Local-first.</i>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/UI%20Library-Iced-00c2cb?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/Secure-AES--GCM-brightgreen?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/Cross--Platform-Windows%20|%20Linux%20|%20macOS-blue?style=for-the-badge"/>
</p>

---

## 🌟 Overview

**GraBbit** is a privacy-focused, cross-device clipboard and file sharing application that works across your personal devices on the same network. Designed for **security**, **usability**, and **efficiency**, GraBbit allows you to effortlessly sync clipboard content and transfer small files between devices — all without the cloud.

🛡️ **Privacy-first** — all data is encrypted  
🖥️ **Dual-mode operation** — run as host or node  
🌐 **Local-only** — no external servers, fully offline

---

## 🔧 Key Features

### 🎭 Dual Modes
- **Host Mode**:  
  Acts as a central receiver, listening on a local IP and port. Configurable data retention.
- **Node Mode**:  
  Connects to the host to send clipboard entries and files.

### 🔄 Clipboard Synchronization
- Real-time clipboard monitoring  
- Stores data with rich metadata:  
  `timestamp`, `device name`, `username`, `OS`  
- Smart deduplication of entries

### ⏳ Data Retention Control
- Store data:
  - Until manual deletion
  - 1 day
  - 1 week
  - 1 month

### 🔐 Secure Transfer
- **AES-GCM** encryption for all data in transit  
- Guarantees both **confidentiality** and **integrity**

### 🖥️ Dashboard Interface
- Minimal and responsive UI  
- View received data with metadata  
- Live monitoring:  
  - Active devices  
  - Transfer activity  
  - Clipboard item count

### ⚙️ Application Settings
- Light/Dark theme toggle 🌗  
- Image compression level setting  
- Host/Node mode switch  

---

## 🛠️ Technical Highlights

| Component     | Description |
|---------------|-------------|
| **Language**  | Rust 🦀 |
| **UI Framework** | [Iced](https://github.com/iced-rs/iced) — Elm-style architecture |
| **Encryption** | AES-GCM (Authenticated Encryption) |
| **Networking** | Local HTTP APIs over LAN |
| **Storage**    | JSON-based metadata and clipboard history |

---

## 📦 Use Case Scenarios

- ✅ Instantly sync clipboard content between laptop and desktop  
- ✅ Secure file transfers across your local network  
- ✅ Temporary data sharing without relying on the internet  
- ✅ Perfect for teams in secure environments or privacy-focused individuals

---

## 💡 Why GraBbit?

✔️ **No cloud** — your data never leaves your network  
✔️ **Encryption-first** — strong, modern security by default  
✔️ **Beautifully minimal UI** — built for focus and speed  
✔️ **Host-Node model** — allows flexible device arrangements  
✔️ **Cross-platform** — works on Windows, Linux, and macOS

> ⚡ “Grab it. Share it. Forget it.” — That's the GraBbit way.

---

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/yourusername/grabbit.git
cd grabbit

# Build (requires Rust toolchain)
cargo build --release

# Run
cargo run
