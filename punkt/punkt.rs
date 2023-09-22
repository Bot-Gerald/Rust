// Definiere eine Struktur mit dem Namen "Punkt", die zwei generische Typen `T` und `U` hat.
struct Punkt<T, U> {
    x: T, // Das Feld `x` ist vom Typ `T`
    y: U, // Das Feld `y` ist vom Typ `U`
}

// Implementiere Funktionen für den "Punkt"-Typ. Die Typen `T` und `U` sind vom umgebenen Kontext und entsprechen den in der Strukturdefinition von "Punkt" angegebenen
impl<T, U> Punkt<T, U> {

    // Die Funktion "mixup" akzeptiert einen anderen "Punkt" als Parameter, der jedoch andere generische Typen `V` und `W` haben kann.
    fn mixup<V, W>(self, other: Punkt<V, W>) -> Punkt<T, W> {

        // Gibt einen neuen "Punkt" zurück, der das Feld `x` aus dem ursprünglichen "Punkt" (`self`) und das Feld `y` aus dem anderen "Punkt" verwendet.
        Punkt {
            x: self.x,  // `x` ist das `x` vom ursprünglichen "Punkt"
            y: other.y, // `y` ist das `y` vom anderen "Punkt"
        }
    }
}

// Die Hauptfunktion des Programms beginnt hier. Rust-Programme beginnen immer mit der Ausführung von `main`.
fn main() {

    // Erzeuge einen "Punkt" `p1` mit `x` vom Typ `i32` und `y` vom Typ `f64`.
    let p1 = Punkt { x: 5, y: 10.4 };
    
    // Erzeuge einen anderen "Punkt" `p2` mit `x` vom Typ `&str` und `y` vom Typ `char`.
    let p2 = Punkt { x: "Hallo", y: 'c'};

    // Verwende die Funktion "mixup", um einen neuen "Punkt" zu erzeugen, `p3`, der das `x` von `p1` und das `y` von `p2` verwendet.
    let p3 = p1.mixup(p2);

    // Gib die Werte von `p3` aus. Da "mixup" das `x` von `p1` und `y` von `p2` verwendet, sollte dies "p3.x = 5, p3.y = c" ausgeben.
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
