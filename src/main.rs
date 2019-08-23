fn main() {
    println!("Hello, world!");

    // https://github.com/balena-io-projects/balena-rust-hello-world/blob/master/src/main.rs
    // Infinite loop - otherwise the application will quit and the container
    // will be launched again and again and your logs will be flooded with
    // the "Hello, world!" messages.
    loop {
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
