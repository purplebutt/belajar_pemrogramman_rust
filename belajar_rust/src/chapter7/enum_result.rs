fn divide_2_number(na: i32, nb: i32) -> Result<i32, String> {
    if nb < 1 {
        let err_msg = format!("Expected value > 0, but you provided {}", nb);
        Err(err_msg)
    }
    else {
        let result = na / nb;
        Ok(result)
    }
}

pub fn enum_result_demo() {
    let number_a = 20;
    let number_b = -4;

    let result = divide_2_number(number_a, number_b);

    match result {
        Ok(value) => println!("{}/{}={}", number_a, number_b, value),
        Err(err) => {
            // println!("{}", err)
            panic!("{}", err)
        }
    }
}
