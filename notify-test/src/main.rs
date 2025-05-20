use std::path::Path;

use notify::{Watcher, RecursiveMode, Result};

fn main() -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    println!("watcher: {:?}", watcher);

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("/Users/steve/Downloads"), RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        // Wait for events
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Ok(())
}
