use std::{thread, time::Duration, sync::{Mutex, Arc}};

fn add_text(text: &mut String, new: String) {
    text.push_str(&new);
}

pub fn poison_demo() {
    let text = "Some text".to_string();
    let mx = Mutex::new(text);
    let atomic_rc0 = Arc::new(mx);

    // thread 1
    let atomic_rc1 = atomic_rc0.clone();
    let t1 = thread::spawn(move|| {
        let guard =  atomic_rc1.lock();
        thread::sleep(Duration::from_secs(2));
        let mut text = guard.unwrap();
        let tid = thread::current().id();
        add_text(&mut text, format!(" ={:?}= ", tid) );
        panic!("Intentional panic!");
        ()
    });

    // thread 2
    let atomic_rc2 = atomic_rc0.clone();
    let t2 = thread::spawn(move|| {
        thread::sleep(Duration::from_secs(1));
        let tid = thread::current().id();
        let lock_result =  atomic_rc2.lock();
        match lock_result {
            Ok(mut guard) => {
                println!("==NOT POISONED==");
                add_text(&mut guard, format!(" ={:?}= ", tid) );
            },
            Err(poisoned) => {
                println!("==POISONED!!!==");
                let mut guard = poisoned.into_inner();
                add_text(&mut guard, format!(" ={:?}= ", tid) );
            }
        }
    });

    let _t1result = t1.join();
    let _t2result = t2.join();

    let lock_result = atomic_rc0.lock();
    match lock_result {
        Ok(guard) => {
            println!("After mut (not poisoned): {}", guard)
        },
        Err(poisoned) => {
            println!("After mut (poisoned): {}", poisoned.into_inner())
        }
    }
}

