fn print<T>(data: T) 
    where T: std::fmt::Display
{
    // data pasti memiliki fungsi to_string karena data 
    // terikat dengan trait std::fmt::Display
    let some_text: String = data.to_string();
    println!("item {}", some_text);
}

fn main_demo() {
    let s = String::from("Apa kabar..?");
    let t = 234;
    let u = "I love you";

    print(s);
    print(t);
    print(u);
}


// saat kompilasi rust membuat salinan kode program generic
// dengan concrete type nya masing-masing seperti berikut:
fn print_string(data: String) {
    let some_text: String = data.to_string();
    println!("item {}", some_text);
}

fn print_i32(data: i32) {
    let some_text: String = data.to_string();
    println!("item {}", some_text);
}
fn print_str(data: &str) {
    let some_text: String = data.to_string();
    println!("item {}", some_text);
}
