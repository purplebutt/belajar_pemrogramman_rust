#![allow(unused_imports)]
use std::time::Duration;
use std::thread;

async fn sleep1sec_blocking(task: &str) {
    println!("{}", "~".repeat(30));
    println!("Entering sleep 1 sec, blocking");
    // thread::sleep(Duration::from_secs(1));
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Returning from sleep 1 sec, blocking {task}");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
// #[tokio::main]
async fn main() {
    let ins = std::time::Instant::now();

    // println!("Test 1: run 2 async sequentially!");
    // sleep1sec_blocking("Job A").await;
    // sleep1sec_blocking("Job B").await;

    // println!("Test 2: run 2 async task concurrently (same thread)");
    // tokio::join!(
    //     sleep1sec_blocking("Job C"),
    //     sleep1sec_blocking("Job D")
    // );

    println!("Test 3: Run 2 async task in parallel");
    let _ = tokio::join!(
        tokio::spawn(sleep1sec_blocking("Task E")),
        tokio::spawn(sleep1sec_blocking("Task F")),
    );

    let xtime = ins.elapsed().as_secs();
    println!("Exec time: {xtime} second(s)");
    return ();
}

