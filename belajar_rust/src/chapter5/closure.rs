pub fn sample_closure() {
    let x = 5;

    let add_two_x = |n: i32| { n + 2 + x };


    let result = add_two_x(3);
    println!("{}", result);     // output "10"
}


pub fn sample_closure2() {
    // definisi variabel v pada parent scope
    let mut v = vec![1,2,4];
    let exist; let mut add;
    {
        exist = |number| {v.contains(number)};
        let x = exist(&2);
        println!("exist: {}", x);
    }
    add = |number| {v.push(number)};
    // let mut_v = &mut v;
    add(3);
    println!("v: {:?}", v);

    let _drop = || {drop(v)};
    // println!("{:?}", v);
}

pub fn sample_move() {
    let mut v = vec![1,2,4];

    let exist = move|number| { v.contains(number)};
    println!("exist: {:?}", exist(&3));

    // println!("v: {:?}", v);
}
