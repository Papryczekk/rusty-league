# 🦀 Rusty League

**Rusty League** is a lightweight, custom launcher and account manager for _League of Legends_ written in Rust. It allows you to store multiple accounts, switch between them quickly, and automatically log in to the Riot Client without re-typing your credentials every time.

## ✨ Features

- **Account Manager**: Securely store multiple accounts (Username, Password, Region, IGN, Tag).
- **Auto-Login**: Automatically launches the Riot Client and inputs your credentials using simulated keyboard input.
- **Process Management**: Button to instantly kill all League/Riot processes if the client freezes or if you're tired to play this game.
- **Minimalist Mode**: A compact view for quick access to launching the game.
- **Customizable**:
  - Set custom path to Riot Client.
  - Option to start with Windows.
  - Dark mode GUI.

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Core logic and performance.
- **[eframe / egui](https://github.com/emilk/egui)** - Immediate mode GUI library.
- **[enigo](https://github.com/enigo-rs/enigo)** - Cross-platform input simulation (for auto-typing passwords).
- **[uiautomation](https://crates.io/crates/uiautomation)** - Windows UI Automation to detect the Riot Client window state.
- **[serde](https://serde.rs/)** - Serialization for saving settings and encrypted credentials.

## 🚀 Installation & Building

### Prerequisites

- **Windows OS** (The project uses Windows specific APIs for process management and UI automation).
- **Rust Toolchain** (cargo).

### Build from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rusty-league.git
   cd rusty-league
   ```

2. Build the project in release mode:

   ```bash
   cargo build --release
   ```

3. The executable will be located in:
   `./target/release/rusty-league.exe`

## 📖 Usage

1. **First Launch**:
   - Go to **Settings** (Gear icon).
   - Select the path to your `RiotClientServices.exe` (usually in `C:\Riot Games\Riot Client\`).
   - Click "Confirm Settings".

2. **Adding Accounts**:
   - In the main view, fill in your login details (Username, Password, Region).
   - (Optional) Add your In-Game Name and Tag for easy identification.
   - Click **"Save Account"**.

3. **Logging In**:
   - Select an account from the list on the left.
   - Click **"Login To League"**.
   - _Hands off!_ The app will launch the client and type your password for you.

4. **Minimalist Mode**:
   - Enable "Minimalist Mode" in settings to shrink the window to a small launcher interface.

## ⚠️ Disclaimer

This project is a third-party tool and is **not affiliated with, endorsed, sponsored, or specifically approved by Riot Games**. Riot Games and League of Legends are trademarks or registered trademarks of Riot Games, Inc.

**Use at your own risk.** While this tool only simulates keyboard input (like a macro) and does not inject code into the game client, using automated login tools is always subject to the compiled terms of service of the game.

## 📄 License

[MIT](LICENSE)
