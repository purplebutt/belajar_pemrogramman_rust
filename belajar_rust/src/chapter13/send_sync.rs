use std::thread;
use std::rc::Rc;
use std::time::Duration;

// helper function untuk build url
fn url_builder(url: &str, port: &str) -> String {
        let mut url = String::from(url);
        url.push(':');
        url.push_str(port);
        url.push('/');
        url
}

trait AsUrl {
    fn as_url(&self) -> String;
}

// Url1
struct Url1 {
    url: String, port: u32
}
impl Url1 {
    pub fn new(url: &str, port: u32) -> Self {
        Self { url: url.into(), port }
    }
}
impl AsUrl for Url1 {
    fn as_url(&self) -> String {
        url_builder(&self.url, self.port.to_string().as_str())
    } 
}
//

// Url2
struct Url2 {
    url: String, port: Rc<u32>
}
impl Url2 {
    pub fn new(url: &str, port: u32) -> Self {
        Self { url: url.into(), port: Rc::new(port) }
    }
}
impl AsUrl for Url2 {
    fn as_url(&self) -> String {
        url_builder(&self.url, self.port.to_string().as_str())
    } 
}
unsafe impl Send for Url2 { }
//

fn download(url: impl AsUrl) -> String {
    thread::sleep(Duration::from_millis(700));
    format!("{{\n  'message': 'hello',\n  'server': {}\n}}", url.as_url())
}

pub fn send_sync_demo() {
    let url1 = Url1::new("www.mastodon.com", 80);
    let url2 = Url2::new("www.duckduckgo.com", 80);

    let spawned1 = thread::spawn(move||{
        download(url1)
    });
    let spawned2 = thread::spawn(move||{
        download(url2)
    });
    
    let _s1 = spawned1.join();
    let _s2 = spawned2.join();

    println!("Spawned Thread 1:\n{}", _s1.unwrap());
    println!("Spawned Thread 2:\n{}", _s2.unwrap());
}

