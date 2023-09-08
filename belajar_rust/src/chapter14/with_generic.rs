use std::marker::PhantomData;

pub trait Builder<T> { }

pub trait Finishable {
    type Output;
    fn build(self) -> Self::Output;
}

pub trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

#[derive(Debug)]
pub struct Bicycle {
    make: String,
    model: String,
    size: u32,
    color: String
}

#[allow(dead_code)]
impl Bicycle {
    pub fn make(&self) -> &str { &self.make }
    pub fn model(&self) -> &str { &self.model }
    pub fn size(&self) -> u32 { self.size }
    pub fn color(&self) -> &str { &self.color }
    pub fn print(&self) {
        println!("Make: {}\nModel: {}\nSize: {}\nColor: {}\n", 
            self.make, self.model, self.size, self.color
        )
    }
}
impl Buildable<Self, BicycleBuilder<Start>> for Bicycle {
    fn builder() -> BicycleBuilder<Start> {
        BicycleBuilder::new()
    } 
}


pub struct Start{} pub struct Two{} pub struct Three{} pub struct Four{} pub struct Finish{}
trait Stage { }

impl Stage for Start { } impl Stage for Two { } impl Stage for Three { } impl Stage for Four { } impl Stage for Finish { }

pub struct BicycleBuilder<Stage> {
    bicycle: Bicycle,
    marker: PhantomData<Stage>
}

impl Builder<Bicycle> for BicycleBuilder<Start> { }

impl BicycleBuilder<Start> {
    fn new() -> Self {
        Self { 
            bicycle: Bicycle { 
                make: String::new(), 
                model: String::new(), 
                size: 0, 
                color: String::new()
            },
            marker: PhantomData::default()
        }
    } 
    pub fn with_make(mut self, make: &str) -> BicycleBuilder<Two> {
        self.bicycle.make = make.into();
        BicycleBuilder { 
            bicycle: self.bicycle, 
            marker: PhantomData::<Two>::default() 
        }
    }
}
impl BicycleBuilder<Two> {
    pub fn with_model(mut self, model: &str) -> BicycleBuilder<Three> {
        self.bicycle.model = model.into();
        BicycleBuilder { 
            bicycle: self.bicycle, 
            marker: PhantomData::<Three>::default() 
        }
    }
}
impl BicycleBuilder<Three> {
    pub fn with_size(mut self, size: u32) -> BicycleBuilder<Four> {
        self.bicycle.size = size;
        BicycleBuilder { 
            bicycle: self.bicycle, 
            marker: PhantomData::<Four>::default() 
        }
    }
}
impl BicycleBuilder<Four> {
    pub fn with_color(mut self, color: &str) -> BicycleBuilder<Finish> {
        self.bicycle.color = color.into();
        BicycleBuilder { 
            bicycle: self.bicycle, 
            marker: PhantomData::<Finish>::default() 
        }
    }
}

// impl Finishable
impl Finishable for BicycleBuilder<Finish> {
    type Output = Bicycle;
    fn build(self) -> Self::Output { self.bicycle }
}
impl Finishable for BicycleBuilder<Two> {
    type Output = Bicycle;
    fn build(self) -> Self::Output { self.bicycle }
}

pub fn with_generic_demo() {
    let builder = Bicycle::builder()
        .with_make("Poligon")
        .with_model("AB220")
        .with_size(23)
        .with_color("RED")
        .build();

    builder.print();
}

