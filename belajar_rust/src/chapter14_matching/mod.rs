pub mod patt_match;

enum Art { Painting, Music, Acting }

enum Major {
    Art(Art),
    Science,
    Other {
        id: u32,
        name: String
    }
}

pub fn struct_extract_enum() {
    let art = Art::Music;
    let major1 = Major::Art(art);

    let major2 = Major::Other { 
        id: 123, name: "Sociology".to_string()
    };

    // match major1 {
    //     Major::Art(art) => {
    //         match art {
    //             Art::Music => println!("Music"),
    //             Art::Painting => println!("Painting"),
    //             Art::Acting => println!("Acting"),
    //         }
    //     }
    //     _ => println!("Others")
    // }

    match major1 {
        Major::Art(Art::Music) => println!("Music"),
        Major::Art(Art::Painting) => println!("Painting"),
        Major::Art(Art::Acting) => println!("Acting"),
        Major::Science => println!("Science"),
        Major::Other { id, name } => {
            println!("Other: {name}, with id: {id}")
        }
    }
}

struct Student {
    id: u32,
    name: Name,
    major: Major
}

struct Name {
    first: String,
    last: String,
    nickname: String
}


pub fn struct_extract_demo() {
    let name = Name {
        first: "Peter".to_string(),
        last: "Parker".to_string(),
        nickname: "Spiderman".to_string()
    };

    let first = &name.first;
    let last = &name.last;
    let nickname = &name.nickname;
    println!("{first} {last}, aka. {nickname}");

    let Name { first, last, nickname } = &name;
    println!("{first} {last}, aka. {nickname}");

    let Name { last, .. } = name;
    println!("{last}");
}

pub fn tuple_extract_demo() {
    let tup = ("Satu", 2, 3.0, "Empat".to_string());

    let (satu, ..) = tup;
    let (_, dua, ..) = tup;
    let (.., tiga, _) = tup;
    let (.., empat) = tup;

    println!("{} {} {} {}", satu, dua, tiga, empat);
}

pub fn at_dan_match_guard() {
    let number = 20;

    match number {
        1|3|5|7|11 => println!{"Bilangan prima"},
        n @ 10..=30 if (n%2==0) => {
            println!{"{n} = bilangan genap antara 10 s/d 30"};
        }
        n @ 10..=15 | n @ 20..=30 if (n%2!=0) => {
            println!{"{n} = bilangan ganjil antara 10 s/d 15 dan 20 s/d 30"};
        }
        0.. => println!("0 sampai infinity"),
        _ => ()     // do nothing
    }
}
