<h1 align="center">🚀 GraBbit</h1>
<p align="center">
  <b>The missing link in your local device ecosystem.</b><br>
  <i>Clipboard & File Sharing — Secure. Seamless. Local-first.</i>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/UI%20Library-Iced-00c2cb?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/Secure-AES--GCM-brightgreen?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/Cross--Platform-Windows%20|%20Linux%20|%20macOS-blue?style=for-the-badge"/>
</p>

---

## 🌐 What is GraBbit?

**GraBbit** turns your local devices into a powerful, private ecosystem — think **Apple Handoff** or **Samsung Quick Share**, but for **any platform**, **without the cloud**, and **with full encryption**.

📱💻 Imagine copying a text on your laptop and instantly seeing it on your phone or desktop, over the same Wi-Fi.  
📂 Want to send an image or file? Just drop it — GraBbit handles it securely and instantly.

> ⚡ It's a personal ecosystem — when you're in **desperate need of device synergy**, GraBbit becomes the **soul** of your workflow.

---

## 🌟 Key Features

### 🎭 Dual Modes
- **Host Mode**  
  Central device that receives clipboard and files from others. Set IP/port, manage retention.

- **Node Mode**  
  Connect to a host and push clipboard entries or files with a single key press.

### 🔄 Instant Clipboard Sharing
- Works like magic: Press `Ctrl + C` on one device, open GraBbit on another, and it's there.  
- Tracks and syncs:
  - Text 📝  
  - Files 📁  
  - Images 🖼️

- Rich metadata (timestamp, device name, OS, username)  
- Smart deduplication for clean history

### 🔒 Encrypted Transfers
- Every transfer is secured with **AES-GCM** — protecting your data in transit  
- Zero cloud, zero leakage — 100% private, 100% local

### 📂 File & Image Support
- Transfer small files and images with ease  
- Optional compression for images  
- Cross-device, cross-platform compatibility

### 🧠 Configurable Data Retention
- Keep data:
  - Until manual deletion
  - For 1 day
  - For 1 week
  - For 1 month

### 🖥️ Minimal Dashboard Interface
- View all received clipboard entries and files  
- See which devices are connected  
- Monitor live transfer activity

### ⚙️ Customizable Settings
- Light & Dark Mode 🌗  
- Image compression slider  
- Switch between Host and Node anytime

---

## 📦 Real Use Cases

| Scenario | Solution |
|----------|----------|
| Copy code on laptop → paste on desktop | ✅ GraBbit syncs it instantly |
| Share a quick image across devices without cloud | ✅ Encrypted local transfer |
| Keep work and personal device clipboards in sync | ✅ One host, multiple nodes |
| Offline LAN-only environments | ✅ 100% local, no internet needed |

> You don’t need to be in the Apple or Samsung ecosystem. **Build your own with GraBbit.**

---

## 🛠️ Under the Hood

| Component     | Description |
|---------------|-------------|
| **Language**  | Rust 🦀 |
| **UI**        | [Iced](https://github.com/iced-rs/iced) — Elm-style architecture |
| **Storage**   | JSON files with full metadata and timestamping |
| **Networking**| Local HTTP APIs over Wi-Fi |
| **Encryption**| AES-GCM authenticated encryption |

---

## 🚀 Getting Started

```bash
# Clone the repo
git clone https://github.com/yourusername/grabbit.git
cd grabbit

# Build (requires Rust toolchain)
cargo build --release

# Run
cargo run
