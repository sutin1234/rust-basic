pub fn get_concurrency() {
    for i in 0..10 {
        println!("another thread: {}", i);
    }
}
