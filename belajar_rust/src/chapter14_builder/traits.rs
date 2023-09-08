pub trait Builder<T> { }

// pub trait Builder<T> {
//     fn default() -> Self;
// }

pub trait Buildable<B: Builder<Self>> where Self: Sized {
    fn builder() -> B;
    // fn builder() -> B {
    //     B::default()
    // }
}

pub trait Finishable {
    type Output;
    fn finish(self) -> Self::Output;
}

pub trait SkipTo<To: Finishable> {
    fn skip(self, text: &str) -> To;
}
