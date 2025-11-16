# ☕ Coffee Break Terminal

A friendly terminal assistant that reminds you to take coffee breaks while coding, with project analysis and memory features.

## 🎬 Demo

<div align="center">
  

  **📹 [View Demo Video](https://github-production-user-asset-6210df.s3.amazonaws.com/7267026/514902713-1303d20e-9f4b-4b92-a136-d00b561adf4c.mp4?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20251116%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20251116T173838Z&X-Amz-Expires=300&X-Amz-Signature=8c048983d4c15d9b092df8d33572738d6c866666becabec1a4be577eb6732aa9&X-Amz-SignedHeaders=host)**
  
</div>

## ✨ Features

### ☕ Coffee Break Reminders
- **Automatic reminders** - Get notified to take breaks at configurable intervals
- **System notifications** - Cross-platform popups (macOS/Windows)
- **Visual dialog** - Beautiful dialog with coffee icon
- **Coffee animation** - Fun 20-second coffee dance animation

### 🔧 Built-in Commands
- `help` (or `?` or `h`) - Show help message
- `remember <text>` - Save information to persistent memory
- `mem` - Display all saved memories
- `insight` - Analyze current project structure
- `coffee` - Trigger coffee dance animation manually
- `explain <topic>` - AI explanation placeholder
- `bye` - Exit the program

### 🔍 Project Analysis
- Detects project type (Rust, Node.js, Python, Go, Java, etc.)
- Shows file statistics and types
- Lists dependency files
- Displays project structure tree

### 🛡️ Safety Features
- **File watcher** - Monitors workspace for changes
- **Safe shell execution** - Commands with `rm` require confirmation
- **Persistent memory** - Stores data in `~/.coffee_break_memory.json`

## 📦 Installation

### Prerequisites
- Rust 2021 edition or later
- macOS or Windows (for system notifications)

### Build from Source

```bash
# Clone the repository
git clone git@github.com:amirafroozeh1/coffee-break-cli.git
cd coffee-break-cli

# Build the project
cargo build --release

# The binary will be available at:
# ./target/release/coffee-break
```

## 🚀 Usage

### Command-Line Options

```bash
# Run with default 60-minute interval
cargo run

# Run with custom interval (in minutes)
cargo run -- --interval 30
# or use the short form
cargo run -- -i 30

# Run the release binary directly
./target/release/coffee-break --interval 45

# View help
./target/release/coffee-break --help
```

**Command-Line Arguments:**
- `--interval` or `-i`: Set coffee break interval in minutes (default: 60)
  - Example: `cargo run -- --interval 30` for 30-minute reminders
  - Example: `cargo run -- -i 15` for 15-minute reminders

### Interactive Commands

Once the program is running, you'll see a `coffee>` prompt. Available commands:

```bash
coffee> help          # Show all available commands
coffee> insight       # Analyze current project structure
coffee> remember Fix the bug in main.rs  # Save a note
coffee> mem           # View all saved memories
coffee> coffee        # Trigger coffee dance animation
coffee> explain Rust  # AI explanation (placeholder)
coffee> ls -la        # Execute shell commands
coffee> bye           # Exit the program
```

### Example Workflow

1. Start the terminal: `cargo run -- --interval 30`
2. Work on your project - the program watches for file changes
3. Get automatic reminders every 30 minutes
4. Use `insight` to analyze your project structure
5. Use `remember` to save important notes or TODOs
6. Type `bye` when you're done

## ⚙️ Configuration

- **Coffee break timing**: Set via `--interval` command-line parameter (in minutes)
  - Default: 60 minutes
  - Recommended: 25-30 minutes (Pomodoro technique)
- **Memory location**: Stored in `~/.coffee_break_memory.json`
- **File watcher**: Monitors the current working directory

## 📝 Examples

### Quick Start (25-minute Pomodoro intervals)
```bash
cargo run -- -i 25
```

### Long Coding Session (90-minute intervals)
```bash
cargo run -- --interval 90
```

### Using the Release Binary
```bash
# Build once
cargo build --release

# Run anytime
./target/release/coffee-break -i 30
```

## 🏗️ Project Structure

```
coffee_break_cli/
├── src/
│   ├── main.rs      # Main entry point and REPL loop
│   ├── coffee.rs    # Coffee break reminders and animations
│   ├── memory.rs    # Persistent memory system
│   ├── insight.rs   # Project analysis
│   ├── shell.rs     # Safe shell command execution
│   ├── watcher.rs   # File system watcher
│   └── resource/
│       ├── coffee_icon.png    # Icon for notifications
│       └── coffee_break.mp4   # Demo video
├── Cargo.toml
└── README.md
```

## 🔧 Development

### Running Tests
```bash
cargo test
```

### Building for Development
```bash
cargo build
# Binary will be at ./target/debug/coffee-break
```

### Running in Development Mode
```bash
cargo run -- -i 1  # Use 1-minute intervals for testing
```

## 🎯 Why Coffee Break Terminal?

Taking regular breaks is essential for:
- 🧠 **Better Focus** - Regular breaks improve concentration and productivity
- 💪 **Health** - Reduces eye strain and prevents burnout
- 🎨 **Creativity** - Stepping away helps solve complex problems
- ⚡ **Energy** - Maintains high energy levels throughout the day

## 📄 License

MIT License - Feel free to use and modify!

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

### Ideas for Contributions
- Add support for Linux notifications
- Integrate with productivity tracking tools
- Add customizable break messages
- Support for different types of breaks (short/long)
- Integration with local LLMs for the `explain` command

## 🙏 Acknowledgments

Made with ☕ and ❤️

Special thanks to the Rust community for the amazing tools and libraries.

---

**Note**: The coffee break reminders run in a background thread and won't interrupt your workflow. The notifications appear as system popups, and you can continue working while the reminders are active.
