pub enum Progress {
    Finish,
    UnFinish(String)
}

pub trait AFuture {
    fn update(&mut self) -> Progress;
}

pub struct MyFuture {
    is_complete: bool,
    messages: std::collections::VecDeque<String>,
}

impl MyFuture {
    pub fn new(messages: Vec<&str>) -> Self {
        let mut vd = std::collections::VecDeque::with_capacity(messages.len());
        for message in messages {
            vd.push_back(message.to_string())
        }

        Self { 
            is_complete: false, 
            messages: vd,
        }
    }
}

impl AFuture for MyFuture {
    fn update(&mut self) -> Progress {
        match self.is_complete {
            true => Progress::Finish,
            false => {
                let message = self.messages.pop_front().unwrap();
                if self.messages.len() == 0 {
                    self.is_complete = true
                }
                std::thread::sleep(std::time::Duration::from_millis(700));
                Progress::UnFinish(message)
            }
        } 
    }
}

