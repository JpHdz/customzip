# customzip
Smart compression tool for developers. Right-click to zip projects while automatically ignoring node_modules, target, .git, and build artifacts. Available for Windows, Linux &amp; macOS.



<div align="center">

# 📦 czip
### Stop zipping `node_modules`.

**The compression tool designed for developers.** Create clean, lightweight backups of your projects with a single Right-Click.

[**Download on Gumroad**](PON_AQUI_TU_LINK_DE_GUMROAD) | [**View Website**](PON_TU_WEB_SI_TIENES)

![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![macOS](https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white)

</div>

---

## The Problem
You want to share your project or make a quick backup. You hit "Compress", and 5 minutes later you have a **400MB ZIP file** because it included:
- ❌ `node_modules` (Thousands of tiny files)
- ❌ `.git` folder (History you don't need in a zip)
- ❌ `target/` or `bin/` (Build artifacts)
- ❌ `__pycache__`

## The Solution: Czip
**Czip** is a native, cross-platform utility that understands developer environments. It compresses your source code while intelligently ignoring the heavy junk files automatically.

### Features
* **Context Menu Integration:** Just Right-Click on any folder -> **"Compress with czip"** / CLI supported too.
* **Smart Ignore:** Pre-configured to ignore `node_modules`, `.git`, `.vscode`, `target`, `dist`, `build`, `__pycache__`, `venv`, and more.
* **Native & Fast:** Written in **Rust** 🦀 for maximum speed and minimal file size.
* **Cross-Platform:** Works seamlessly on Windows, Linux, and macOS.
* **Zero Config:** Works out of the box, but fully customizable via `ignores.txt`.

---

## 📸 See it in action

### Windows
*(Right-click integration and native notifications)*
![Windows Demo](PON_AQUI_LINK_A_TU_GIF_DE_WINDOWS.gif)

### Linux / macOS
*(Terminal and Nautilus integration)*
![Linux Demo](PON_AQUI_LINK_A_TU_GIF_DE_LINUX.gif)

---

## 📥 How to Get It

Since this is a compiled binary tool, we distribute the installers via Gumroad.

### [👉 Download czip Installer](PON_AQUI_TU_LINK_DE_GUMROAD)

* **Pay what you want:** You can get it for free or support the development with a small donation.
* **Instant Setup:**
    * **Windows:** Comes with a professional `.exe` installer. Sets up PATH and Context Menu automatically.
    * **Unix:** Includes a `install.sh` script for one-command installation.

---

## ⚙️ Customization

Want to ignore specific files?
**Czip** looks for an `ignores.txt` file in its installation directory. You can add your own rules easily using standard glob patterns:

```text
# Default ignores.txt example
node_modules/
target/
.git/
*.log
secret_config.json
