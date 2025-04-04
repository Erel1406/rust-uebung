use std::io;

mod uebung1;
mod uebung2;
mod uebung3;
mod uebung4;
mod uebung5;

fn main() {
    println!("🦀 Willkommen zur interaktiven Rust-Übung!");
    println!("Welche Aufgabe möchtest du ausführen?");
    println!("Beispieleingaben:");
    println!("  1.1  → Übung 1, Aufgabe 1");
    println!("  2.3  → Übung 2, Aufgabe 3");
    println!("  5.1  → Übung 5, Aufgabe 1");
    println!("------------------------------------------");
    print!("Deine Auswahl: ");

    let mut eingabe = String::new();
    io::stdin()
        .read_line(&mut eingabe)
        .expect("Fehler beim Einlesen");

    let eingabe = eingabe.trim();

    match eingabe {
        "1.1" => uebung1::aufgabe_1_1::run(),
        "1.2" => uebung1::aufgabe_1_2::run(),
        "1.3" => uebung1::aufgabe_1_3::run(),
        "1.4" => uebung1::aufgabe_1_4::run(),

        "2.1" => uebung2::aufgabe_2_1::run(),
        "2.2" => uebung2::aufgabe_2_2::run(),
        "2.3" => uebung2::aufgabe_2_3::run(),
        "2.4" => uebung2::aufgabe_2_4::run(),

        "3.1" => uebung3::aufgabe_3_1::run(),
        "3.2" => uebung3::aufgabe_3_2::run(),
        "3.3" => uebung3::aufgabe_3_3::run(),
        "3.4" => uebung3::aufgabe_3_4::run(),

        "4.1" => uebung4::aufgabe_4_1::run(),
        "4.2" => uebung4::aufgabe_4_2::run(),

        "5.1" => uebung5::aufgabe_5_1::run(),
        "5.2" => uebung5::aufgabe_5_2::run(),

        _ => println!("Ungültige Eingabe. Bitte z.B. 1.2 oder 4.1 eingeben."),
    }
}
