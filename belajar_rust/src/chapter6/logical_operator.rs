pub fn logical_operator() {
    let a = 3 == 3;
    let b = 3 > 3;
    let c = 3 >= 3;
    let d = 3 < 3;
    let e = 3 <= 3;
    let f = 3 != 3;

    println!("3 == 3: {}", a);      // output "true"
    println!("3 > 3: {}", b);       // output "false"
    println!("3 >= 3: {}", c);      // output "true"
    println!("3 < 3: {}", d);       // output "false"
    println!("3 <= 3: {}", e);      // output "true"
    println!("3 != 3: {}", f);      // output "false"
}

pub fn if_statement() {
    let kondisi = true;

    if kondisi {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

pub fn multi_if_statement() {
    let economic = "bad";

    if economic == "good" {
        println!("Presiden bicara");
    }
    else if economic == "bad" {
        println!("Menteri bicara");
    }
    else {
        println!("Jubir bicara");
    }
}

pub fn animal() {
    let have_wing = true;
    let color = "pink";
    let mut msg = String::from("It's a ");

    if have_wing == true {

        msg.push_str("flying ");

        if color == "pink" {
            msg.push_str("pig");
        }
        else {
            msg.push_str("bird");
        }
    }
    else {
        msg.push_str(" just animal");
    }
    println!("{}", msg);
}


#[allow(dead_code)]
enum Status {
    Kawin(String),
    Lajang
}

pub fn iflet() {
    let romeo = Status::Kawin("Juliet".to_string());

    if let Status::Kawin(istri) = romeo {
        println!("Istri Romeo: {}", istri)
    }
}