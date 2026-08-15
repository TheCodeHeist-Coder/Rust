pub mod my_threads;

fn main() {
    println!("Hello, world!");

    my_threads::spawn_thread();
}
