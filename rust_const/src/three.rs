struct Human<'a> {
    name: &'a str,
    age: u32
}
impl<'a> Human<'a> {
    const fn new(name: &'a str, age: u32) -> Self {
        Self { name, age }
    }
    const fn default() -> Self {
        Self { name: "", age: 0 }
    }
    const fn setup<const LEN: usize>
        (names: [&'a str; LEN], ages: [u32; LEN]) -> [Self; LEN] {
        const HUMAN: Human = Human::default();
        let mut humans = [HUMAN; LEN];
        let mut i = 0;

        while i < LEN {
            humans[i] = Self::new(names[i], ages[i]);
            i += 1;
        }
        humans
    }
}

const fn setup<'a, const LEN: usize>
    (names: [&'a str; LEN], ages: [u32; LEN]) 
    -> [Human<'a>; LEN] {
    const H: Human = Human::default();
    let mut humans = [H; LEN];
    let mut i = 0;

    while i < LEN {
        humans[i] = Human::new(names[i], ages[i]);
        i += 1;
    }

    humans
}

#[allow(non_snake_case)]
pub fn withConst() {
    const NAMES: [&str; 5] = ["Prabo", "Joko", "Basuki", "Ganjar", "Anies"];
    const AGES: [u32; 5] = [60, 55, 49, 58, 59];

    // const HUMES: [Human; 5] = setup(NAMES, AGES);
    // for h in HUMES {
    //     println!("{} {}", h.age, h.name);
    // }

    // println!("{}", "~".repeat(30));

    const HUMES2: [Human; 5] = Human::setup(NAMES, AGES);
    for h in HUMES2 {
        println!("{} {}", h.age, h.name);
    }
}

#[allow(non_snake_case)]
pub fn withoutConst() {
    let h1 = Human::new("Prabo", 60);
    let h2 = Human::new("Joko", 55);
    let h3 = Human::new("Basuki", 49);
    let h4 = Human::new("Ganjar", 58);
    let h5 = Human::new("Anies", 59);

    let humes = vec![h1, h2, h3, h4, h5];
    
    for h in humes {
        println!("{} {}", h.name, h.age);
    }
}


