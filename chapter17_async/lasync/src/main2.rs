#![allow(dead_code)]

fn server() -> std::io::Result<()>{
    use std::io::Read; // agar metode read pada File bisa di akses
    use std::net::TcpListener;
    use cbfr::prelude::BFRDYN;

    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    for data_stream in listener.incoming() {
        let mut data = data_stream.unwrap();
        println!("Connected!");
        let mut buffer = BFRDYN::withcap::<512>();
        let result = data.read(unsafe {buffer.bytes_mut()});
        match result {
            Ok(ok) => {
                buffer.auto_len();
                print!("Incoming: {}", buffer);
                println!("Received: {ok} byte(s)");
            }
            Err(e) => {
                return Err(e)
            }
        }
    }
    Ok(())
}


fn fib(n: u32) -> u128 {
    if n < 2 { return 1 }
    else {
        fib(n-1) + fib(n-2)
    }
}

fn readfile() -> std::io::Result<()> {
    use std::io::Read; // agar metode read pada File bisa di akses
    use std::fs::File;
    use cbfr::prelude::BFRDYN;

    let mut file = File::open("./test.txt")?;
    let mut buffer: BFRDYN<512> = BFRDYN::new();
    let _ = file.read(unsafe {buffer.bytes_mut()})?;
    buffer.auto_len();
    println!("{}", buffer);
    Ok(())
}

fn run() -> std::io::Result<()> {
    println!("Hello, world!");

    let fib10 = fib(40);    // set this > 40 will make program to run slower
    println!("Fib 10 = {fib10}");

    // server()?;
    readfile()
}

fn send_http_request(url: &str) {
    use std::thread::sleep;
    use std::time::Duration;

    println!("Send request to {url}");
    sleep(Duration::from_millis(500));
    println!("Receive response from {url}");
}

fn write_to_file(filename: &str, text: &str) {
    use std::io::Write;
    use cbfr::BFRDYN;
    use std::thread::sleep;

    let mut file = std::fs::File::create(filename).unwrap();
    let mut buffer: BFRDYN = text.into();

    file.write(unsafe {buffer.bytes_mut()}).unwrap();

    println!("Writing to {filename}");
    sleep(std::time::Duration::from_millis(700));
    println!("Finish writing");
}

fn main() {
    use std::thread;

    let ins = std::time::Instant::now();
    let handle1 = thread::spawn(|| {
        send_http_request("http://crates.io");
    });
    let handle2 = thread::spawn(|| {
        write_to_file("mytext.txt", "Lorem ipsum");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let extime = ins.elapsed().as_millis();
    println!("Execution time: {extime} millis");
}

fn single_thread() {
    let ins = std::time::Instant::now();
    send_http_request("http://crates.io");
    write_to_file("mytext.txt", "Lorem ipsum");
    let extime = ins.elapsed().as_millis();
    println!("Execution time: {extime} millis");
}

