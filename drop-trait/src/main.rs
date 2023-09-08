use std::ops::{Deref, DerefMut};

struct Item<T> {
    value: T,
    counter: u32
}

pub struct Kotak<T> {
    item: *mut Item<T>
}

impl<T> Kotak<T> {
    pub fn new(value: T) -> Self {
        let i = Item { value, counter: 1};
        Self { 
            item: Box::into_raw(Box::new(i))
        }
    }
    pub fn clone(&self) -> Self {
        unsafe {
            (*self.item).counter += 1;
        }
        Self { item: self.item.clone() }
    }
    fn decrement(&self) {
        unsafe {
            (*self.item).counter -= 1;
        }
    } 
    /// never call this function, you have been warned!
    unsafe fn drop_item(&self) {
        let _ = Box::from_raw(self.item);
    }
}
impl<T> Deref for Kotak<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*self.item).value
        }
    }
}
impl<T> DerefMut for Kotak<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut(*self.item).value
        }
    }
}
impl<T> Drop for Kotak<T> {
    fn drop(&mut self) {
        self.decrement();
        let counter = unsafe { (*self.item).counter };
        if counter == 1 { 
            unsafe {self.drop_item()}
        }
        // println!("Dropping... counter: {}", counter);
    }
}


fn main() {
    let ori = Kotak::new(String::from("Hello"));

    let shower = ori.clone();
    let mut modifier2 = ori.clone();
    
    println!("Before change: {}", *shower);
    {
        let mut modifier1 = ori.clone();
        *modifier1 = "World".to_string();
        println!("First change: {}", *shower);
    }

    *modifier2 = "Nice".to_string();
    println!("Second change: {}", *shower);
}

