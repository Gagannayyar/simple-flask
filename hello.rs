use std::thread;

fn main() {
    let v = vec![1,2,3];

    

    let mut thread_handle = Vec::new();

    for e in v {
        thread_handle.push(thread::spawn(move || println!("Thread {}", e)));
    }

    println!("Main Thread");

    for handle in thread_handle {
        handle.join().unwrap();
    }
}