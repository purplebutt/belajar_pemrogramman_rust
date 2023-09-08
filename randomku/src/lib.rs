//! Simple library untuk membuat angka acak

use std::time::{Instant, Duration};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// data structure untuk men-generate angka acak
pub struct Rng {
    ins: Instant,
    dur: Duration
}

impl Rng {
    /// Membuat instance baru Rng
    /// # Contoh
    /// ```
    /// use randomku::Rng;
    /// let mut random = Rng::new();
    /// let number = random.rnd_between(3.0,5.0);
    /// assert!(number > 2);
    /// assert!(number < 6);
    /// ```
    pub fn new() -> Self {
        Self { 
            ins: Instant::now(), 
            dur: Duration::new(0,0) 
        }
    }

    /// Membuat angka acak antara 0.0 s/d 1.0
    /// # Contoh
    /// ```
    /// use randomku::Rng;
    /// let mut random = Rng::new();
    /// let number = random.create();
    /// assert!(number < 1.1);
    /// assert!(number > -1.0);
    /// ```
    pub fn create(&mut self) -> f64 {
        let mut hasher = DefaultHasher::new();
        self.dur = self.ins.elapsed();
        self.dur.hash(&mut hasher);
        let x = hasher.finish();
        (1.0/u64::MAX as f64) * x as f64
    }
    /// Membuat angka acak antar lower dan upper
    /// Return value bertipe u128, jika anda ingin return
    /// value dengan tipe f64, gunakan [rnd_between_f64]
    /// # Contoh
    /// ```
    /// use randomku::Rng;
    /// let mut random = Rng::new();
    /// let number = random.rnd_between(3.0,5.0);
    /// assert!(number > 2);
    /// assert!(number < 6);
    /// ```
    pub fn rnd_between(&mut self, lower: f64, upper: f64) -> u128 {
        let x = self.create();
        let a_number = (x * (upper - (lower-0.5))) + lower;
        a_number as u128
    }
    /// Membuat angka acak antar lower dan upper
    /// Return value bertipe f64, jika anda ingin return
    /// value dengan tipe u128, gunakan [rnd_between]
    /// # Contoh
    /// ```
    /// use randomku::Rng;
    /// let mut random = Rng::new();
    /// let number = random.rnd_between_f64(3.0,5.0);
    /// assert!(number > 2.0);
    /// assert!(number < 6.0);
    /// ```
    pub fn rnd_between_f64(&mut self, lower: f64, upper: f64) -> f64 {
        let x = self.create();
        let a_number = (x * (upper - (lower-0.8))) + lower;
        a_number
    }
}

