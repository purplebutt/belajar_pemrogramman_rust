// fungsi dengan argument immutable reference
fn imut_ref(arg: &String) -> String {
    println!("{}", arg);    
    arg.to_string()
}

// fungsi dengan argument mutable reference
fn mut_ref(arg: &mut String) -> String {
    *arg = "By Ref".to_string();
    println!("{}", arg);
    arg.to_string()
}

// fungsi dengan argument take ownership
fn by_value(mut arg: String) -> String {
    arg = "By Value".to_string();
    println!("{}", arg);
    arg
}

pub fn closure_demo() {
    let text1 = "Red".to_string();
    let _text2 = "Green".to_string();
    let text3 = "Blue".to_string();

    // closure secara otomatis mengimplementasikan trait Fn
    let c_Fn = || imut_ref(&text1);
    // let mut c_FnMut = || { mut_ref(&mut text2) };
    let c_FnOnce = || by_value(text3);

    let closure_result = c_Fn();
    println!("{}", closure_result);
    
    let mut text = "Cool".to_string();
    exec_FnOnce(c_FnOnce);
    exec_FnMut(|| { 
        text.push_str(" and Fresh"); text.clone() 
    } );
    exec_Fn(c_Fn);

}

pub fn caller() {
    let mut text = "Cool".to_string();
    
    let c_fn_once = || "nice".to_string();
    let c_fn_mut = || {
        text.push_str(" and Fresh"); 
        text.clone() 
    };

    // exec_FnOnce(c_fn_once);
    // exec_FnMut(c_fn_mut);
    // exec_Fn(|| "cool".to_string());

    exec_FnOnce(c_fn_mut);
    // exec_Fn(c_fn_mut);
}

fn exec_FnOnce(closure: impl FnOnce() -> String) {
    let return_value = closure();
    println!("{}", return_value);
}

fn exec_FnMut(mut closure: impl FnMut() -> String) {
    let return_value = closure();
    println!("{}", return_value);
}

fn exec_Fn(closure: impl Fn() -> String) {
    let return_value = closure();
    println!("{}", return_value);
}

pub fn closure_demo2() {
    let mut msg = "Hello".to_string();
    exec_Fn_with_arg(&mut msg, |m| { m.to_uppercase() })
}

fn exec_Fn_with_arg(text: &mut String, closure: impl Fn(&mut String) -> String) {
    let return_value = closure(text);
    println!("{}", return_value);
}

