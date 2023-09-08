
fn float() {
    let a = 23.5;       // f64
    let b = 23.5f32;    // f32
    let c: f32 = 23.5;       // f32
    let d = 1.75e8;     // f64
    let e = 1.75e8f32;  // f32
    let f: f32 = 1.75e8;     // f32


    let x = a + b as f64;
    let y = a + d;
}
