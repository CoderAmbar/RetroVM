██████╗ ███████╗████████╗██████╗  ██████╗     ██╗   ██╗███╗   ███╗
██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔═══██╗    ██║   ██║████╗ ████║
██████╔╝█████╗     ██║   ██████╔╝██║   ██║    ██║   ██║██╔████╔██║
██╔══██╗██╔══╝     ██║   ██╔══██╗██║   ██║    ██║   ██║██║╚██╔╝██║
██║  ██║███████╗   ██║   ██║  ██║╚██████╔╝    ╚██████╔╝██║ ╚═╝ ██║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝      ╚═════╝ ╚═╝     ╚═╝

> **Version 0.9.0 — “Ghost in the Shellcode”**
> *Crafted by a nostalgic engineer with two decades of awkward late‑night commits under his belt.*

![Retro VM Banner](assets/readme/banner.png)

---

## 📚 Table of Contents

1. [About the Project](#about-the-project)
2. [Feature Matrix](#feature-matrix)
3. [Installation](#installation)
4. [Quick Start](#quick-start)
5. [Architecture Primer](#architecture-primer)
6. [Hacker Mode Toolchain](#hacker-mode-toolchain)

   * [Digital Footprint Scanner](#digital-footprint-scanner)
   * [Network Scanner](#network-scanner)
   * [Fake Site Generator](#fake-site-generator)
7. [Security & Compliance](#security--compliance)
8. [Performance Benchmarks](#performance-benchmarks)
9. [Contribution Guide](#contribution-guide)
10. [Roadmap](#roadmap)
11. [License](#license)
12. [Acknowledgements](#acknowledgements)

---

## 🧐 About the Project

Retro VM fuses the pixel‑perfect charm of 1995 with the virtualisation muscle of 2025.
Think **Win95 UI**, **Matrix‑green console**, and **QEMU‑powered payloads**—all wrapped in a security‑hardened sandbox.  If *The X‑Files* opened a co‑working space with *Neuromancer*, this would be the resident kiosk.

---

## 🚀 Feature Matrix

| Layer       | Module                                             | Highlights                                                               |
| ----------- | -------------------------------------------------- | ------------------------------------------------------------------------ |
| Home Screen | Animated night sky, parallax clouds, crescent moon | 60 FPS canvas renderer, low‑power idle mode                              |
| Normal Mode | 2048, Chess, Floppy Maze, Notepad, AI ChatBot ’96  | Typewriter intro, Indian hip‑hop Easter egg                              |
| Ghost Mode  | Screen‑saver, idle detection, cute ghost sprite    | Hidden hot‑key `Shift + D` unlocks Hacker Mode                           |
| Hacker Mode | Kali Linux VM, QEMU backend, resource sliders      | **NEW:** Digital Footprint Scanner, Network Scanner, Fake Site Generator |

> **Keyboard Safety Net** — Tap **Tab** anywhere to resurface the main‑menu; **Esc** politely nopes out of the current app.

---

## 🛠️ Installation

> **Prerequisites**
>
> * Windows 10 / macOS 12 / Linux 5.15+
> * 8 GB RAM (16 GB recommended for Kali)
> * 5 GB free disk
> * DirectX 11 / Metal 2 / OpenGL 4.6 GPU

```bash
# 1 — Clone the repository
$ git clone https://github.com/retrovm/retrovm.git && cd retrovm

# 2 — Grab sub‑modules (pixel art, sound‑fonts, QEMU builds)
$ git submodule update --init --recursive

# 3 — Install dependencies (Node >= 20, pnpm, cargo, rust‑toolchain)
$ pnpm i && cargo build --release

# 4 — Run the launcher
$ pnpm run dev        # or `--prod` for minified assets
```

ℹ️ **First Run Wizard** downloads the default **Kali Linux 2025.2 ISO** (2.9 GB).  Bring coffee.

---

## ⚡ Quick Start

| Action             | Keys / Clicks                             | Outcome                                   |
| ------------------ | ----------------------------------------- | ----------------------------------------- |
| Launch Normal Mode | `Enter` on **Normal Mode**                | Auth prompt (`HOLA Amigo`) ➜ Desktop apps |
| Ghost Mode         | `Enter` on **Ghost Mode** or idle > 120 s | Animated ghost screen‑saver               |
| Hacker Mode        | While in Ghost Mode press `Shift + D`     | Kali VM console appears                   |
| Emergency Exit     | `Ctrl + Alt + Q`                          | Graceful shutdown of all VMs              |

---

## 🏗️ Architecture Primer

```text
┌─────────────────────────────┐
│  React 18 + Vite Front‑end  │  (Canvas renderer, UI state)
└────────┬────────────────────┘
         │ IPC (Wails)
┌────────▼────────────────────┐
│   Rust Core Engine          │  (ECS‑based game/apps runtime)
└────────┬────────────────────┘
         │ gRPC
┌────────▼────────────────────┐
│   QEMU 6.2 Wrapper (Rust)   │  (VM process supervision)
└─────────────────────────────┘
```

*All userland code is audited with **Clippy + MIRAI** and fuzz‑tested via **cargo‑fuzz**.*

---

## 🛡️ Hacker Mode Toolchain

Once inside Kali VM the following utilities are baked into */usr/local/bin* and accessible via the in‑app sidebar or CLI.

### 🕵️‍♂️ Digital Footprint Scanner

*OSINT aggregator for e‑reputation mapping.*

| Capability         | Command                      | Notes                                 |
| ------------------ | ---------------------------- | ------------------------------------- |
| Username Sweep     | `footprint hunt -u <handle>` | Checks 120+ platforms via public APIs |
| Breach Check       | `footprint leaks -e <email>` | Pwned? API with k‑anon hashing        |
| Metadata Harvester | `footprint meta -d <domain>` | WHOIS, SSL, DNS history               |
| Report Export      | `--report pdf/html`          | Drops to */reports/yyyy‑mm‑dd*        |

> **Disclaimer:** Operates solely on publicly available endpoints.  You are liable for local laws.

### 🌐 Network Scanner

*Rust‑flavoured wrapper around Nmap & Masscan with JSON output.*

Usage:

```bash
netradar scan 192.168.0.0/24 --top-ports 1000 --rate 10000 -o scan.json
netradar viz  scan.json             # Opens interactive graph
```

* All scans run inside the VM’s **bridge** adapter. Host network untouched.
* Results auto‑archived to Kali’s `/var/log/netradar/`.

### 🕸️ Fake Site Generator

*Swiss‑army knife for phishing simulations & red‑team training.*

| Step          | CLI                                           | Description                                             |
| ------------- | --------------------------------------------- | ------------------------------------------------------- |
| Template List | `phake list`                                  | Shows 40+ pre‑built site templates (Google, O365, etc.) |
| Generate      | `phake bake -t google-login -o /var/www/fake` | Builds pixel‑perfect clone                              |
| Launch        | `phake serve --ssl`                           | Starts Caddy on :443 with self‑signed cert              |
| Collect       | `phake harvest`                               | Live credentials & metadata view                        |

> **Ethics First:** Only use on infrastructure you own *or* have explicit consent to test.  Violations can land you in a cold cell with lousy Wi‑Fi.

---

## 🔐 Security & Compliance

* **Sandbox Isolation** — QEMU VM blocked by AppArmor & firejail; host FS mounted read‑only.
* **Secure Boot** — SHA‑256 checksums validated before VM launch.
* **Opt‑in Telemetry** — Disabled by default. When enabled only non‑PII perf metrics leave the box via TLS 1.3.
* **Privacy Pledge** — No trackers, no ads, no nonsense. Period.

---

## 📈 Performance Benchmarks

| Scenario                         | Intel i9‑13900H + RTX 4060 | Apple M3 Pro | AMD Ryzen 9 7940HS |
| -------------------------------- | -------------------------- | ------------ | ------------------ |
| UI Cold Start                    | 2.8 s                      | 2.4 s        | 3.1 s              |
| Switch Mode (Normal→Ghost)       | 590 ms                     | 530 ms       | 610 ms             |
| Kali VM Boot                     | 43 s                       | 39 s         | 46 s               |
| NetRadar /24 Scan (top 1K ports) | 28 s                       | 25 s         | 30 s               |

Benchmarks captured on Retro VM 0.9.0 — tweak `benchmarks.yml` to reproduce.

---

## 🤝 Contribution Guide

1. **Fork** the repo & `git checkout -b feat/amazing`
2. Run `pnpm run lint && cargo clippy --fix`
3. Add unit tests (**mandatory**), commit in past‑tense ("Fixed X", not "Fix X").
4. PR into `develop`; CI (GitHub Actions) must turn **green**.
5. One of the old‑timers will review within 48 h.

Coding Style:

* Front‑end — [StandardJS](https://standardjs.com) with 2‑space indent.
* Rust — `rustfmt +nightly` default profile.

---

## 🗺️ Roadmap

* [ ] **v1.0** — Plugin SDK, multi‑guest support (Win 11, Ubuntu LTS)
* [ ] **v1.1** — Distributed NetRadar (agent swarm)
* [ ] **v2.0** — Headless API & Terraform provider

Check `docs/ROADMAP.md` for the gory details.

---

## 📜 License

> **GPL‑3.0‑or‑later** — Yes, corporate lawyers may scream, but freedom is delicious.

Commercial licensing available; email **[sales@retrovm.dev](mailto:sales@retrovm.dev)** to swap vows.

---

## 🙏 Acknowledgements

* Pixel art by **@NeonByte**
* SID‑chiptune loops by **8BitBeeps Studio**
* QEMU wrappers inspired by **@crosvm** & **Firecracker** teams
* Early alpha testers: **Ananya**, **Ambi**, and my enduring insomnia.

> *“There is no cloud, just other people’s computers running QEMU.”*

---

### ⚠️ Final Words

Play nice, hack responsibly, and never trust a wizard offering free Wi‑Fi.

**Happy Computing!** 🚀👻🔓
