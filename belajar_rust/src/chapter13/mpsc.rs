use std::sync::mpsc;
use std::thread;
use std::time::Duration;


pub fn mpsc_demo() {
    let (sdr, rcv) = mpsc::channel();
    
    // thread 1
    let sdr1 = sdr.clone();
    let t1 = thread::spawn(move|| {
        sdr1.send("<T1>: Welcome").unwrap();
    });

    // thread 2
    let sdr2 = sdr.clone();
    let t2 = thread::spawn(move|| {
        for _i in 0..5 {
            sdr2.send("<T2>: Hello").unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    // let sdr0 = sdr.clone();
    // let _thelper = thread::Builder::new()
    //     .name("helper".into())
    //     .spawn(move|| loop { 
    //     let _r = sdr0.send("==helper=="); 
    //     thread::sleep(Duration::from_secs(1)); 
    // }).unwrap();

    // println!("Press CTRL+c to exit");
    // for i in rcv.try_iter() {
    //     if i != "==helper==" {
    //         println!("{i}");
    //         println!("\t{}", t1.is_finished());
    //         println!("\t{}", t2.is_finished());
    //     }
    //     if t1.is_finished() && t2.is_finished() { break; }
    // }

    // println!("Tekan CTRL+c untuk menutup program!");
    // for i in rcv {
    //     println!("{i}");
    // }

    loop {
        let next = rcv.try_iter().next();

        if let Some(i) = next {
            println!("{i}");
        }

        if t1.is_finished() && t2.is_finished() {
            break;
        }
        
        // mencegah program spin terlalu cepat
        thread::sleep(Duration::from_millis(200));
    }
}

