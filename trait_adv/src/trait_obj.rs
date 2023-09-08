// definisi variabel const juga tidak object safe
const VAL: i32 = 23;

pub trait Support {
    fn safe1(&self);             // ini contoh fungsi yang valid
    fn safe2(&self) -> String;   // ini contoh fungsi yang valid

    // berikut contoh fungsi-fungsi yang
    // membuat trait tidak object safe
    fn notsafe1<T>(&self) where Self: Sized; 
    fn notsafe2(&self) -> Self where Self: Sized; 
    fn notsafe3(self) where Self: Sized; 
}

