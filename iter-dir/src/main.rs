use std::fs::read_dir;

fn main() {
    let entries = read_dir("./").unwrap();
    for entry in entries {
        if let Ok(e) = entry {
            if let Ok(metadata) = e.metadata() {
                if metadata.is_dir() {
                    println!("Dir: {:?}", e.path());
                } else {
                    println!("Not Dir: {:?}", e.path());
                }
            } else {
                println!("Error getting meta data: {:?}", e.path())
            }
        }
    }
}
