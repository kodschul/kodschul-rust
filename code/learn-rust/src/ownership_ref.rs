pub fn main_old() {
    let mut s = String::from("Hallo");

    let r1 = &mut s;
    r1.push_str(" Welt");
    println!("r1: {}", r1);

    let r2 = &mut s;

    let reader = &s;

    // r1 wird nicht mehr benutzt → jetzt darf man wieder s verwenden
    println!("{:?}", (reader));
}

pub fn main() {
    let mut s = String::from("Hallo");

    let r1 = &s; // immutable borrow (Leser)
    let r2 = &mut s; // ❌ Versuch: mutable borrow (Schreiber)

    println!("{:?}", (r2));
}
