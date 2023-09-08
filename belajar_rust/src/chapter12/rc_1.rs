use std::rc::Rc;

pub fn not_okay() {
    // let mut onumber;
    // {
    //     let inumber = Box::new(18);
    //     onumber = &inumber;
    //     println!("{}", inumber)
    // }

    // println!("{}", onumber)
}

pub fn okay() {
    let onumber;
    {
        let inumber = Rc::new(18);
        onumber = inumber.clone();
        println!("{}", inumber)
    }

    println!("{}", onumber)
}
