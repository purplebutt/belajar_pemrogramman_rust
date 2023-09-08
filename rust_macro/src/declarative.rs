#[macro_export]
macro_rules! whoami {
    () => {
        println!("Your name is \"Cheries\"");
    };
    ($looptype:ident) => {
        $looptype _ in 0..3 {
            println!("[ident] \"{}\"", stringify!($looptype));
        }
    };
    ($name:ty) => {
        let x: $name = "Wow";
        println!("The type you entered is: {}, and value is: {}", stringify!($name), x);
    };
    ($name:tt) => {
        println!("[tt]Your name is \"{}\"", $name);
    };
    ($name:literal) => {
        println!("[literal]Your name is \"{}\"", $name);
    };
    ($name:expr) => {
        println!("[expr]Your name is \"{}\"", $name);
    };
}

#[macro_export]
macro_rules! with_arg {
    ($arg:block) => {
        println!("[$arg:block] = \"{}\"", stringify!($arg));
        $arg;
    };
    ($arg:expr) => {
        println!("[$arg:expr] = \"{}\"", $arg);
    };
    ($arg:stmt) => {
        $arg    // execute statement
        println!("[$arg:stmt] = end of program");
    };
    ($arg:ident) => {
        let mut i = 2;
        $arg i < 5  {
            println!("[{}], i: {i}", stringify!($arg));
            i += 1;
        }
        // println!("{}", stringify!($arg));
    };
    ($name:tt) => {
        let a = 3;
        let b = 2;
        let result = a $name b;
        println!("[tt]result is \"{}\"", result);
    };
    ($name:ty) => {
        let x: $name = "Wow";
        println!("[ty]The type you entered is: {}, and value is: {}", stringify!($name), x);
    };
    ($arg:tt) => {
        if 5 $arg 6 {
            println!("[$arg:pat] (true) 5 {} 6", stringify!($arg));
        }
        else {
            println!("[$arg:pat] (false) 5 {} 6", stringify!($arg));
        }
    };
    ($name:literal) => {
        { 
            println!("[literal] - \"{}\"", $name);
            1 
        }
    };
}

macro_rules! create_print {
    ($name:ident) => {
        fn $name() {
            println!("Hello from {}", stringify!($name));
        }
    };
}

macro_rules! author {
    () => {
        {
            "someone@email.com"
        } 
    };
}

macro_rules! greet {
    () => {
        {
            "Hello ".to_string()
        }
    };
    ($name:expr) => {
        {
            let mut result = String::new();
            result.push_str("Hello user");
            result.push_str($name);
            {
                result
            }
        } 
    };
}

macro_rules! separator {
    () => {
        println!("{}", "-".repeat(40)); 
    };
    ($type:expr) => {
        println!("{}", $type.repeat(40)); 
    };
    ($type:expr, $n:expr) => {
        println!("{}", $type.repeat($n)); 
    };
}

macro_rules! multi_arg {
    ($len:tt, $op:tt $start:literal) => {
        {
            let mut result = $start;
            for i in 1..=$len {
                result = result $op i;
            }
            result
        } 
    };
}

macro_rules! test {
    ($($names:expr),+) => {
        $(print!("{}", $names);)+
    };
    ($space:expr, $($names:expr),+ => $op:expr, $cl:expr) => {
        let mut names = vec![];
        $(names.push($names);)+
        //
        print!("{}", $op);
        for (idx, name) in names.iter().enumerate() {
            if idx < (names.len()-1) {
                print!("{}{}", name, $space);
            }
            else {
                print!("{}", name);
            }
        }
        // $(print!("{}{}", $names, $space);)+
        println!("{}", $cl);
    };
}




// struct Dog;
// fn afunc() {}

pub fn demo() {
    let result1 = multi_arg!(3, *10);
    println!("{result1}");

    let result2 = multi_arg!(3, +10);
    println!("{result2}");

    // with_arg!(24_f32);
    // with_arg!('c');
    // with_arg!("\u{ba34}");
    // with_arg!(r#"raw \"string""#);
    // with_arg!(0b0101);

    // with_arg!(*);

    // with_arg!(while);
    // with_arg!(if);
    // with_arg!(Dog);
    // with_arg!(afunc);

    // with_arg!({ "hello "}); // error
    // with_arg!({
    //     println!("I am rust statement");
    //     println!("so dont expect me to return any value.");
    // });


    // with_arg!(>);
    // with_arg!({ println!("------")});
    // with_arg!("Hello");
    // with_arg!(10);
    // with_arg!({ 10 * 2 });

    // let expression = {
    //     "This is a rust expression"
    // };
    // with_arg!(expression);

}


#[allow(unused)]
pub fn demo2() {
    whoami!(&str);
    whoami!(for);
    whoami!("gibran");
    whoami!({ "return value" });
    create_print!(salam);
    salam();
    //
    println!("{}", author!());
    //
    separator!();
    test!("Ani", "Budi", "Susi", "Kevin");
    separator!();
    test!(" ", "Ani", "Budi", "Susi", "Kevin" => "(", ")");
    separator!();
    //
    let g = greet!("Fika");
    println!("{g}");
    println!("{}", greet!());
}



