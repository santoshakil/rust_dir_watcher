use std::sync::mpsc::channel;
use std::thread;
use std::{path::Path, time::SystemTime};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("watching {}", path);

    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (sp, rp) = channel();

    let mut watcher = RecommendedWatcher::new(sp, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let handle = thread::spawn(move || {
        while let Ok(event) = rp.recv() {
            handle_event(&event.unwrap()).unwrap();
        }
    });

    handle.join().unwrap();

    Ok(())
}

fn handle_event(event: &notify::Event) -> notify::Result<()> {
    let id = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    if event.kind.is_create() {
        println!("{}: File created", id);
    } else if event.kind.is_modify() {
        println!("{}: File modified", id);
    } else if event.kind.is_remove() {
        println!("{}: File removed", id);
    } else if event.kind.is_access() {
        println!("{}: File accessed", id);
    } else {
        println!("{}: Other event", id);
    }
    println!("{:?}", event);

    Ok(())
}
