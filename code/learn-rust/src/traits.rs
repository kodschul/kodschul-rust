struct Counter {
    name: String,
    value: u8,
}

struct UpCounter {
    name: String,
    value: u8,
}

struct DownCounter {
    name: String,
    value: u8,
}

trait Count {
    fn count(&mut self) {
        self.reset();
    }

    fn reset(&mut self);
}

impl Count for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }

    fn reset(&mut self) {}
}

impl Count for DownCounter {
    fn count(&mut self) {
        self.value -= 1;
    }

    fn reset(&mut self) {
        println!("reset");
    }
}

pub fn run() {
    let mut up_counter = UpCounter {
        name: "up_counter".to_string(),
        value: 0,
    };
    let mut down_counter = DownCounter {
        name: "down_counter".to_string(),
        value: 20,
    };

    for _i in 0..3 {
        up_counter.count();
        down_counter.count();
    }

    println!(
        "{}: hat folgende Wert {}",
        up_counter.name, up_counter.value
    );
    println!(
        "{}: hat folgende Wert {}",
        down_counter.name, down_counter.value
    );
}
