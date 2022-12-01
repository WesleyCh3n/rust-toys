use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();

    thread::spawn(move || {
        for i in 0..100 {
            println!("running: {}", i);
            if stop_clone.load(Ordering::Relaxed) {
                println!("{}: thread stop", i);
                break;
            }
            thread::sleep(time::Duration::from_millis(100));
        }
        println!("thread finished");
    });

    thread::sleep(time::Duration::from_millis(300));
    stop.store(true, Ordering::Relaxed);
    // wait for thread to show msg
    thread::sleep(time::Duration::from_millis(300));
    println!("All done");
}
