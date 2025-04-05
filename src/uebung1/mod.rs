pub mod aufgabe_1_1;
pub mod aufgabe_1_2;
pub mod aufgabe_1_3;
pub mod aufgabe_1_4;

#[allow(dead_code)]
pub fn start() {
    println!("<3-lich Willkommen zur Übung 1!");

    aufgabe_1_1::run();
    aufgabe_1_2::run();
    aufgabe_1_3::run();
    aufgabe_1_4::run();
}
