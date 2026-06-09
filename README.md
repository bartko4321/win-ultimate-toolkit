# WinUltimate Toolkit 🛠️🚀

**WinUltimate Toolkit** is an advanced, graphical tool for optimizing, cleaning, and automating the configuration of the Windows operating system. The application is written in **Rust** using the modern, ultra-fast GUI library **eframe / egui**. It was built with power users and administrators in mind — people who want to quickly set up a freshly installed Windows system for work or gaming by eliminating unnecessary services, telemetry, and so-called bloatware.

The application features full **bilingual support (Polish / English)**, automatically detecting or allowing the user to switch the interface language.

---

## 📺 Main Modules & Features

### 1. 📦 Software Installer
Bulk, automated installation of popular software without visiting dozens of websites.
* **Winget integration:** The tool makes full use of the official Windows package manager (`winget`).
* **Smart detection:** Before installing, the app checks whether a given application is already present on the system, saving time and bandwidth.
* **Wide category selection:** Browsers, developer tools, media players, messaging apps, and runtime libraries.

### 2. 🛠️ System Tools
A one-click command center for system repair and diagnostics.
* **SFC & DISM Scans:** Automated execution of system file repair and Windows image restoration procedures (`sfc /scannow`, `DISM /Online /Cleanup-Image /RestoreHealth`).
* **Disk Management:** Quick access to `CHKDSK` error checking and forced storage optimization with the `TRIM` command for SSDs.
* **Network Optimization:** Instantly switch your DNS configuration to the ultra-fast and secure **Cloudflare (1.1.1.1)** servers, with simultaneous DNS cache flushing (`ipconfig /flushdns`).

### 3. 🗑️ Debloater
Remove apps that Microsoft doesn't allow you to easily uninstall through the standard Control Panel.
* **Complete Xbox service removal:** Disable and remove all processes related to the Xbox app — ideal for office machines or virtual environments.
* **Disable AI Copilot:** Remove Microsoft's AI integration from the taskbar and registry.
* **Built-in app removal:** Safely remove Microsoft Edge, system Widgets, and optionally clean up classic apps like Notepad and Calculator (if you prefer your own alternatives).

### 4. ⚙️ Service & Registry Tweaks
Advanced system modifications aimed at reclaiming RAM/CPU resources and improving privacy.
* **Telemetry Blocking:** Maximize restrictions on diagnostic and tracking data being sent to Microsoft's servers.
* **Windows Update Pause:** Freeze automatic system updates for up to 180 days, preventing unexpected restarts during work.
* **Visual Performance:** Instantly switch Windows visual effects to maximum performance mode (disabling resource-heavy animations).
* **UI Modifications:** Customize the Start menu, and toggle hidden options in File Explorer (e.g. showing file extensions for known file types).

---

## 🛠️ Requirements & Technologies

The project is built on the modern and safe Rust ecosystem:
* **Language:** Rust (2021 edition or newer)
* **GUI Framework:** `egui` with the `eframe` backend (providing native hardware rendering, low RAM usage, and instant responsiveness).
* **Operating System:** Windows 10 / Windows 11 (administrator privileges required for most operations).

---

## 🚀 How to Build & Run

To compile the project from source, you need a Rust environment (Cargo) installed.

1. Clone this repository
   ```bash
   git clone https://github.com/bartko4321/win-ultimate-toolkit.git
   ```
   
2. Enter the downloaded folder
   ```bash
   cd win-ultimate-toolkit
   ```

3. Run the app in development mode
   ```bash
   cargo run
   ```

4. Build an optimized production release (the compiled `.exe` will be found in `/target/release/`)
   ```bash
   cargo build --release
   ```

> 💡 **Tip:** Since the application makes deep modifications to the registry and system services, **always run the compiled `.exe` as Administrator**.

---

Bank account for support: 06291000060000000005038936

## ⚠️ Disclaimer

*This tool makes advanced changes to the Windows operating system configuration, including the system registry and system services. The author takes no responsibility for any system damage, data loss, or instability caused by incorrect or unintended use of the application. Before running aggressive cleanup scripts (Tweaks/Debloater), it is strongly recommended to create a System Restore Point.*

---
Made with passion in 🦀 **Rust**. If you find this project useful, leave a star! ⭐

<img width="1920" height="1050" alt="Zrzut ekranuEN" src="https://github.com/user-attachments/assets/c302d793-ac54-4b24-a8fe-c36cd6c6e9f4" />
