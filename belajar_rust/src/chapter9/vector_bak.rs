pub struct Domino {
    up: u8, dw: u8, idx: u8
}

impl Domino {
    pub fn new(up: u8, dw: u8, idx: u8) -> Self {
        Self { up, dw, idx }
    }

    pub fn compare(&self, rhs: &Domino) -> std::cmp::Ordering {
        if self.idx < rhs.idx {
            std::cmp::Ordering::Less
        }
        else if self.idx == rhs.idx {
            std::cmp::Ordering::Equal
        }
        else {
            std::cmp::Ordering::Greater
        }
    }

    pub fn diff(&self, rhs: &Domino) -> i8 {
        self.idx as i8 - rhs.idx as i8
    }
}

pub struct Dominos(Vec<Domino>);

impl Dominos {
    pub fn new() -> Self {
        let mut collections  = Vec::<Domino>::new();
        let mut idx = 0;
        for up in 0..=6 {
            for dw in 0..=6 {
                collections.push(Domino::new(up, dw, idx));
                idx += 1;
            }
        }
        Self(collections)
    }

    pub fn from_filtered(filtered: Vec<&Domino>) -> Self {
        let mut new_col:Vec<Domino> = Vec::new();
        for domino in filtered {
            new_col.push(Domino::new(domino.up, domino.dw, domino.idx));
        }
        Self(new_col)
    }

    pub fn print_all(&self) {
        for domino in self.0.iter() {
            println!("|{}:{}|", domino.up, domino.dw);
        }
    }

    pub fn filter_up_into(&self, filter: u8) -> Self {
        let filtered: Vec<&Domino> = self.0.iter().filter(|d| d.up == filter).collect();
        // let filtered = self.0.iter().filter(|d| d.up == filter).collect::<Vec<&Domino>>();
        Self::from_filtered(filtered)
    }

    pub fn filter_dw_into(&self, filter: u8) -> Self {
        let filtered: Vec<&Domino> = self.0.iter().filter(|d| d.dw == filter).collect();
        // let filtered = self.0.iter().filter(|d| d.dw == filter).collect::<Vec<&Domino>>();
        Self::from_filtered(filtered)
    }

    pub fn filter_up_ref(&self, filter: u8) -> Vec<&Domino> {
        self.0.iter().filter(|d| d.up == filter).collect()
    }

    pub fn filter_dw_ref(&self, filter: u8) -> Vec<&Domino> {
        self.0.iter().filter(|d| d.dw == filter).collect()
    }
}

pub fn vec_demo() {
    let dominos = Dominos::new();
    dominos.print_all();

    println!("{}", "=".repeat(30));

    let dominos_dw2 = dominos.filter_dw_ref(2);

    for &domino in dominos_dw2.iter() {
        println!("|{}:{}| {}", domino.up, domino.dw, domino.idx);
    }

    println!("{}", "=".repeat(30));
    let dominos_dw5 = dominos.filter_up_into(5);
    dominos_dw5.print_all();
    
    let da = dominos_dw2.first().unwrap();
    let db = dominos_dw2.last().unwrap();

    match da.compare(db) {
        std::cmp::Ordering::Equal => println!("Equal"),
        std::cmp::Ordering::Greater => println!("Greater"),
        std::cmp::Ordering::Less => println!("Less")
    }

    println!("{}", da.diff(db));
}
