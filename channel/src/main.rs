use std::thread::spawn;
fn main() {
    basic_usage();
}

fn basic_usage() {
    let (sender, reciver) = std::sync::mpsc::channel();
    let sender_clone = sender.clone(); // sneder can clone just like Arc
    let mut handl_vec = vec![];

    handl_vec.push(spawn(move || {
        sender.send("Send 1").unwrap();
    }));
    handl_vec.push(spawn(move || {
        sender_clone.send("Send 2").unwrap();
    }));

    // Opt 1
    // for _ in handl_vec {
    //     println!("{:?}", reciver.recv().unwrap())
    // }

    // Opt 2
    /* for received in reciver {
        println!("{}", received);
    } */

    // Opt 3, none bloking
    // while let Ok(received) = reciver.try_recv() {
    //     println!("{}", received);
    // }

    let hdl = std::thread::spawn(move || loop {
        if let Ok(received) = reciver.recv() {
            println!("{}", received);
        }
    });
    for handle in handl_vec {
        handle.join().unwrap();
    }

    println!("close main");
}
