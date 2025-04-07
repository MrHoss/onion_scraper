### ✅ `README.md` (atualizado com **GPL**, instruções de build e execução):

```markdown
# Onion Scraper

A multithreaded `.onion` address tester that attempts to discover reachable Tor hidden services by generating random v3 addresses and checking their availability through the Tor network.

⚠️ **Disclaimer**: This tool is for educational and research purposes only. Use it responsibly and legally. Do not access unauthorized services.

---

## ✨ Features

- Generates random 56-character `.onion` v3 addresses
- Uses the Tor SOCKS5 proxy to test reachability
- Multithreaded for higher throughput
- Outputs successful hits to a file
- Configurable via `.env` file

---

## 🔧 Requirements

- Rust (latest stable)
- Tor proxy running locally
- `.env` file (see config below)

---

## ⚙️ Configuration

Create a `.env` file in the root:

```env
PROXY_ADDRESS=127.0.0.1:9150
NUM_THREADS=10
TIMEOUT_SECS=10
TOTAL_REQUESTS=100
OUTPUT_FILE=found_onions.txt
```

> Use `127.0.0.1:9150` for Tor Browser or `127.0.0.1:9050` for system Tor.

---

## 🛠️ Build & Run

```bash
git clone https://github.com/yourusername/onion-scraper.git
cd onion-scraper

# Build optimized binary
cargo build --release

# Run
./target/release/onion_scraper
```

---

## 📁 Output

Valid `.onion` URLs will be printed to the console and saved in the file defined by `OUTPUT_FILE`.

---

## ❗ Limitations

- Random v3 `.onion` discovery is probabilistically unlikely
- Most hidden services are private and not discoverable this way
- This tool is more experimental and educational than practical

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**.

---

## 🤝 Contributing

Pull requests and suggestions are welcome! Fork the repo and send improvements.