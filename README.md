# Retro VM+

**Retro VM+** is a cross-platform Rust and Python-powered cybersecurity simulator that blends gamification with real-world ethical hacking tools. Designed to teach, test, and entertain, the app includes hidden features, retro-style terminals, virtualization, ML-based network analysis, and embedded games – all geared toward providing practical cybersecurity exposure.

---

## 🧩 Core Features

### 🎮 Dual Boot Modes

- **Normal Mode:**
  - UI mimics a booted system.
  - Prompts password entry. If incorrect (not "Hola Amigos!"):
    - Spawns fake virus attack by opening multiple notepad processes.
    - Simulates malware behavior.
    - Must type `stop` to terminate the attack.

- **Ghost Mode:**
  - Shows ghost animation.
  - Press `Shift + D` to launch **Kali Linux VM** using **QEMU** and a virtual disk (`.iso` and `.qcow2`).
  - Provides a virtual penetration testing environment.

---

### 💻 Hacker Terminal (Stealth Activation)

- Achieved by:
  - Playing **Vedic Math Game**
  - Scoring above **2048**
  - Pressing `Ctrl + H` → Black screen mode
  - Then press `Ctrl + Alt + G` → Opens terminal interface

#### Supported Terminal Commands:

| Command     | Description |
|-------------|-------------|
| `phishgen`  | Generates phishing page via `warp`, tunnels through Ngrok, logs captured credentials. |
| `footscan`  | Simulates OSINT-based digital footprint scanning. |
| `netscan`   | Scans Wi-Fi interfaces and browser extensions; uses ML to identify threats. |
| `vault`     | SHA-256 password hash vault; verifies hashes without storing plaintext. |


🔐 Terminal Modules & Commands
Command	Description
phishgen	Phishing Website Generator: Generates a fake login page using HTML templates in Rust. Tunnels it online using Ngrok, logs credentials in real-time.
netscan	AI-Powered Network Scanner: Uses Python + trained ML model (via scikit-learn + xgboost) and NLP techniques with nltk to scan for:

Risky Wi-Fi names (SSID fingerprinting)

Suspicious browser extensions (parsed using BeautifulSoup)

Detects potential keylogging patterns and trackers via page content scraping |
| footscan | Digital Footprint Scanner: Uses OSINT and metadata extraction to simulate how attackers might gather passive data about you online. |
| vault | Password Hash Vault: Accepts input, hashes it with SHA-256 using Rust’s crypto lib. Decryption only simulated via input match. |
| ngoltek | Combines Ngrok with simulated attacker endpoint logic. Shows users how malicious tunnels can be used for C2 (Command and Control). |

🧠 Under the Hood (Tech Used)
✅ Rust for the terminal engine (tui), fake site host (warp), and secure modules

✅ Python for ML/NLP backend:

nltk for tokenizing Wi-Fi SSIDs / extension names

BeautifulSoup for browser extension parsing

sentence-transformers to semantically understand extension descriptions

Pre-trained model classifies risk levels (e.g., “Tracker”, “Suspicious Ad Injector”)

---

### 📝 Tools & Embedded Learning

- **SHA-256 Notepad**
  - Write text, encrypt using SHA-256.
  - Simulates secure password storage.
  - Decryption (verification) only possible within this notepad interface.

- **Chess Game**
  - Functional gameplay.
  - Hidden **"Save Game"** button opens embedded cybersecurity lessons:
    - Linux commands
    - OWASP Top 10
    - CTF steps and payload tips

- **Floppy Disk Game**
  - Encrypts an image with secret text.
  - Only upon winning is it saved.
  - If user fails, image + message is deleted, simulating data loss.

- **1996-Themed Chatbot**
  - Powered by **Ollama LLM**
  - Offers insights on ethical hacking, cybersecurity practices, and fun facts.

---

## 🔧 Tech Stack

### Rust
- `tokio` – async runtime
- `warp` – lightweight HTTP server
- `tui` – terminal-based UI
- `sha2` – hashing algorithm
- `std::process` – QEMU execution, fake virus, notepad spawning
- `serde`, `reqwest` – API requests and configs (e.g., Ngrok tunnel info)

