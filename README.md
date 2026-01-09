# customzip
Smart compression tool for developers. Right-click to zip projects while automatically ignoring node_modules, target, .git, and build artifacts. Available for Windows, Linux &amp; macOS.



<div align="center">

# 📦 czip
### Stop zipping `node_modules`.

**The compression tool is designed for developers.** Create clean, lightweight backups of your projects with a single Right-Click.
</div>

---

## The Problem
You want to share your project or make a quick backup. You hit "Compress", and 5 minutes later, you have a **400MB ZIP file** because it included:
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

## 📥 How to Get It
The download to this tool is attached to the current repository.

* **Instant Setup:**
    * **Windows:** Comes with a professional `.exe` installer. Sets up PATH and Context Menu automatically.
    * **Unix:** Includes a `install.sh` script for one-command installation.
    *    To install in unix just do sudo ./install.sh

---

## CLI 
You can use the tool via cli by writing the following command - czip <path-to-compress>
Example:
czip C:\Users\myuser\folderexample

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
```

Feel free to modify the ignores.txt to add or remove ignored files in compression. This tool uses regex, so you are welcome to add your own expressions. 
