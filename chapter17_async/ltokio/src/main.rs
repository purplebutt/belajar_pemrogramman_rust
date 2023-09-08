#![allow(dead_code)]

use ltokio::observer::{ Observable, Subject, MyObserver };
use ltokio::filesys::test;


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    test().await
}


async fn test_observer() {
    let mut subject = Subject::new("test");
    let obs1 = MyObserver::new("ob1");
    let obs2 = MyObserver::new("ob2");

    subject.attach(obs1.clone());
    subject.attach(obs2.clone());

    subject.update().await;
}

