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

Get Retro VM running with these straightforward steps. This setup enables you to use the embedded Kali Linux VM, QEMU support, and unlock the app's full cybersecurity simulation capabilities.


System Requirements
----------------------------
- Operating System: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- RAM: 8 GB minimum (16 GB recommended for virtual machine operations)
- Storage: 5 GB free space
- Processor: Multi-core CPU (4+ cores recommended with virtualization enabled)
- Graphics: DirectX 11 or OpenGL 3.3+ compatible GPU

Installation Steps
----------------------------

1. DOWNLOAD RETRO VM
   - Get the latest release from the official repository or release archive.
   - Extract/unzip to your desired installation directory.

2. SETUP ASSETS FOLDER
   Ensure the following directory structure inside your project folder:

   /assets
   ├── kali-linux.iso           → Kali Linux bootable installer ISO
   ├── kali-linux.qcow2         → QEMU virtual disk image (optional or created via CLI)
   └── qemu/
       ├── qemu-system-x86_64   → QEMU executable
       └── supporting binaries  → Necessary DLLs or shared libs

3. DOWNLOAD KALI LINUX ISO
   - Visit: https://www.kali.org/get-kali/
   - Download: “Kali Linux 64-bit Installer (ISO)”
   - Rename and place the file in: /assets/kali-linux.iso

4. INSTALL QEMU
   - For Windows: https://qemu.weilnetz.de/w64/
   - For macOS: Install via Homebrew
       $ brew install qemu
   - For Ubuntu/Linux:
       $ sudo apt update && sudo apt install qemu qemu-kvm

   - Place the QEMU binary in: /assets/qemu/qemu-system-x86_64

5. CREATE QCOW2 VIRTUAL DISK (if not included)
   - Open a terminal and run:
       $ qemu-img create -f qcow2 kali-linux.qcow2 30G
   - This creates a 30GB virtual disk.
   - Move this file into the /assets/ directory.

6. LAUNCH THE APP
   - Run the main executable (retro_vm.exe or equivalent).
   - Enter Ghost Mode and press SHIFT + D to boot Kali Linux using QEMU.

First-Time Configuration
----------------------------

- The built-in setup wizard will:
   * Detect the Kali ISO and QEMU binary
   * Prompt for virtual disk path (qcow2)
   * Allow resource allocation (RAM, CPU cores)
   * Test VM boot sequence
   * Enable customization of themes and security settings

----------------------------
Verify Setup
----------------------------
- Confirm these components are working:
   * Normal and Ghost modes
   * Virtual machine boots correctly
   * Notepad, Encryption, and Hacker Terminal accessible
   * Hidden terminal activated via Ctrl+H → Ctrl+Alt+G

----------------------------
Support & Contribution
----------------------------
- For bugs: Submit a GitHub issue with logs and steps to reproduce
- For feature suggestions: Open a discussion or enhancement proposal
- For development: Follow code style, test thoroughly, and document your changes

Enjoy Retro VM – your gamified, retro-infused ethical hacking simulator!

---

## 🎯 Conclusion

Retro VM stands as a testament to the seamless integration of nostalgia and modern utility. It offers users a uniquely comprehensive virtual machine management experience, enveloped in a captivating retro aesthetic. From casual gaming to rigorous penetration testing, this application provides a cohesive journey through different computing eras, all while adhering to the stringent security and performance standards of contemporary software.

The intelligently layered authentication system, subtly integrated hidden features, and an expansive application suite create an engaging experience that rewards exploration and curiosity. Whether you're reliving the golden age of pixel art gaming or performing serious security research, Retro VM provides the quintessential tools and an immersive atmosphere to make your computing endeavors both productive and profoundly enjoyable.

**Key Highlights:**
* **Comprehensive Application Suite**: Five fully-featured, meticulously designed applications.
* **Dual-Mode Architecture**: Intuitive Normal and the enigmatic Ghost modes, with hidden access to powerful functionalities.
* **Professional VM Management**: Full-fledged Kali Linux virtualization for serious work.
* **Authentic Retro Design**: True pixel-perfect 8-bit aesthetics that transport you back in time.
* **Cultural Integration**: Meaningful references and subtle Easter eggs that enrich the user experience.
* **Security Focus**: Robust security and privacy protections ensuring a safe and isolated environment.

**The Secret Path**: Always remember, the true power of Retro VM resides within its hidden depths. Begin your journey with the welcoming **Normal Mode**, uncover the mysterious **Ghost Mode**, and ultimately unlock the professional capabilities of the hidden **Hacker Mode** using the **Shift + D** hotkey. Each layer you reveal will unveil new possibilities and deeper, more profound functionality.

*"In the intersection of past and present, where pixels meet possibility, Retro VM creates a bridge between the computing dreams of yesterday and the technological realities of today."*

---

**Happy Computing! 🚀👻🔓**
