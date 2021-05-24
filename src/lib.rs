//Make connection send available outside.
use crate::network::HttpSend;
use std::thread::JoinHandle;


mod presentation;
mod datatypes;
mod storage;
mod network;
mod threads;

async fn main(){
    //Talk about the modules and how you define them.
    presentation::say_hello();
    //Introduction to types.
    datatypes::show_types();

    presentation::usage_of_types();

    //Equivalent of a class?
    storage::storage_usage();

    //Equivalent of interfaces? Traits?
    network::usage();

    //Threads
    threads::usage();

}




#[cfg(test)]
mod tests {
    use crate::main;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        println!("Main Thread Running!.");
        let runner = main();
         runner.await;
        thread::sleep(Duration::from_millis(1000));
    }
}
