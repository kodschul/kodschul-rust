pub struct UpCounter {
    pub name: &'static str,
    pub value: u8,
}

pub struct DownCounter {
    pub name: &'static str,
    pub value: u8,
}

pub trait Count {
    fn count(&mut self);
    fn display(&self);
}

impl Count for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }

    fn display(&self) {
        println!("{} hat den Wert {}", self.name.to_string(), self.value);
    }
}

impl Count for DownCounter {
    fn count(&mut self) {
        self.value -= 1;
    }

    fn display(&self) {
        println!("{} hat den Wert {}", self.name.to_string(), self.value);
    }
}

pub fn run() {
    let mut up = UpCounter {
        name: "up",
        value: 0,
    };

    let mut down = DownCounter {
        name: "down",
        value: 20,
    };

    for i in 0..3 {
        up.count();
        up.display();
        down.count();
        down.display();
    }
}
