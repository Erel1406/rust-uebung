pub mod aufgabe_4_1;
pub mod aufgabe_4_2;

pub fn start() {
    println!("Willkommen zur Übung 4!");

    #[allow(dead_code)]
    aufgabe_4_1::run();
    aufgabe_4_2::run();
}
