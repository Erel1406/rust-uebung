pub mod aufgabe_5_1;
pub mod aufgabe_5_2;

pub fn start() {
    println!("Willkommen zur Übung 5! :)");

    #[allow(dead_code)]
    aufgabe_5_1::run();
    aufgabe_5_2::run();
}
