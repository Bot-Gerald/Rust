use std::fmt::Debug;

// Generische Stack-Struktur, die in der Lage ist, Elemente irgendeines Typs zu speichern
pub struct Stack<T> where T: Debug{
    // Innerer Vektor, der die Elemente speichert
    items: Vec<T>,
}

impl<T> Stack<T> where T: Debug {
    // Erstellt einen neuen leeren Stack
    pub fn new() -> Self {
        // Gibt auf der Konsole aus, dass ein neuer Stack erstellt wird
        println!("Erstellt einen neuen, leeren Stack");
        Stack {
            // Initialisiert den Stack mit einem leeren Vektor
            items: Vec::new(),
        }
    }

    // Fügt ein Element zum Stack hinzu
    pub fn push(&mut self, item: T) {
        // Gibt auf der Konsole aus, welches Element zum Stack hinzugefügt wird
        println!("Füge das Element `{:?}` zum Stack hinzu", item);
        // Die push-Methode von Vektor fügt ein Element am Ende hinzu
        self.items.push(item);
    }

    // Entfernt und gibt das oberste Element vom Stack zurück
    pub fn pop(&mut self) -> Option<T> {
        // Die pop-Methode von Vektor entfernt und gibt das letzte Element zurück
        // In einem leeren Vektor gibt es eine None zurück
        match self.items.pop() {
            Some(value) => {
                // Gibt aus, welches Element entfernt wurde
                println!("Entfernen und returnieren des obersten Elements vom Stack: `{:?}`", value);
                Some(value)
            },
            None => {
                // Gibt aus, dass versucht wurde, ein Element aus einem leeren Stack zu entfernen
                println!("Versucht, ein Element aus einem leeren Stack zu entfernen: NULL");
                None
            }
        }
    }

    // Gibt das oberste Element vom Stack zurück ohne es zu entfernen
    pub fn peek(&self) -> Option<&T> {
        // Die last-Methode von Vektor gibt eine Referenz zum letzten Element zurück
        // In einem leeren Vektor gibt es eine None zurück
        match self.items.last() {
            Some(value) => {
                // Gibt aus, welches Element das oberste Element ist
                println!("Peek auf das oberste Element vom Stack (ohne entfernen): `{:?}`", value);
                Some(value)
            }
            None => {
                // Gibt aus, dass ein Peek auf einen leeren Stack durchgeführt wurde
                println!("Peek auf einen leeren Stack: NULL");
                None
            }
        }
    }

    // Überprüft, ob der Stack leer ist
    pub fn is_empty(&self) -> bool {
        // Die is_empty-Methode von Vektor gibt true zurück, wenn der Vektor leer ist
        if self.items.is_empty() {
            // Gibt aus, dass der Stack leer ist
            println!("Der Stack ist leer");
        } else {
            // Gibt aus, dass der Stack nicht leer ist
            println!("Der Stack ist nicht leer");
        }
        
        self.items.is_empty()
    }
}

fn main() {

    // Erstellt einen neuen leeren Stack
    let mut stack = Stack::new();

    stack.peek();

    stack.push("moin");

    stack.peek();

    stack.push("servus");
 
    stack.peek();


    stack.pop();

    stack.pop();

    // Versucht, ein Element zu entfernen, aber der Stack ist leer
    stack.pop();
}