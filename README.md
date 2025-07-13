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

## 📝 Installation & Setup

Get Retro VM up and running quickly with these straightforward steps.

### System Requirements
* **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+).
* **RAM**: Minimum **8GB** (16GB recommended for VM operations).
* **Storage**: **5GB** free space for complete installation.
* **Processor**: Multi-core processor (4+ cores recommended).
* **Graphics**: DirectX 11 compatible graphics card.

### Installation Steps
1.  **Download**: Obtain the latest release from the official repository.
2.  **Extract**: Unzip the application files to your desired location.
3.  **Assets**: Ensure all asset files are correctly placed within the specified directory structure.
4.  **Dependencies**: Install any required system dependencies (e.g., QEMU).
5.  **Launch**: Run the main executable to start the Retro VM application.

### Configuration
* **First Run**: An initial setup wizard will guide you through system configuration.
* **VM Setup**: Download and configure the Kali Linux ISO as prompted.
* **Preferences**: Customize settings for optimal performance and user experience.
* **Testing**: Verify all features are working correctly after setup.

---

## 🤝 Contributing & Support

We welcome contributions and provide support for our community.

### Bug Reports
* **Issue Tracking**: Please use GitHub issues for all bug reports.
* **Detailed Reports**: Include system information and clear reproduction steps.
* **Screenshots**: Provide visual evidence of issues when applicable.
* **Log Files**: Include relevant log files and error messages.

### Feature Requests
* **Enhancement Proposals**: Suggest new features and improvements.
* **Community Voting**: Community input will help prioritize feature development.
* **Development Timeline**: We aim to provide realistic timelines for feature implementation.
* **Testing**: We encourage community testing of new features.

### Development Guidelines
* **Code Style**: Adhere to consistent coding standards and practices.
* **Documentation**: Comprehensive documentation for all features is encouraged.
* **Testing**: Thorough testing of all functionality is essential.
* **Review Process**: All code changes will undergo a review and approval process.

---

## 📚 Documentation & Resources

Access comprehensive guides and community support.

### User Guides
* **Getting Started**: A complete beginner's guide to using the application.
* **Advanced Features**: Detailed documentation of advanced functionality.
* **Troubleshooting**: Solutions for common issues and problems.
* **FAQ**: Frequently asked questions and answers.

### Technical Documentation
* **API Reference**: Complete API documentation for developers.
* **Architecture Guide**: Insights into the technical architecture and design decisions.
* **Performance Guide**: Optimization tips and best practices.
* **Security Guide**: Security considerations and best practices.

### Community Resources
* **Forums**: Community discussion and support forums.
* **Wiki**: A community-maintained documentation and guides.
* **Video Tutorials**: Step-by-step video guides and tutorials.
* **Blog**: Development updates and announcements.

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
