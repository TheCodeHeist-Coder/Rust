use std::thread::spawn;

pub fn test_threads() {
    let mut x = 0u128;

    for i in 1..500_0 {
        x += i;
    }

    println!("Main thread works a little bit... let's check some more threads");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;

        for i in 1..500_000_0 {
            x += i;
        }

        println!("The last value of x is: {}", x);
    };
    println!("Starting new thread ....");

    let handle: std::thread::JoinHandle<()> = spawn(thread_fn);
    let handle2: std::thread::JoinHandle<()> = spawn(thread_fn);

    println!("Ending the initialized thread");

    // test_threads();

    loop {
        test_threads();

        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are finished. ....");
            break;
        }
    }

    // handle.join();
    // handle2.join();
}
