// Eine einfache Funktion, um den Durchschnitt einer Liste von Gleitkommazahlen zu berechnen
fn average(numbers: &[f32]) -> f32 {
    let sum: f32 = numbers.iter().sum(); // Summe aller Zahlen in der Liste
    let count = numbers.len() as f32; // Anzahl der Elemente in der Liste
    sum / count // Durchschnitt = Summe / Anzahl
}

// Verwendungsbeispiel
fn main() {
    let nums = [1.0, 2.0, 3.0, 4.0, 5.0];
    let avg = average(&nums); // Wir müssen eine Referenz (&) zu dem Array übergeben, weil Rust Ownership-Regeln die Übertragung der Variable an die Funktion verhindern würden
    println!("Durchschnitt: {}", avg);
}
