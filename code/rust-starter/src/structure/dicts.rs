use std::collections::{HashMap, HashSet};

pub fn run() {
    let mut fruit_colors = HashMap::new();

    fruit_colors.insert("Apple", "Red");
    fruit_colors.insert("Banana", "Yellow");
    fruit_colors.insert("Grape", "Purple");

    fruit_colors.insert("Apple", "Green");

    println!("{:?}", fruit_colors);

    let apple_color = fruit_colors.get("Apple");
    let apple_color_text = if apple_color != None {
        apple_color.unwrap()
    } else {
        "no_color"
    };

    println!("Apple color: {}", apple_color_text);

    return;

    let mut fruits = Vec::new();

    fruits.push("Apple");
    fruits.push("Banana");
    fruits.push("Apple");

    println!("{:?}", fruits);

    let mut fruits = HashSet::new();
    fruits.insert("Apple");
    fruits.insert("Banana");
    fruits.insert("Apple");

    println!("{:?}", fruits);

    let fruits = ("Apple", "Banana", "Lemonade");

    println!("{:?}", fruits);
    println!("Element 0: {} Element 2: {}", fruits.0, fruits.2);

    return;
}
