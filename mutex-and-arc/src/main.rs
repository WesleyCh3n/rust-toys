use std::sync::{Arc, Mutex};
fn main() {
    let data: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let clone_data = Arc::clone(&data);
    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            let mut number = clone_data.lock().unwrap();
            *number += 1
        }
    });

    let clone_data = Arc::clone(&data);
    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            let mut number = clone_data.lock().unwrap();
            *number += 1
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("data result: {}", data.lock().unwrap());
}