### Python
Used for ML-based scanning and chatbot backend:
- `Flask`, `beautifulsoup4`, `requests`, `sentence-transformers`
- `scikit-learn`, `xgboost`, `nltk`, `transformers`, `torch`, `matplotlib`

---

## 📦 Requirements

### ✅ Prerequisites

| Tool       | Required For             |
|------------|--------------------------|
| **Rust**   | Core app, QEMU launcher  |
| **Python 3.10+** | ML, chatbot, API calls |
| **QEMU**   | Booting Kali Linux VM    |
| **Ngrok**  | Phishing site tunneling  |
| **Kali ISO** | Virtual hacking system  |

### Folder Structure:
```
project/
├── assets/
│   ├── kali-linux.iso
│   └── qemu/
│       └── qemu-system-x86_64(.exe)
├── src/
│   ├── main.rs
│   ├── server.rs
│   ├── ngrok.rs
│   ├── tui.rs
│   └── ...
├── requirements.txt
├── Cargo.toml
└── README.txt
```

---

## 🔐 Real-World Cybersecurity Concepts Simulated

| Concept              | Implemented In                                  |
|----------------------|--------------------------------------------------|
| Virtualization       | Kali Linux boot via QEMU                        |
| Red Team Tactics     | Phishing site, fake malware                    |
| Hashing              | SHA-256 Notepad encryption                     |
| Data Loss Simulation | Floppy Game (delete-on-fail)                   |
| OSINT Footprint      | `footscan` module                              |
| Risk Detection       | Network & extension scanner with ML            |
| LLM & Security Chat  | Ollama chatbot                                 |

---


📝 Installation & Setup Guide
==============================

Retro VM is a gamified cybersecurity simulation app built with Rust and Python. Follow the steps below to install, configure, and run it on your system.

----------------------------
System Requirements
----------------------------

- OS: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- RAM: 8 GB minimum (16 GB recommended)
- Storage: 5 GB+ free
- CPU: Multi-core (4+ cores, virtualization supported)
- GPU: DirectX 11 or OpenGL 3.3+ compatible
- Tools Required:
  * Git
  * Python 3.8+
  * Rust (via rustup)
  * QEMU
  * Ngrok

Step-by-Step Installation
----------------------------

1. CLONE THE PROJECT FROM GITHUB
   $ git clone https://github.com/Cyberpunk-San/osdc.git
   $ cd osdc

2. INSTALL PYTHON DEPENDENCIES
   $ pip install -r requirements.txt

3. INSTALL RUST (IF NOT INSTALLED)
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   $ rustup update
   $ cargo --version

4. DOWNLOAD KALI LINUX ISO
   - Go to https://www.kali.org/get-kali/
   - Download "Kali Linux 64-bit Installer (ISO)"
   - Place it in: /assets/kali-linux.iso

5. SETUP QEMU
   - Download QEMU for your OS:
     * Windows: https://qemu.weilnetz.de/w64/
     * macOS: `brew install qemu`
     * Linux: `sudo apt install qemu qemu-kvm`
   - Create a directory: /assets/qemu/
   - Place the QEMU binary in that folder:
     Example: /assets/qemu/qemu-system-x86_64

6. CREATE QCOW2 VIRTUAL DISK
   $ qemu-img create -f qcow2 assets/kali-linux.qcow2 30G

7. BUILD THE PROJECT
   $ cargo build --release

8. RUN RETRO VM
   $ cargo run

Optional Notes
----------------------------

- To run in release mode (faster):
   $ cargo run --release

- To rebuild:
   $ cargo clean
   $ cargo build

----------------------------
Folder Structure Overview
----------------------------

  /assets/
  ├── kali-linux.iso           (Bootable Kali image)
  ├── kali-linux.qcow2         (QEMU virtual disk)
  └── qemu/
      └── qemu-system-x86_64   (QEMU binary)

🔐 Hidden Features & Easter Eggs (Retro VM) Summary
==================================================

Retro VM is packed with hidden cybersecurity tools and gamified challenges. Below is a complete list of hidden features, how to unlock them, and what each one teaches.

------------------------------------------------
1. 🔥 Ghost Mode (From Main Menu)
------------------------------------------------
- Description:
  Switches interface to stealth mode with animated ghost visuals.
