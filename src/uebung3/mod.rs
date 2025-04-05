pub mod aufgabe_3_1;
pub mod aufgabe_3_2;
pub mod aufgabe_3_3;
pub mod aufgabe_3_4;

#[allow(dead_code)]
pub fn start() {
    println!("Grüß dich von Übung 3!");

    aufgabe_3_1::run();
    aufgabe_3_2::run();
    aufgabe_3_3::run();
    aufgabe_3_4::run();
}
