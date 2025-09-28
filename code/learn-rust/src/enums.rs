pub fn run() {
    let current_state = State::Running(None);

    match current_state {
        State::Idle => println!("idle"),
        State::Running(None) => println!("running without Duration "),
        State::Running(duration) => println!("running, for {} long", duration.unwrap()),
        State::Paused(10) => println!("Paused 10 secs"),
        State::Paused(20) => println!("Paused 20 secs"),
        State::Paused(seconds) => println!("Paused dynamic: for {seconds} secs"),
        State::Completed => println!("completed"),
    }

    let x: Option<i32> = Some(10);

    // println!("x: {}", x)
}

enum State {
    Idle,
    Running(Option<usize>),
    Paused(usize),
    Completed,
}
