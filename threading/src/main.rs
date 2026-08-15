pub mod my_threads;
pub mod my_scoped_thread;

fn main() {
    println!("Hello, world!");

    // my_threads::spawn_thread();

    my_scoped_thread::test_thread_variables();
}
