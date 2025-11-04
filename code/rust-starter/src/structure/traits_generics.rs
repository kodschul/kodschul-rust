struct UpCounter {
    name: &'static str,
    value: u8,
}

struct DownCounter {
    name: &'static str,
    value: u8,
}

trait Count {
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

    count_10x(&mut up);
    count_10x(&mut down);
}

fn count_10x<T: Count>(item: &mut T) {
    for _ in 0..10 {
        item.count();
        item.display();
    }
}
