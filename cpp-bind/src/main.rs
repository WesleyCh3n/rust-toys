mod binding;

use crate::binding::{free_vec, get_vec, Counter, WThread};

fn main() {
    println!("Hello, world!");
    let mut cnt = Counter::new();
    (0..10).for_each(|_| cnt.inc());

    unsafe {
        let raw_ptr = get_vec();
        let vec = std::slice::from_raw_parts(raw_ptr, 10);
        for i in vec {
            println!("{i}");
        }
        free_vec(raw_ptr);
    }

    let mut t = WThread::new();
    t.start();
    std::thread::sleep(std::time::Duration::from_secs(5));
    t.stop();
}
