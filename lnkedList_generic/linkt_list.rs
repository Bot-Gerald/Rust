// Die folgenden Zeilen sind nötig, um Display-Eigenschaften nutzen zu können (für die print-Anweisung)
use std::fmt::{self,Debug};

// Füge das Debug-Trait zu Node und LinkedList hinzu, um sie auszudrucken
#[derive(Debug)]

// Definiere eine generische LinkedList-Struktur
pub struct LinkedList<T> {
    
    // head ist eine Option, die entweder einen generischen Knoten-Typen oder nichts (None) enthält
    head: Option<Box<Node<T>>>,
}

// Füge das Debug-Trait zu Node und LinkedList hinzu, um sie auszudrucken
#[derive(Debug)]

// Definiere eine generische Node-Struktur als Innere Struktur der LinkedList
struct Node<T> {

    // Jeder Knoten hat ein Element vom generischen Typen T
    elem: T,

    // Jeder Knoten hat einen nächsten Knoten, den es zulässt sich selbst auf einen anderen Knoten-Typen oder nichts zu referenzieren
    next: Option<Box<Node<T>>>,
}

// Implementiere Methoden für LinkedList
impl<T> LinkedList<T> {

    // Erstelle eine neue LinkedList mit leerem Kopf
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Hänge ein Element an den Beginn der Liste an
    pub fn push(&mut self, elem: T) {

        // Erstelle einen neuen Knoten, der das Element enthält und auf den aktuellen Kopf zeigt
        let new_node = Box::new(Node {
            elem: elem,

            // Der nächste Knoten ist der aktuelle Kopf
            next: self.head.take(),
        });

        // Setze den aktuellen Kopf auf den neu erstellten Knoten
        self.head = Some(new_node);
    }

    // Entferne das Element am Anfang der Liste und gib es zurück
    pub fn pop(&mut self) -> Option<T> {

        // Nehme den aktuellen Kopf und aktualisiere ihn dann auf seinen nächsten Knoten
        self.head.take().map(
            
            |node| {
                // Das nächste Element nach dem Kopfknoten wird zum neuen Kopfknoten
                self.head = node.next;

                // Gib das Element vom entfernten Knoten zurück
                node.elem
            }
        )
    }
}

fn main() {
    // Erstelle eine neue LinkedList
    let mut liste = LinkedList::new();

    // Füge Elemente in die Liste ein
    liste.push(1);
    liste.push(2);
    liste.push(3);
    liste.push(4);

    // Drucke die Liste aus
    println!("{:?}", liste);

    // Entferne ein Element aus der Liste
    match liste.pop() {
        Some(wert) => println!("Entfernter Wert: {}", wert),
        None => println!("Die Liste ist leer!"),
    };

    // Drucke die Liste nach dem Entfernen eines Elements erneut aus
    println!("{:?}", liste);
}