- Activation:
  Select “Ghost Mode” on the home screen.
- Purpose:
  Enables access to advanced tools including the VM launcher.

------------------------------------------------
2. 🖥️ Kali Linux Virtual Machine
------------------------------------------------
- Description:
  Launches Kali Linux using QEMU in a sandboxed virtual environment.
- Activation:
  Inside Ghost Mode → Press `Shift + D`
- Requirements:
  `/assets/kali-linux.iso` and a `kali-linux.qcow2` file.
- Purpose:
  Safe and isolated environment to practice ethical hacking techniques.

------------------------------------------------
3. 🧠 Fake Virus Simulator
------------------------------------------------
- Description:
  A security prank that mimics a malware outbreak.
- Trigger:
  Enter any password **except** `hola amigos!` in Normal Mode login.
- Effect:
  Spawns multiple Notepad windows simulating a virus.
- Exit:
  Type `stop` on the main screen to terminate.

------------------------------------------------
4. 📓 SHA-256 Notepad
------------------------------------------------
- Description:
  Secure notepad that hashes user text using SHA-256.
- Features:
  - Hashing and internal decryption.
  - Decryption only allowed inside the app.
- Purpose:
  Teaches irreversible hashing and secure data entry.

------------------------------------------------
5. 🤖 1996-Style Chatbot (Ollama)
------------------------------------------------
- Description:
  Retro terminal-based chatbot built with Ollama + Transformers.
- Theme:
  Classic terminal UI mimicking late-90s shell interactions.
- Use Case:
  Acts as a retro-styled AI mentor for cybersecurity and math logic.

------------------------------------------------
6. 🧮 Vedic Math Game → Unlock Hacker Terminal
------------------------------------------------
- Description:
  Math challenge with a reward system.
- Activation Path:
  - Score ≥ 2048 → Press `Ctrl + H` → screen darkens.
  - Then press `Ctrl + Alt + G` → Hacker Terminal opens.

------------------------------------------------
7. 🖥️ Hacker Terminal (Rust-powered CLI)
------------------------------------------------
- Description:
  Advanced command-line tools for simulated ethical hacking.
- Key Commands:
  - `phishgen`   → Generates phishing sites + tunnels via Ngrok.
  - `netscan`    → Network scanner using NLP (NLTK) and XGBoost model.
  - `footscan`   → Digital footprint + metadata scanner.
  - `vault`      → Password manager with SHA-256 encryption.
  - `ngoltek`    → Simulates attacker tunnel behavior using Ngrok logic.
- Purpose:
  Learn red teaming, scanning, phishing, and footprinting.

------------------------------------------------
8. ♟️ Cyber Chess (with Hacking Module)
------------------------------------------------
- Description:
  Classic chess interface with an integrated cybersecurity knowledge drop.
- Hidden Feature:
  Clicking the "Save" button opens an interactive learning module on ethical hacking topics.
- Purpose:
  Gamifies security awareness while testing logic skills.

------------------------------------------------
9. 💾 Floppy Disk Game (Steganography Challenge)
------------------------------------------------
- Description:
  A mini-game teaching steganography through play.
- Objective:
  Navigate and reach the goal to securely embed hidden text into an image.
- If You Win:
  → Image is encrypted using steganography (text is embedded inside).
- If You Lose:
  → The image is saved in raw binary format and the secret text is NOT recoverable.
- Purpose:
  Demonstrates real-world risks of improper encryption and hidden data manipulation.

------------------------------------------------
🎉 Bonus Easter Eggs
------------------------------------------------
- Secret terminal phrases like:
  - `sudo hacktheplanet`
  - `initiate legacy mode`
  - `unlock darklayer`
- May trigger animations, ASCII art, system sounds, or rare messages.

------------------------------------------------
💡 Tip:
Every layer of Retro VM is designed like a real-world capture-the-flag (CTF) environment. Exploration = education.
----------------------------
Support & Contribution
----------------------------
- For bugs: Submit a GitHub issue with logs and steps to reproduce
- For feature suggestions: Open a discussion or enhancement proposal
- For development: Follow code style, test thoroughly, and document your changes

Enjoy Retro VM – your gamified, retro-infused ethical hacking simulator!

