/* Übung 3.3 – Menüauswahl simulieren
   TODO: Nutze match, um verschiedene Menüoptionen umzusetzen.
*/

pub fn run() {
    let eingabe = "2"; // ← Stelle dir vor, das kommt von der Tastatur

    match eingabe {
        "1" => println!("Option 1: Daten anzeigen"),
        "2" => println!("Option 2: Daten bearbeiten"),
        // TODO: weitere Option hinzufügen
        _ => println!("Ungültige Eingabe"),
    }
}
