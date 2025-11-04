enum State {
    Idle,
    Running,
    Paused(i32),
    Completed,
}

pub fn run() {
    let current_state = State::Paused(20);

    let state_message: String = match current_state {
        State::Idle => "idling".to_string(),
        State::Running => "running".to_string(),
        State::Paused(10) => format!("paused for specific 10 seconds!"),
        State::Paused(x) => format!("paused for {x} seconds"),
        State::Completed => "completed".to_string(),
        _ => "unknown".to_string(),
    };

    println!("Current State: {state_message}");
}
