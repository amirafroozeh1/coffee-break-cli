use colored::*;
use std::process::Command;

pub fn run_smart_shell(cmd: &str) {
    // Safety layer: warn before dangerous commands
    if cmd.starts_with("rm ") || cmd.contains(" rm ") {
        println!("{}", "⚠️  You are about to delete files. Proceed? (y/n)".red());
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        if answer.trim() != "y" {
            println!("❌ Cancelled.");
            return;
        }
    }

    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute shell");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    println!("{}", "✔️  done".bright_green());
}


