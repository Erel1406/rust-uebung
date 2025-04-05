pub mod aufgabe_2_1;
pub mod aufgabe_2_2;
pub mod aufgabe_2_3;
pub mod aufgabe_2_4;

#[allow(dead_code)]
pub fn start() {
    println!("Moin aus Übung 2! :)");

    aufgabe_2_1::run();
    aufgabe_2_2::run();
    aufgabe_2_3::run();
    aufgabe_2_4::run();
}
