use rust_starter::structure::traits::{Count, DownCounter, UpCounter};

#[test]
fn test_up_count() {
    let mut up = UpCounter {
        name: "up",
        value: 0,
    };
    assert_eq!(0, up.value);
    up.count();
    assert_eq!(1, up.value);
}
