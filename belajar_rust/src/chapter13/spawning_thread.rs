use std::thread;
use std::time::Duration;
use std::time::Instant;

fn download(_url: &str) -> String {
    thread::sleep(Duration::from_secs(2));
    "{'message': 'hello'}".into()
}

pub fn instant_demo() {
    let ins = Instant::now();
 
    let d1 = download("www.example.com/hello.json");
    let d2 = download("www.example.com/hello.json");
    
    let elapsed = ins.elapsed().as_secs();

    println!("Download1: {}", d1);
    println!("Download2: {}", d2);
    println!("Elapsed: {} micro seconds", elapsed);
}

pub fn test() {
    thread::spawn(||{
        download("www.example.com/hello.json");
    });
    
    thread::sleep(Duration::from_secs(3));
}

