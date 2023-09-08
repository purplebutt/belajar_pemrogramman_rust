pub fn variabel_lifetime() {
    let a = String::from("Hello ");
    {
        let b = String::from("World");
    }                       // akhir lifetime variabel b
    // println!("", b)      // error, lifetime variabel b sudah berakhir
    println!("{}", a);      // valid, lifetime variabel a belum berakhir
}                           // akhir lifetime variabel a

pub fn variabel_lifetime2() {
    let ref_text= "";
    {
        let text = String::from("World"); 
        // ref_text = &text;
    }                           // lifetime text berakhir di sini
    println!("{}", ref_text);   // error, lifetime text sudah berakhir
}

pub fn a_str_ref() -> &'static str {
    let a = String::from("A message");
    //&a
    ""
}

pub fn lifetime_in_if_block() {
    let mut j = String::from("Jakarta");
    let r = &j;

    if j.len() > 10 {
        j = "Bali".to_string();
    }
    else {
        println!("{}", r);
    }
    //println!("{}", r);      // kode ini akan menghasilkan error
}

pub fn lifetime_in_match_block() {
    let mut j = String::from("Jakarta");
    let r = &mut j;

    let b = true;

    match b {
        true => {
            j = "Bali".to_string();
            // println!("{}", r);   // kode ini akan meghasilkan error
        },
        false => {
            println!("{}", r);
        }
    }
    //println!("{}", r);            // kode ini akan menghasilkan error
}

// pub fn lifetime_demo()  {
//     let x;            // 'a |
//     {                       //    |
//         let y = 23;    //    |     // 'b |
//         x = &y;             //    |     //    |
//     } // ___________________//____|___________|
//     println!("{}", x);      //    |
// } // _______________________//____|

use std::collections::HashMap;


pub fn lifetime_demo_fixed()  {
    let x;            // 'a |
    {                       //    |
        let y = 23;    //    |     // 'b |
        x = &y;             //    |     //    |
        println!("{}", x);  //    |     //    |
    } // ___________________//____|___________|
}


fn return_arg(arg: &str) -> &str {
    println!("{}", arg);
    arg
}

pub fn lifetime_with_function() {
    let name = "Mega";
    let n = return_arg(name);

    println!("Returned: {}", n);
}

// fn get_shorter(name1: &str, name2: &str) -> &str {
//     if name1.len() < name2.len() {
//         name1
//     }
//     else {
//         name2
//     }
// }

fn get_shorter<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    if name1.len() < name2.len() {
        name1
    }
    else {
        name2
    }
}

pub fn call_get_shorter() {
    let name1 = "Ganjar";   // len = 6
    let name2 = "Anies";    // len = 5

    let shorter = get_shorter(name1, name2);

    println!("Shorter name is: {}", shorter);
}


pub struct User<'a, 'b> {
    // name: String
    name: &'a str,
    country: &'b dyn std::fmt::Display
}

pub fn user_demo() {
    // let user1 = User { name: "Fika".to_string() };

    // println!("{}", user1.name)

    let user1 = User { name: "Fika", country: &"Indonesia" };

    println!("Name: {}, Country: {}", user1.name, user1.country);
}

