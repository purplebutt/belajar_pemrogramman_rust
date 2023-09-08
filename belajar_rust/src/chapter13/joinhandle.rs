use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

fn echo(message: &str) -> String {
    thread::sleep(Duration::from_secs(2));
    message.into()
}

pub fn test() {
    let ins = Instant::now();
    let mut threads = Vec::<JoinHandle<String>>::new();

    let t1 = thread::spawn(||{
        let result = echo("Satu");
        result
    });

    threads.push(t1);
    threads.push(thread::spawn(|| echo("Dua")));
    threads.push(thread::spawn(|| echo("Tiga")));
    threads.push(thread::spawn(|| echo("Empat")));
    threads.push(thread::spawn(|| echo("Lima")));
    
    for t in threads {
        let thread_id = t.thread().id();
        match t.join() {
            Ok(v) => println!("thread: {:?}, result: {}", thread_id, v),
            Err(e) => println!("Error: {:?}", e)
        }
    }

    // elapsed harus di tuliskan setelah thread join untuk
    // mengukur lama waktu eksekusi semua thread
    let elapsed = ins.elapsed().as_secs();
    println!("Elapsed: {} seconds", elapsed);
}

