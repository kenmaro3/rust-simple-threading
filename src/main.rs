use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn my_function1() {
    println!("my_function1");
}

fn my_function2() {
    println!("my_function2");
}

fn create_waiting_thread(
    should_continue: &Arc<Mutex<bool>>,
    should_block: &Arc<Mutex<bool>>,
    flag_my_function1: &Arc<Mutex<bool>>,
    flag_my_function2: &Arc<Mutex<bool>>,
) -> JoinHandle<()> {
    // Clone the Arc for the thread
    let thread_should_continue = Arc::clone(&should_continue);
    let thread_should_block = Arc::clone(&should_block);
    let thread_flag_my_function1 = Arc::clone(&flag_my_function1);
    let thread_flag_my_function2 = Arc::clone(&flag_my_function2);

    // Create a new thread
    let thread_handle = thread::spawn(move || {
        // Loop while the condition is true
        while *thread_should_continue.lock().unwrap() {
            // Do some work
            println!("Thread is running...");

            // Sleep for some time
            thread::sleep(Duration::from_secs(1));

            if *thread_flag_my_function1.lock().unwrap() {
                println!("\n===============thread_flag_my_function1: true");
                let res = my_function1();
                *thread_flag_my_function1.lock().unwrap() = false;
                // *thread_shared_file_name.lock().unwrap() = String::from("");
                println!("thread_flag_my_function1: false");
                *thread_should_block.lock().unwrap() = false;
            }
            if *thread_flag_my_function2.lock().unwrap() {
                println!("\n===============thread_flag_my_function2: true");
                let res = my_function2();
                *thread_flag_my_function2.lock().unwrap() = false;
                // *thread_shared_file_name.lock().unwrap() = String::from("");
                println!("thread_flag_my_function2: false");
                *thread_should_block.lock().unwrap() = false;
            }
        }
    });
    return thread_handle;
}

fn api_my_function1(
    should_block: &Arc<Mutex<bool>>,
    flag_my_function1: &Arc<Mutex<bool>>,
) -> String {
    *flag_my_function1.lock().unwrap() = true;
    *should_block.lock().unwrap() = true;

    while *should_block.lock().unwrap() {
        thread::sleep(Duration::from_micros(1));
    }

    let res = String::from("res");
    return res;
}

fn api_my_function2(
    should_block: &Arc<Mutex<bool>>,
    flag_my_function2: &Arc<Mutex<bool>>,
) -> String {
    *flag_my_function2.lock().unwrap() = true;
    *should_block.lock().unwrap() = true;

    while *should_block.lock().unwrap() {
        thread::sleep(Duration::from_micros(1));
    }

    let res = String::from("res");
    return res;
}

fn main() {
    // Create a shared mutable state using Arc and Mutex
    let should_continue = Arc::new(Mutex::new(true));
    let should_block = Arc::new(Mutex::new(false));
    let flag_my_function1 = Arc::new(Mutex::new(false));
    let flag_my_function2 = Arc::new(Mutex::new(false));

    let mut thread_handle = create_waiting_thread(
        &should_continue,
        &should_block,
        &flag_my_function1,
        &flag_my_function2,
    );

    // Wait for user input to stop the thread
    println!("Press Enter to stop the thread...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // call api_my_function1
    let res = api_my_function1(&should_block, &flag_my_function1);

    // call api_my_function2
    let res = api_my_function2(&should_block, &flag_my_function2);

    // Set the condition to false to stop the thread
    *should_continue.lock().unwrap() = false;

    // Wait for the thread to finish
    thread_handle.join().unwrap();

    println!("Thread stopped.");
}
