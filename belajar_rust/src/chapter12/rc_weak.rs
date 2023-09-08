use std::{cell::RefCell, rc::{Rc, Weak}};


type CBox = RefCell<Vec<Rc<Human>>>;
type PBox = RefCell<Weak<Human>>;

#[derive(Debug)]
struct Human {
    value: String,
    parent: Option<PBox>,
    childs: CBox
}

impl Human {
    fn new(value: &str) -> Self {
        Self { 
            value: value.to_string(), 
            parent: Some(Self::def()),
            childs: RefCell::new(vec![]) 
        }
    }
    fn def() -> PBox {
        let h = Self { 
            value: "".to_string(), 
            parent: None, 
            childs: RefCell::new(vec![])
        };
        RefCell::new(
            Rc::downgrade(&h.as_rbox())
        )
    }
    fn as_rbox(self) -> Rc<Self> {
        Rc::new(self)
    }
    fn get_parent(&self) -> Rc<Human> {
        self.parent.as_ref().unwrap()
            .borrow().upgrade().unwrap()
    }
    fn get_value(&self) -> String {
        self.value.clone()
    }
    fn set_parent(&self, parent: &Rc<Human>) {
        if let Some(p) = self.parent.as_ref() {
            *p.borrow_mut() = Rc::downgrade(parent);
        }
    }
    fn set_child(&self, child: Rc<Human>) {
        self.childs.borrow_mut().push(child);
    }
}

fn preview_count(human: &Rc<Human>, stage: &str) {
    let name = human.get_value();

    println!("{}", ".".repeat(40));
    println!("strong_count {name} \t({}): {}", 
        stage, Rc::strong_count(&human)
    );

    println!("weak_count {name} \t({}): {}", 
        stage, Rc::weak_count(&human)
    );
    println!("{}", ".".repeat(40));
}

pub fn weakdemo() {
    let joko = Human::new("Joko").as_rbox();
    preview_count(&joko, "awal");

    let raka = Human::new("Raka").as_rbox();
    let gibran = Human::new("Gibran").as_rbox();

    // set joko childs
    joko.set_child(raka.clone());
    joko.set_child(gibran.clone());

    raka.set_parent(&joko);
    println!("Weakcount (after add raka): {}", Rc::weak_count(&joko));

    gibran.set_parent(&joko);
    println!("Weakcount (after add gibran): {}", Rc::weak_count(&joko));

    preview_count(&joko, "akhir");
}

