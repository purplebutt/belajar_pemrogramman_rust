use std::future::Future;
use std::pin::Pin;
use std::sync::Weak;
use std::sync::Arc;


pub trait Observer: Sync + Send {
    type Subject;
    type Output;
    fn observe<'a>( &'a self, subject: &'a Self::Subject
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>>;
}

pub struct MyObserver {
    id: String
}
impl MyObserver {
    pub fn new(state: &str) -> Arc<Self> {
        let me = Self { id: state.into() };
        Arc::new(me)
    }
}
impl Observer for MyObserver {
    type Subject = Subject;
    type Output = ();
    fn observe<'a>( &'a self, subject: &'a Self::Subject
        ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>> {
        Box::pin(
            async {
                use tokio::time::sleep;
                use std::time::Duration;
                println!("observe subject with state: '{}' in {}", subject.state, self.id);
                sleep(Duration::from_secs(1)).await;
            }
        )
    }
}

pub trait Observable {
    type Observer;
    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self, Output = ()>>>,
    state: String
}
impl Subject {
    pub fn new(state: &str) -> Self {
        Self { observers: vec![], state: state.into() }
    }
    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}
impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self, Output = ()>>;
    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> {
        let observers: Vec<_> = self.observers.iter().flat_map(|o| o.upgrade()).collect();
        Box::pin( 
            async move {
                futures::future::join_all(
                    observers.iter().map(|o| o.observe(self))
                ).await;
            }
        )
    }
    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }
    fn detach(&mut self, observer: Self::Observer) {
        self.observers.retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}

