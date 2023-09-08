
fn get_value() -> Option<String> {
    Some("Hello".to_string())
}

fn call_get_value() -> Result<String, i8> {
    get_value().ok_or(-1)
    // get_value().ok_or_else(|| -1)
}

fn call_call_get_value() -> Option<String> {

    // jika return value fungsi call_get_value = Err<T> maka
    // ok() akan mengembalikan nilai None.
    // Jika return value fungsi call_get_value = Ok<T> maka
    // ok() akan mengembalikan nilai Some(T)
    call_get_value().ok()

    // if call_get_value().is_ok() {
    //     Some("ok".to_string())
    // }
    // else {
    //     None
    // }
}

pub fn demo_option_result() {
    let result = call_get_value();
    match result {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{}", err)
    }

    let some_value = call_call_get_value();
    match some_value {
        Some(value) => println!("{}", value),
        None => println!("No value")
    }
}