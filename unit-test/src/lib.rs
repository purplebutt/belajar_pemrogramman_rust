//! Simple library untuk melakukan basic operasi 
//! matematika. Library ini menyediakan fungsi 
//! untuk melakukan [addition], [add_one], [multiply], 
//! dan juga fungsi [show] untuk menampilkan pesan.
//!
//! -- [tutorial video]: <http://youtube.com/simpe_calc/>

/// Menjumlahkan dua angka bertipe u32
/// fungsi ini mengembalikan nilai bertipe u32
/// # example
/// ```
/// use unit_test::addition;
/// let result = addition(3, 2);
/// assert!(result == 5);
/// 
/// ```
pub fn addition(n1: u32, n2: u32) -> u32 {
    n1+n2
}

/// Menampilkan message pada stdout terminal
/// # panic
/// Fungsi ini akan panic jika "message" = ""
pub fn show(message: &str) {
    if message == "" {
        panic!("Message can not be empty!");
    } else {
        println!("{}", message)
    }
}

/// # Header di awali dengan hashtag
/// ~~strikethrough~~ di apit dengan double tilde
/// ## Contoh table
/// |No.|Nama         |
/// |---|-------------|
/// |1  |Anies        |
/// |2  |Ganjar       |
/// |3  |Prabowo      |
/// ## Contoh checkbox
/// - [x] yes
/// - [ ] no
pub fn add_one(number: u32) -> u32 {
    number+1
}

#[doc(hidden)]
/// A secret note. Tidak boleh di ekspose
pub fn multiply(n1: u32, n2: u32) -> u32 {
    n1*n2
}


#[cfg(test)]
mod basic_test {
    use crate::*;

    #[test]
    fn addition_test() {
        let five = addition(3, 2);
        assert_eq!(5, five);
    }
    #[test]
    fn add_one_test() {
        let four = add_one(3);
        assert_eq!(4, four);
    }

    #[test]
    fn multiply_test() {
        let num1: u32 = 3;
        let num2: u32 = 2;

        assert_ne!(0, num1);
        assert!(num2 > 0);

        let result = multiply(num1, num2);

        assert_eq!(6, result);
    }
}

#[cfg(test)]
mod show_test {
    use crate::show;

    #[test]
    #[should_panic]
    fn show_test_panic() {
        show("");
    }

    #[test]
    fn show_test() {
        show("Hello");
    }
}

