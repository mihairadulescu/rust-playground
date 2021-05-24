use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

pub fn run_threads() -> JoinHandle<()> {
    let result = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    result
}

pub async fn run_async() {
    print!("running Asynch");
}

pub async fn usage() {
    let join_handle: JoinHandle<()> = run_threads();
    run_async().await;
}