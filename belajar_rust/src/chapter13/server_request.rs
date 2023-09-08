use std::error::Error;
use std::thread;
use std::time::Duration;


fn download(_url: &str) -> Result<String, &dyn Error> {
    thread::sleep(Duration::from_secs(2));
    Ok("{'message': 'hello'}".to_string())
}


pub fn test() {
    let data = download("www.example.com/hello.json");

    match data {
        Ok(d) => println!("Response: {}", d),
        _ => ()
    }
}
