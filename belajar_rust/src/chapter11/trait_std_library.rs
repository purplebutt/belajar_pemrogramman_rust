use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::{Add, Mul, Sub, Div};


#[derive(Debug, Clone, PartialEq)]
pub enum FruitTaste {
    Sweet,
    Sour,
    Other
}

#[derive(Debug, PartialEq)]
pub enum FruitMedia {
    VideoSMALL([u8;128]),        // 2^14  >    16,384 bytes / 16 KB
    VideoBIG([u8;1_024]),       // 2^20  > 1,048,576 bytes /  1 MB
    None
}

//impl Clone for FruitTaste {
    //fn clone(&self) -> Self {
        //match self {
            //Self::Sweet => Self::Sweet,
            //Self::Sour => Self::Sour,
            //Self::Other => Self::Other
        //}
    //}
//}

pub struct Fruit{
    name: String,
    price: f64,
    taste: FruitTaste,
    media: FruitMedia,
    iter: FruitIter
}

impl Fruit {
    pub fn new(name: &str, price: f64) -> Self {
        Self { 
            name: name.into(), 
            price, 
            taste: FruitTaste::Other, 
            media: FruitMedia::None,
            iter: FruitIter::new(price, 1.0) 
        }
    }
}

impl Display for Fruit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, 
            "Fruit (name: {}, price: {}, taste: {:?})", 
            self.name, self.price, self.taste
        )
    }
}

impl Clone for Fruit {
    fn clone(&self) -> Self {
        Self { 
            name: self.name.to_string(), 
            price: self.price, 
            taste: self.taste.clone(), 
            media: FruitMedia::None,    // media tidak di clone
            iter: self.iter.clone() 
        }
    }
}


impl From<(&str, i32)> for Fruit {
    fn from(value: (&str, i32)) -> Self {
        Self { 
            name: value.0.into(), 
            price: value.1 as f64, 
            taste: FruitTaste::Other, 
            media: FruitMedia::None,
            iter: FruitIter::new(value.1 as f64, 10.0) 
        }
    } 
}

impl Into<String> for Fruit {
    fn into(self) -> String {
        self.to_string()
    }     
}

impl Mul<i32> for Fruit {
    type Output = f64;
    fn mul(self, rhs: i32) -> Self::Output {
        self.price * rhs as f64
    }    
}

impl Mul<Fruit> for Fruit {
    type Output = f64;
    fn mul(self, rhs: Fruit) -> Self::Output {
        self.price * rhs.price
    }
}

impl Add<Self> for Fruit {  // sama dengan "impl Add<Fruit> for Fruit"
    type Output = f64;
    fn add(self, rhs: Self) -> Self::Output {
        self.price.add(rhs.price)   // sama dengan "self.price + rhs.price"
    }
}

impl Sub<i32> for Fruit {
    type Output = f64;
    fn sub(self, rhs: i32) -> Self::Output {
        self.price - rhs as f64     // sama dengan "self.price.sub(rhs as f64)"
    }
}

impl Div<f64> for Fruit {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        self.price.div(rhs)     // sama dengan self.price / rhs
    }
}

trait KaliRef { fn kali(&self, n: f64) -> f64; }

impl KaliRef for Fruit {
    fn kali(&self, n: f64) -> f64 {
        self.price * n
    } 
}

// trait Clone di butuhkan karena
// struct Fruit memiliki implementasi trait Clone
#[derive(Clone)]
struct FruitIter {
    current: f64, decval: f64
}

impl FruitIter {
    pub fn new(current: f64, decval: f64) -> Self {
        Self { current, decval }
    }
    pub fn modify(mut self, value: f64) -> Self {
        self.decval = self.current/value;
        self
    }
}

impl Iterator for FruitIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.decval || self.decval < 1.0 { 
            self.current = 0.0;
            None
        }
        else { 
            self.current -= self.decval;
            Some(self.current)
        }
    }
}

impl Iterator for Fruit {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        // tidak perlu di implementasikan lagi
        // cukup dengan memanggil metode next() pada field iter
        self.iter.next() 
    }
}


pub fn test_clone() {
    let mut apple = Fruit::new("Apple", 4500.0);
    apple.media = FruitMedia::VideoBIG([0;1024]);

    let apple_clone = apple.clone();

    assert_eq!(apple.name, apple_clone.name);       // equal
    assert_eq!(apple.price, apple_clone.price);     // equal
    assert_eq!(apple.taste, apple_clone.taste);     // equal
    assert_ne!(apple.media, apple_clone.media);     // not equal

    let ukuran_apple = std::mem::size_of_val(&apple.media);
    let ukuran_apple_clone = std::mem::size_of_val(&apple_clone.media);

    println!("Ukuran apple: {}", ukuran_apple);
    println!("Ukuran apple cloned: {}", ukuran_apple_clone);
    
}

pub fn test() {
    // semua fungsi pada fungsi test() berikut membutuhkan trait
    // jika struct Fruit tidak memiliki implementasi trait yang
    // di butuhkan, maka kode program tidak akan bisa di compile

    let apple = Fruit::new("Apple", 5000.0);
    let manggo = Fruit::new("Manggo", 3500.0);

    let kali = apple.clone() * manggo.clone();
    let bagi = apple.clone() / 10.0;
    let kurang = apple.clone() - 1000;
    let tambah = apple.clone() + manggo.clone();

    println!("apple kali manggo = {}", kali);
    println!("apple bagi 10.0 = {}", bagi);
    println!("apple kurang 1000 = {}", kurang);
    println!("apple tambah manggo = {}", tambah);

    println!("{}", apple);                           // require trait Display
    let apple_text = apple.to_string();      // require trait Display
    let apple_clone = apple.clone();          // require trait Clone
    let apple_and_manggo = 
        apple.clone() + manggo.clone();              // require trait Add<Self>
    let apple_mul_23 = apple.clone() * 23;      // require trait Mul<i32>
    let apple_mul_110 = apple.clone().mul(110); // require trait Mul<i32>
    let apple_sub_100 = apple - 100;            // require trait Sub<i32>


    let tup_banana = ("Banana", 3500);     // define a tuple (&str, i32)
    let banana: Fruit = tup_banana.into();            // require trait From<(&str, i32)>
    let str_banana: String = banana.into();             // require trait Into<String>


    let banana = Fruit::new("Banana", 5000.0);

    // for loop require Iterator trait
    // for (i, v) in banana.iter.modify(50.0).enumerate() {
    for b in banana {
        // println!("loop {}: {}", i, v)
        println!("loop {}", b)
    }
}
