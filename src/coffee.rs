use colored::*;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn resolve_coffee_icon() -> Option<String> {
    let image_paths = vec![
        "./src/resource/coffee_icon.png",
        "./src/resource/coffee_icon.jpg",
        "./coffee_icon.png",
        "./coffee_icon.icns",
        "./coffee_icon.jpg",
        "./coffee_icon.jpeg",
        "./assets/coffee_icon.png",
        "./assets/coffee_icon.icns",
        "./assets/coffee_icon.jpg",
        "./coffee.png",
        "./coffee.jpg",
    ];

    for path in image_paths {
        let candidate = Path::new(path);
        if candidate.exists() {
            if let Ok(abs) = std::fs::canonicalize(candidate) {
                return Some(abs.to_string_lossy().to_string());
            } else {
                return Some(candidate.to_string_lossy().to_string());
            }
        }
    }

    None
}

#[cfg(target_os = "macos")]
fn run_osascript(script: &str, context: &str) {
    match Command::new("osascript").arg("-e").arg(script).output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "[coffee reminder] {} failed: {}",
                    context,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(err) => {
            eprintln!("[coffee reminder] could not run osascript for {}: {}", context, err);
        }
    }
}

#[cfg(target_os = "windows")]
fn run_powershell(script: &str, context: &str) {
    match Command::new("powershell")
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(script)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "[coffee reminder] {} failed: {}",
                    context,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(err) => {
            eprintln!(
                "[coffee reminder] could not run PowerShell for {}: {}",
                context, err
            );
        }
    }
}

fn show_system_notification(title: &str, message: &str) {
    #[cfg(target_os = "macos")]
    {
        let icon_clause = resolve_coffee_icon()
            .map(|p| format!("with icon POSIX file \"{}\"", p))
            .unwrap_or_else(|| "with icon note".to_string());

        let script = format!(
            "display notification \"{}\" with title \"{}\" sound name \"Glass\" {}",
            message, title, icon_clause
        );

        run_osascript(&script, "notification");
    }

    #[cfg(target_os = "windows")]
    {
        fn escape_ps(value: &str) -> String {
            value.replace('"', "`\"")
        }

        let icon_snippet = resolve_coffee_icon().map(|path| {
            let url = format!(
                "file:///{}",
                path.replace('\\', "/").replace(' ', "%20")
            );
            format!(
                "$xml.GetElementsByTagName(\"image\")[0].Attributes.GetNamedItem(\"src\").Value = \"{}\";",
                url
            )
        }).unwrap_or_default();

        let script = format!(
            r#"
Add-Type -AssemblyName System.Runtime.WindowsRuntime | Out-Null
[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
$template = [Windows.UI.Notifications.ToastTemplateType]::ToastImageAndText02
$xml = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent($template)
$xml.GetElementsByTagName("text")[0].AppendChild($xml.CreateTextNode("{title}")) | Out-Null
$xml.GetElementsByTagName("text")[1].AppendChild($xml.CreateTextNode("{message}")) | Out-Null
{icon}
$toast = [Windows.UI.Notifications.ToastNotification]::new($xml)
$notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("coffee-break")
$notifier.Show($toast)
"#,
            title = escape_ps(title),
            message = escape_ps(message),
            icon = icon_snippet
        );

        run_powershell(&script, "notification");
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        println!("☕ {} - {}", title, message);
    }
}

#[cfg(target_os = "macos")]
fn show_coffee_dialog() {
    // Show a compact modal dialog box with coffee theme
    // This requires user interaction (clicking OK)

    let icon_clause = resolve_coffee_icon()
        .map(|p| format!("with icon POSIX file \"{}\"", p))
        .unwrap_or_else(|| "with icon note".to_string());

    let script = format!(
        r#"
        tell application "System Events"
            activate
        end tell
        
        display dialog "☕ COFFEE BREAK TIME! ☕

You've been coding for a while!
Time to take a break and grab some coffee! 🍵

Your code will still be here when you return!" with title "☕ Coffee Break" buttons {{"☕ OK ☕"}} default button 1 {} giving up after 30
        "#,
        icon_clause
    );
    
    run_osascript(&script, "dialog");
}

#[cfg(not(target_os = "macos"))]
fn show_coffee_dialog() {
    // Dialog not available on this platform; notification already shown
}

pub fn start_coffee_dance(interval_minutes: u64) {
    thread::spawn(move || {
        loop {
            // Wait for the specified interval (in minutes)
            thread::sleep(Duration::from_secs(interval_minutes * 60));
            
            // Show system notification popup
            let message = if interval_minutes == 1 {
                format!("You've been coding for {} minute! Time for coffee!", interval_minutes)
            } else {
                format!("You've been coding for {} minutes! Time for coffee!", interval_minutes)
            };
            show_system_notification("☕ Coffee Break Time!", &message);
            
            // Show modal dialog popup in IDE/terminal
            show_coffee_dialog();
            
            // Start coffee dance animation directly
            show_coffee_dance();
        }
    });
}

pub fn show_coffee_dance() {
    // Run animation in a separate thread so it doesn't block REPL
    thread::spawn(move || {
        // Coffee dance animation frames
        let frames = vec![
            "     ☕",
            "    ☕ ",
            "   ☕  ",
            "  ☕   ",
            " ☕    ",
            "☕     ",
            " ☕    ",
            "  ☕   ",
            "   ☕  ",
            "    ☕ ",
        ];

        let start_time = std::time::Instant::now();
        let duration = Duration::from_secs(20);
        let mut frame_index = 0;

        // Show system notification
        show_system_notification("☕ Coffee Dance!", "Coffee animation starting in terminal!");
        
        // Print initial message
        println!("\n{}", "☕ Coffee dance starting! ☕".bright_yellow().bold());

        // Show animation for 20 seconds
        while start_time.elapsed() < duration {
            let frame = &frames[frame_index % frames.len()];
            // Print each frame on a new line for visibility
            println!("{}", frame.bright_yellow().bold());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
            frame_index += 1;
        }
        
        // Final message
        println!("{}", "☕ Coffee time! ☕".bright_yellow().bold());
        println!(); // Extra newline for spacing
    });
}

