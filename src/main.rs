use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

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
            handle_event(&event).unwrap();
        }
    });

    handle.join().unwrap();

    Ok(())
}

fn handle_event(data: &Result<notify::Event, notify::Error>) -> notify::Result<()> {
    let event = data.as_ref().unwrap();
    let path = match event.paths.first() {
        Some(path) => path,
        None => return Ok(()),
    };

    if event.kind.is_create() {
        println!("File created: {:?}", path);
    } else if event.kind.is_modify() {
        println!("File modified: {:?}", path);
    } else if event.kind.is_remove() {
        println!("File removed: {:?}", path);
    } else if event.kind.is_access() {
        println!("File accessed: {:?}", path);
    } else {
        println!("Other event: {:?}", path);
    }

    Ok(())
}
