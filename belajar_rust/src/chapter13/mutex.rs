use std::{sync::{Mutex, Arc}, thread, ops::DerefMut};


fn doubleit(n: &mut i32) -> i32 {
    *n *= 2;
    *n
}

pub fn mutex_demo() {
    let number = Mutex::new(4); 
    println!("number before update: {}", number.lock().unwrap());

   let box_number = Arc::new(number);

   let box_number1 = box_number.clone();
   let t1 = thread::spawn(move|| {
       let l = box_number1.lock();
       if let Ok(mut mguard) = l {
           // add_one(mguard.deref_mut())
           doubleit(&mut mguard)
       }
       else { panic!("something went wrong") }
   });

   let box_number2 = box_number.clone();
   let t2 = thread::spawn(move|| {
       let l = box_number2.lock();
       if let Ok(mut mguard) = l {
           doubleit(mguard.deref_mut())
           // doubleit(&mut mguard)
       }
       else { panic!("something went wrong") }
   });

   let result1 = t1.join();
   let result2 = t2.join();

   if let Ok(r1) = result1 { println!("Result 1: {}", r1); }
   if let Ok(r2) = result2 { println!("Result 2: {}", r2); }
}

