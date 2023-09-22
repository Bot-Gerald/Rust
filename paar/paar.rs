// Definiere eine generische Struktur namens 'Paar', welche zwei Felder des gleichen Datentyps 'T' hat.
struct Paar<T> {
    x: T,  // Das Feld 'x' ist vom Datentyp 'T'
    y: T,  // Das Feld 'y' ist ebenso vom Datentyp 'T'
}

// Die Hauptfunktion unseres Programms beginnt hier.
fn main() {
    
    // Deklariere eine Variable 'punkt', die eine Instanz des 'Paar' Strukturtyps ist, und spezifiziere 'i32' als den generischen Typ 'T'.
    let punkt: Paar<i32> = Paar { x: 1, y: 2 };

    // Deklariere eine weitere Variable 'wort', die eine weitere Instanz des 'Paar' Strukturtyps ist, allerdings ist 'T' diesmal ein String literal (&str).
    let wort: Paar<&str> = Paar { x: "Hallo", y: "Welt" };
  
    // Drucke die 'x' und 'y' Werte der Variablen 'punkt'. Da wir 'T' als 'i32' spezifiziert haben, sind dies Zahlen.
    println!("{}, {}", punkt.x, punkt.y);
  
    // Drucke die 'x' und 'y' Werte der Variablen 'wort'. Da wir 'T' als '&str' spezifiziert haben, sind dies String Literale.
    println!("{}, {}", wort.x, wort.y);
}
