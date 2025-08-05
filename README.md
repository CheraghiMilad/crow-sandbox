# 🐦 Crow Sandbox

**Crow Sandbox** is a modular Linux malware analysis environment designed for automation, performance, and extensibility.  
This project aims to provide a container- or VM-based sandbox that supports static and dynamic analysis of suspicious files with a focus on Linux environments.

> ⚠️ Crow Sandbox is under active development — expect changes and improvements.

---

## 🧠 How It Works

Crow is split into multiple components:

- **`crow-daemon`**: RESTful API to receive and validate malware samples
- **`crow-manager`**: Orchestrates the analysis environment (Docker/VM), loads and executes samples
- **`crow-core`**: Shared logic and utilities (e.g., file handling, config, DB layer)
- **`crow-cli`**: Local CLI interface for interacting with the sandbox
- **`crow-web`**: Frontend interface (in progress) ---> soooon...

---

## 🚀 How to Run It

### 🔧 Build the entire workspace
```bash
cargo build
```

---

### 🧪 Run with API and manager
```bash
cargo run -p crow-daemon
cargo run -p crow-manager
```

---

### 📦 Run a file analysis locally using CLI
```bash
cargo run -p crow-cli -- --file /usr/bin/ls
```


---

## 📌 Notes

- Designed to analyze **Linux malware samples**
- Modular architecture = easy to test & extend each part
- Future integration: Zeek, YARA, Volatility (Linux), Suricata

---

## 📬 Contributing

Suggestions, bug reports, or feature requests are welcome.  
Feel free to open an issue or submit a pull request!

---

## 📄 License

This project is released under the MIT License.