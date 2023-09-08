const ANUMBER: i32 = 20;
const BNUMBER: i32 = 4 + ANUMBER;
const ATEXT: &str = "She come to me and say that...";
const AARRAY: [u32; 5] = [1,2,3,4,5];

const MYTUP: (i32, &str) = (23, "Nice");
struct MyTupStruct<'a>(&'a str);
const MTS: MyTupStruct = MyTupStruct("Cool");


#[allow(non_snake_case)]
pub fn demoOne() {
    println!("{ANUMBER}");
    println!("{BNUMBER}");
    println!("{ATEXT}");
    println!("{:?}",AARRAY);

    println!("{:?}", MYTUP);
    println!("{:?}", MTS.0);

    const SCOPED: f32 = 3.14;
    println!("{SCOPED}");
}

