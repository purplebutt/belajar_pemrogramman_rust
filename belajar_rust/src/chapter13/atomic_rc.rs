use std::thread;
use std::sync::Arc;
use std::time::Duration;


fn show(message: &str) {
    println!("===spawn thread start====");
    thread::sleep(Duration::from_secs(1));
    let tid = thread::current().id();
    println!("{:?} - {}", tid, message);
    println!("===spawn thread end====");
}

pub fn atomic_rc_demo() {
    println!("===main thread start====");
    let arc = Arc::new("wow".to_string());

    let arc1 = arc.clone();
    let t = thread::spawn(move||{
        show(&arc1)
    });

    let _result = t.join();

    let tid = thread::current().id();
    println!("{:?} - {}", tid, arc);
    println!("===main thread end====");
}

// pub fn rc_demo() {
//     let rc = std::rc::Rc::new("wow".to_string());
// 
//     let rc1 = rc.clone();
//     let t = thread::spawn(move||{
//         show(&rc1)
//     });
// 
//     let _result = t.join();
// 
//     let tid = thread::current().id();
//     println!("{:?} - {}", tid, rc);
// }
