use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{env, path::Path};

fn main() {
    // Info
    let os = env::var("OS").unwrap_or_else(|_| "unknown".to_string());
    let arch = env::var("PROCESSOR_ARCHITECTURE").unwrap_or_else(|_| "unknown".to_string());
    println!("OS: {}", os);
    println!("Architecture: {}", arch);

    // let path: String = String::from(".");
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("watching {}", path);

    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
