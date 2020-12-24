use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

mod fork;
fn main() {
    fork::forker();
    std::process::exit(0);
    // Create a channel to receive the events.
    let (sender, receiver) = channel();
    let eval = "php ./testfolder/main.php";
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("./testfolder", RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}