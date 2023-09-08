use std::thread;
use std::time::{Duration, Instant};


fn show_threadid(value: &str) {
    let tid = thread::current().id();
    println!("ID: {:?} - {}", tid, value);
}

fn show_message(message: &str) -> bool {
    println!("===spawn thread start===");
    thread::sleep(Duration::from_secs(2));
    show_threadid(message);
    println!("===spawn thread end===");
    true
}

pub fn scoped_thread() {
    println!("===main thread start===");
    let message = String::from("Hello World");
    // let mut result = Ok(false);

    let _ts = thread::scope(|s| {
        // let r = s.spawn(|| show_message(&message));
        // result = r.join();
        s.spawn(|| show_message(&message));
    });

    // if let Ok(value) = result {
    //     show_threadid(&value.to_string());
    // }

    // variabel message masih di gunakan di sini
    show_threadid(&message);
    println!("===main thread end===");
}


fn show(message: &str) {
    // println!("{}", message);
    let tid = thread::current().id();
    thread::sleep(Duration::from_secs(2));
    println!("{:?} - {}", tid, message);
}

pub fn unscoped_thread() {
    let message = String::from("Hello World");
    // let message = "Static text";
    // let arsi = std::rc::Rc::new("wow".to_string());

    let t = thread::spawn(move||{
        show(&message)
    });
    // let t2 = thread::spawn(||show(&arsi));

    let _result = t.join();

    // variabel message masih di gunakan di sini
    // let tid = thread::current().id();
    // println!("{:?} - {}", tid, message);
}

pub fn scoped_demo() {
    let message = String::from("Hello World");
    let ins = Instant::now();

    thread::scope(|s| {
        s.spawn(|| show(&message));
    });

    let elapsed = ins.elapsed().as_secs();

    // variabel message masih di gunakan di sini
    let tid = thread::current().id();
    println!("{:?} - {}", tid, message);

    println!("Waktu eksekusi: {} detik", elapsed);
}

static I_AM_STATIC: &str = "Hello";
pub fn lifetime_static() {
    // string literal memiliki lifetime 'static
    let static_message = "Hello World";

    let t1 = thread::spawn(|| show(static_message));
    let t2 = thread::spawn(|| show(I_AM_STATIC));

    let _result1 = t1.join();
    let _result2 = t2.join();
}

pub fn single_thread() {
    let message = String::from("Hello World");
    show(&message);

    // variabel message masih di gunakan di sini
    println!("{}", message);
}
