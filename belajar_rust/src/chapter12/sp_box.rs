
pub type NodeBox = Option<Box<Node>>;

pub struct Node {
    val: i32,
    next: NodeBox
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
    pub fn put(self, mut other: Self) -> Self {
        other.next = self.to_box();
        other
    }
    pub fn put_fromval(self, val: i32) -> Self {
        self.put(Node::new(val))
    }
    pub fn to_box(self) -> NodeBox {
        Some(Box::new(self))
    }
    pub fn value(&self) -> i32 {
        self.val
    }
    pub fn get_next(&self) -> &NodeBox {
        &self.next
    }
    pub fn iter(&self) -> NodeIter {
        NodeIter::new(self) 
    }
}

// iterator untuk Node
struct NodeIter<'a>{
    is_first: bool,
    current: &'a Node
}
impl<'a> NodeIter<'a> {
    pub fn new(current: &'a Node) -> Self {
        Self { is_first: true, current }
    }
}

// implementasi trait Iterator untuk struct NodeIter
impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(&self.current)
        }
        else {
            match self.current.next.as_ref() {
                Some(n)  => {
                    self.current = n;
                    Some(n)
                },
                _ => None
            }
        }
    } 
}


pub fn box_example() {
    // use std::ops::Deref;

    let mybox = Box::new(1_000_u32);

    println!("log10 dari {} = {}", mybox, mybox.ilog10());

    let total = *mybox + 500;
    // let total = mybox.deref() + 500;
    println!("{} + 500 = {}", mybox, total);

    
    let mut yourbox = Box::new(23);

    *yourbox = *yourbox + 7;
    println!("{} + 7 = {}", 23, yourbox);

    let yb = yourbox.as_mut();
    *yb /= 2;

    println!("30 / 2 = {}", yourbox);
}


pub fn test() {
    let n1 = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    let n2 = n1.put(n2);
    let n3 = n2.put(n3);

    let n4 = n3.put_fromval(4);

    for n in n4.iter().enumerate() {
        println!("{} {}", n.0, n.1.value())
    }

    // let mut n = &n4.to_box();
    // loop {
    //     match n {
    //         Some(i) => {
    //             println!("{}", i.value());
    //             n = i.get_next();
    //         },
    //         _ => break
    //     }
    // }
}

