use colored::*;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc::channel, time::Duration};

pub fn watch_folder(folder: &str) {
    let (tx, rx) = channel();

    let config = Config::default()
        .with_poll_interval(Duration::from_millis(200));

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, config).unwrap();

    watcher
        .watch(Path::new(folder), RecursiveMode::Recursive)
        .unwrap();

    for event in rx {
        if let Ok(ev) = event {
            if let Some(path) = ev.paths.get(0) {
                println!("{} {}", "📂 File changed:".yellow(), path.display());
            }
        }
    }
}

