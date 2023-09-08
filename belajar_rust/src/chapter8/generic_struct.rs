pub struct User<T> {
    account: T,
    password: T
}

pub fn test_user() {
    let user1 = User {account: "mySecretAccount", password: "firefly123"};
    let user2 = User {account: 0xF3930AD, password: 127403829};

    println!("User1: {} {}", user1.account, user1.password);
    println!("User2: {} {}", user2.account, user2.password);
}

pub fn test_user2() {
    let user_a = User {account: 123, password: 12345};
    let user_b = User {account: 543, password: 54321};

    let user_x: User<User<i32>> = User { account: user_a, password: user_b };

    println!("{} {}", user_x.account.account, user_x.password.password);
}

// tidak bisa di compile
// pub fn test_user3() {
//     // account bertipe &str sedangkan password bertipe i32
//     let user= User {account: "myAccount", password: 48320};

//     println!("{} {}", user.account, user.password);
// }

pub struct User2<T,U> {
    account: T,
    password: U
}

pub fn test_user4() {
    // account bertipe &str sedangkan password bertipe i32
    let user_a: User2<&str, i32> = User2 {account: "accountA", password: 48320};

    // account dan password bertipe String
    let user_b: User2<String, String> = User2 {
        account: "accountB".to_string(), 
        password: "48321".to_string()
    };

    println!("{} {}", user_a.account, user_a.password);
    println!("{} {}", user_b.account, user_b.password);
}