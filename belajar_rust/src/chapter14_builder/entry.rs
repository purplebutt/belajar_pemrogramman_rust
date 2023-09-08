pub struct Item {
    key: u32,
    val: String 
}
impl Item {
    pub fn new (key: u32, val: String) -> Self {
        Self { key, val }
    } 
}

pub struct Dictionary(Vec<Item>);

impl Dictionary {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn insert(&mut self, key: u32, val: &str) {
        let item = Item::new(key, val.to_string());
        self.0.push(item);
    }
    pub fn get(&self, key: u32) -> Option<&str> {
        for i in self.0.iter() {
            if i.key == key {
                return Some(&i.val)
            }
        }
        None
    }
    pub fn item(&self, key: u32) -> Option<Item> {
        if let Some(val) = self.get(key) {
            Some(Item::new(key, val.into()))
        } else {
            None
        }
    }
    pub fn has_key(&mut self, key: u32) -> bool {
        for i in self.0.iter() {
            if i.key == key {
                return  true;
            }
        }
        false
    }
    pub fn keys(&self) -> Vec<u32> {
        self.0.iter().map(|i|i.key).collect::<Vec<u32>>()
    }
}


pub fn entry_demo() {
    let mut d = Dictionary::new();

    let k = 1;
    if !d.has_key(k) { d.insert(k, "Satu".into()); }
    d.insert(5, "Lima");

    d.insert(2, "Dua".into());

    let x = d.get(5);
    println!("{}", x.unwrap());

    for &k in d.keys().iter() {
        println!("{} => {}", k, d.get(k).unwrap());
    }
}

