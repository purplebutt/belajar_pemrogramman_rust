use std::any::{Any, TypeId};

fn dynamic(anytype: &dyn Any) {
    if anytype.type_id() == TypeId::of::<i32>() {
        let val = anytype.downcast_ref::<i32>().unwrap();
        let num = 10_i32;
        let result = val + num;
        println!("i32 => {}", result);
    }
    else if anytype.type_id() == TypeId::of::<String>() {
        let val = anytype.downcast_ref::<String>().unwrap();
        println!("String => {}", val);
    }
    else if anytype.type_id() == TypeId::of::<Vec<i32>>() {
        let val = anytype.downcast_ref::<Vec<i32>>().unwrap();
        for (i, v) in val.iter().enumerate() {
            println!("{}.\t{}", i, v);
        }
    }
    else {
        println!("Invalid type: {:?}", anytype.type_id());
    }
}

fn dynamicmut(anytype: &mut dyn Any) {
    if let Some(ty) = anytype.downcast_mut::<i32>() {
        println!("i32 => {}", ty);
    }
    else if let Some(ty) = anytype.downcast_mut::<String>() {
        ty.push_str(" world!.");
        println!("String => {}", ty);
    }
    else if let Some(ty) = anytype.downcast_mut::<Vec<i32>>() {
        for (i, v) in ty.iter_mut().enumerate() {
            *v *= 2;
            println!("{}\t{}", i, v);
        }
    }
}

#[allow(non_snake_case)] // ignore non_snake_case error
fn withsmartpointer(anytype: Box<dyn Any>) {
    // definisi variabel dengan camelcase
    let boxId = anytype.type_id();
    let tyId = (*anytype).type_id();
    
    assert_eq!(TypeId::of::<Box<dyn Any>>(), boxId);
    assert_eq!(TypeId::of::<String>(), tyId);

    if anytype.is::<String>() {
        println!("It's String");
    } else {
        println!("It's NOT String");
    }
}

fn main() {
    let mut num = 3_i32;
    let mut txt = "Hi";
    let mut numbers = vec![1,2,3,4,5];
    let mut note = String::from("Hello");

    dynamic(&num);
    dynamic(&txt);
    dynamic(&numbers);
    dynamic(&note);

    println!("{}", "~".repeat(40));

    dynamicmut(&mut num);
    dynamicmut(&mut txt);
    dynamicmut(&mut numbers);
    dynamicmut(&mut note);

    println!("{}", "~".repeat(40));

    withsmartpointer(Box::new(note));
}

