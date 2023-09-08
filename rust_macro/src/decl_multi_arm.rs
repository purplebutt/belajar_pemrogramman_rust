macro_rules! multi_arm {
    () => {
        println!("This arm has no argument"); 
    };
    ($code:stmt) => {
        println!("`````start`````");
        $code
        println!("``````end``````");
    };
    ($text:literal, $times:tt) => {
        for i in 1..=$times {
            println!("{}. {}", i, $text);
        }
    };
}

macro_rules! calc {
    ($op:tt; $($number:tt),+ => $start:tt) => {
        {
            let mut result = $start;    
            $(result = result $op $number;)+
            result
        }
    };
}

pub fn demo() {
    // multi_arm!();
    // multi_arm!({
    //     println!("Calling multi_arm! with statement")
    // });
    // multi_arm!("I love coding!", 5);
    
    let total = calc!(+; 1,2,3,4,5 => 0);
    let product = calc!(*; 1,2,3,4,5 => 5);

    println!("total: {total}");
    println!("product: {product}");
}
