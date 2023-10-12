// Stack-Struktur, die nur Integer speichert
pub struct Stack {
    // Innerer Vektor, der die Integer speichert
    items: Vec<i32>,
}

impl Stack {
    // Erstellt einen neuen leeren Stack
    pub fn new() -> Self {
        // Gibt auf der Konsole aus, dass ein neuer Stack erstellt wird
        println!("Erstellt einen neuen, leeren Stack");
        Stack {
            // Initialisiert den Stack mit einem leeren Vektor
            items: Vec::new(),
        }
    }

    // Fügt eine Integer zum Stack hinzu
    pub fn push(&mut self, item: i32) {
        // Gibt auf der Konsole aus, welcher Integer zum Stack hinzugefügt wird
        println!("Füge die Zahl `{}` zum Stack hinzu", item);
        // Die push-Methode von Vektor fügt eine Integer am Ende hinzu
        self.items.push(item);
    }

    // Entfernt und gibt die oberste Integer vom Stack zurück
    pub fn pop(&mut self) -> Option<i32> {
        // Die pop-Methode von Vektor entfernt und gibt das letzte Element zurück
        // Gibt in einem leeren Vektor eine None zurück
        match self.items.pop() {
            Some(value) => {
                // Gibt aus, welcher Integer entfernt wurde
                println!("Entfernen und returnieren die oberste Zahl vom Stack: `{}`", value);
                Some(value)
            },
            None => {
                // Gibt aus, dass versucht wurde, einen Integer aus einem leeren Stack zu entfernen
                println!("Versucht, eine Zahl aus einem leeren Stack zu entfernen: NULL");
                None
            }
        }
    }

    // Gibt die oberste Integer vom Stack zurück ohne es zu entfernen
    pub fn peek(&self) -> Option<&i32> {
        // Die last-Methode von Vektor gibt eine Referenz zum letzten Element zurück
        // In einem leeren Vektor gibt es eine None zurück
        match self.items.last() {
            Some(value) => {
                // Gibt aus, welcher Integer das oberste Element ist
                println!("Peek auf die oberste Zahl vom Stack (ohne entfernen): `{}`", value);
                Some(value)
            },
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

    // Versucht, das oberste Element zu sehen, aber der Stack ist leer
    stack.peek();

    stack.push(10);

    stack.peek();

    stack.push(20);

    stack.peek();

    stack.pop();

    stack.pop();

    // Versucht, ein Element zu entfernen, aber der Stack ist leer
    stack.pop();
}