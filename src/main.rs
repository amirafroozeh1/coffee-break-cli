use colored::*;
use std::io::{self, Write};
use clap::Parser;

mod memory;
mod shell;
mod watcher;
mod insight;
mod coffee;

use memory::*;
use shell::*;
use watcher::*;
use insight::*;
use coffee::*;

#[derive(Parser)]
#[command(name = "coffee-break")]
#[command(about = "A friendly terminal assistant that reminds you to take coffee breaks", long_about = None)]
struct Args {
    /// Coffee break interval in minutes (default: 60)
    #[arg(short, long, default_value_t = 60)]
    interval: u64,
}

fn show_help() {
    println!("\n{}", "=".repeat(70).bright_cyan());
    println!("{}", "☕ COFFEE BREAK TERMINAL - Available Commands".bright_cyan().bold());
    println!("{}\n", "=".repeat(70).bright_cyan());

    println!("{}", "📝 Built-in Commands:".bright_yellow().bold());
    println!("  {}  {} - Show this help message", "help".bright_cyan(), "(or ? or h)".bright_white());
    println!("  {}  {} - Save information to persistent memory", "remember <text>".bright_cyan(), "💾".bright_green());
    println!("  {}  {} - Display all saved memories", "mem".bright_cyan(), "🧠".bright_green());
    println!("  {}  {} - Analyze current project structure", "insight".bright_cyan(), "🔍".bright_green());
    println!("  {}  {} - Trigger coffee dance animation", "coffee".bright_cyan(), "☕".bright_green());
    println!("  {}  {} - AI explanation (placeholder)", "explain <topic>".bright_cyan(), "🔮".bright_green());
    println!("  {}  {} - Exit Coffee Break Terminal", "bye".bright_cyan(), "👋".bright_green());

    println!("\n{}", "☕ Coffee Break Features:".bright_yellow().bold());
    println!("  • {} - Automatic coffee break reminders (configurable via --interval)", "Auto Reminders".bright_green());
    println!("  • {} - System notifications (macOS/Windows)", "System Popups".bright_green());
    println!("  • {} - Beautiful dialog with coffee icon", "Visual Dialog".bright_green());
    println!("  • {} - 20-second coffee dance animation", "Coffee Animation".bright_green());

    println!("\n{}", "🔧 Background Features:".bright_yellow().bold());
    println!("  • {} - Watches your workspace for file changes", "File Watcher".bright_green());
    println!("  • {} - Persistent memory storage (~/.coffee_break_memory.json)", "Memory System".bright_green());
    println!("  • {} - Safe shell command execution with rm protection", "Shell Integration".bright_green());

    println!("\n{}", "💡 Project Analysis (insight command):".bright_yellow().bold());
    println!("  • Detects project type (Rust, Node.js, Python, Go, Java, etc.)");
    println!("  • Shows file statistics and types");
    println!("  • Lists dependency files");
    println!("  • Displays project structure tree");

    println!("\n{}", "⌨️  Shell Commands:".bright_yellow().bold());
    println!("  • Any other command is executed as a shell command");
    println!("  • {} - Commands with 'rm' require confirmation", "Safety Feature".bright_red());

    println!("\n{}", "=".repeat(70).bright_cyan());
    println!();
}

fn main() {
    let args = Args::parse();
    
    println!("{}", r#"
  ██████╗ ██████╗ ███████╗███████╗███████╗███████╗    ██████╗ ██████╗ ███████╗ █████╗ ██╗  ██╗
 ██╔═══██╗██╔══██╗██╔════╝██╔════╝██╔════╝██╔════╝    ██╔══██╗██╔══██╗██╔════╝██╔══██╗██║ ██╔╝
 ██║   ██║██████╔╝█████╗  █████╗  █████╗  █████╗      ██████╔╝██████╔╝█████╗  ███████║█████╔╝ 
 ██║   ██║██╔══██╗██╔══╝  ██╔══╝  ██╔══╝  ██╔══╝      ██╔══██╗██╔══██╗██╔══╝  ██╔══██║██╔═██╗ 
 ╚██████╔╝██║  ██║██║     ██║     ███████╗███████╗    ██████╔╝██║  ██║███████╗██║  ██║██║  ██╗
                                                                                            v0.1
"#.bright_cyan());

    println!("{}", "☕ Coffee Break Terminal initialized and watching your workspace…\n".bright_green());
    println!("{}", format!("☕ Coffee break reminders: every {} minute(s), then 20 sec animation!\n", args.interval).bright_yellow());

    let mut mem = Memory::load();

    // Start file watcher thread
    std::thread::spawn(move || watch_folder("./"));

    // Start coffee dance (runs in background thread) with specified interval
    start_coffee_dance(args.interval);

    // Main REPL loop
    loop {
        print!("{}", "coffee> ".bright_cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        // exit
        if cmd == "bye" {
            println!("{}", "👋 See you later!".bright_yellow());
            break;
        }

        // Coffee Break's own commands
        if cmd.starts_with("remember ") {
            let data = cmd.trim_start_matches("remember ").to_string();
            mem.data.push(data.clone());
            mem.save();

            println!("💾 saved: {}", data.bright_green());
            continue;
        }

        if cmd == "mem" {
            println!("🧠 Memory:");
            for (i, item) in mem.data.iter().enumerate() {
                println!("{} {}", i, item.bright_yellow());
            }
            continue;
        }

        if cmd.starts_with("explain ") {
            let topic = cmd.trim_start_matches("explain ");
            println!("🔮 AI explanation placeholder for '{}'", topic);
            println!("(Here you can connect to a local LLM like llama.cpp)");
            continue;
        }

        if cmd == "insight" {
            let insight = analyze_project("./");
            print_insight(&insight);
            continue;
        }

        if cmd == "coffee" {
            show_coffee_dance();
            continue;
        }

        if cmd == "help" || cmd == "?" || cmd == "h" {
            show_help();
            continue;
        }

        // Execute shell commands (with safety layer)
        run_smart_shell(cmd);
    }
}


