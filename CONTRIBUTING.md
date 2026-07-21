# Contributing to Catisen

We welcome contributions from the community! Here’s how you can help:

---

## 🛠️ How to Contribute

### 1. Fork the Repository
- Click the "Fork" button on GitHub to create your own copy.

### 2. Clone Your Fork
```bash
git clone https://github.com/YourUsername/catisen.git
cd catisen
```

### 3. Set Up the Environment
#### **For Servo (Rust)**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install dependencies
sudo apt-get install cmake pkg-config libssl-dev libgtk-3-dev
# Build Servo
cd servo
./mach build --release
```

#### **For WebKitGTK (C/GTK)**
```bash
# Install dependencies
sudo apt-get install build-essential cmake ninja-build gtk-doc-tools gobject-introspection libgtk-3-dev libwebkit2gtk-4.0-dev
# Build WebKitGTK
cd webkitgtk
cmake -S . -B build -G Ninja
cmake --build build
```

### 4. Make Changes
- Create a new branch for your feature/bugfix:
  ```bash
  git checkout -b feature/your-feature
  ```
- Commit your changes with a clear message:
  ```bash
  git commit -m "Add feature: description of changes"
  ```

### 5. Submit a Pull Request
- Push your changes to your fork:
  ```bash
  git push origin feature/your-feature
  ```
- Open a PR on GitHub and describe your changes.

---

## 📌 Guidelines
- Follow the [code of conduct](CODE_OF_CONDUCT.md).
- Write clear, concise commit messages.
- Test your changes before submitting.
- Document new features in the [wiki](wiki/).

---

## 🔍 Areas to Contribute
| Area                | Description                          |
|---------------------|--------------------------------------|
| **Ad Blocking**     | Improve filter lists, add new rules. |
| **UI/UX**           | Custom themes, controls.             |
| **Download Manager**| Add pause/resume, speed limits.      |
| **Media Playback**  | Optimize FFmpeg, improve PiP.        |
| **Documentation**   | Update README, wiki, usage guides.   |

---

## 📚 Resources
- [Project Wiki](wiki/)
- [Servo Documentation](https://servo.org/docs/)
- [WebKitGTK Documentation](https://webkitgtk.org/documentation.html)