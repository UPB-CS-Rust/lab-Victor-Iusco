//! You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Trecem o referință, nu proprietatea

    string_uppercase(data); // Trecem proprietatea, funcția o va prelua
}

// Nu ia proprietatea, ci doar o referință la șir
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Preia proprietatea asupra șirului și îl modifică
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // `to_uppercase` returnează un nou șir, modificăm `data`

    println!("{}", data);
}
