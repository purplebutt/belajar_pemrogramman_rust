enum Science { Math, Computer }

struct Business {
    class: String,
    year: u32,
}

enum Major { Art, Science(Science), Business(Business) }

struct Name {
    first: String,
    last: String
}

struct Student {
    id: u32,
    name: Name,
    class: String,
    major: Major
}
impl Student {
    pub fn new(id: u32, name: Name, class: &str, major: Major) -> Self {
        Self { id , name, class: class.to_string(), major }
    }
}

pub fn pattern_matching_demo() {
    let name = Name {
        first: "Balmond".to_string(),
        last: "Widodo".to_string()
    };
    let major = Business { class: "33B".into(), year: 4 };
    let balmon = Student::new(123, name, "3C", Major::Business(major));

    match balmon.major {
        Major::Art => println!("Art"),
        Major::Science(s) => {
            match s {
                Science::Math => println!("Math"),
                Science::Computer => println!("Computer Science")
            }
        },
        Major::Business(
            Business { class, year }
        ) => {
            println!("Business");
            println!("class: {class}, year: {year}");
        }
    }
    
    let Name { first, last } = balmon.name;
    println!("first_name: {}, last_name: {}", first, last);
}


pub fn tuple_matching() {
    let t = ("Satu", 2, 3.0, "Empat".to_string());

    let satu = t.0;
    let dua = t.1;
    let tiga = t.2;
    let empat = t.3.clone();
    
    println!("satu: {satu},  dua: {dua}, tiga: {tiga}, empat: {empat}");

    let (satu, _, tiga, _) = t;
    
    println!("satu: {satu}, tiga: {tiga}");

    let (first, ..) = t;
    let (.., last) = t;
    let (_, second, ..) = t;
    
    println!("first: {first}, second: {second}, last: {last}");
}

fn matcher(number: u32) {
    match number {
        0 | 1 => println!("Binary: {number}"),
        n @ 2..=10 if (n%2==0) => {
            println!("Smaller or equal 10 and even")
        },
        2..=10 => println!("Smaller or equal 10 and odd"),
        _ => println!("Greater than 10")
    }
}

pub fn number_matching() {
    let numbers = vec![1,2,3];

    for n in numbers {
        matcher(n)
    }
}

pub fn tuple_matching2() {
    let t = ("Satu", 2, 3.0, "Empat".to_string());

    match t {
        (_, n, ..) if (n % 2 == 0) => println!("Bil. genap: {}", n),
        (.., mut text) => {
            text.push_str(" Lagi");
            println!("{text}");
        }
    }
}

