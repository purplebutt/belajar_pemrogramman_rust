pub struct User2<T,U> { 
    account: T,
    password: U
}

impl<T, U> User2<T, U> 
where 
    T: std::fmt::Display, 
    U: std::fmt::Display,
{
    pub fn new(account: T, password: U) -> Self {
        Self { account, password }
    }

    pub fn show_x_times<Z>(&self, mut x: Z) 
    where 
        Z: PartialOrd<u32> + 
        std::ops::SubAssign<u32>
    {
        while x > 0 {
            println!("{} {}", self.account, self.password);
            x -= 1;
        }
    }
}


pub fn test_user2() {
    let user = User2::new("damnI", "LoveIndonesia");

    user.show_x_times(5);
}
