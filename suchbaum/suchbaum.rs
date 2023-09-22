// Importieren Sie das 'Ord' (Ordnen) Trait, um die '<' und '>' Operatoren verwenden zu können
use std::cmp::Ord;

// Definieren Sie eine Struktur 'Knoten' mit generischem Typ 'T'
#[derive(Debug)]

struct Knoten<T> {
    wert: T,
    links: Option<Box<Knoten<T>>>,
    rechts: Option<Box<Knoten<T>>>,
}

// Implementieren Sie Methoden für 'Knoten'
impl<T: Ord> Knoten<T> {

    // Eine Methode, um einen neuen 'Knoten' zu erstellen
    fn neu(wert: T) -> Knoten<T> {
        Knoten {
            wert: wert,
            links: None,
            rechts: None,
        }
    }

    // Eine Methode zum Einfügen neuer 'Knoten' in den BST
    fn einfügen(&mut self, wert: T) {

        if wert < self.wert {

            // Wenn der einzufügende Wert kleiner ist als der aktuelle Wert, fügen Sie ihn links ein
            if let Some(links) = &mut self.links {
                links.einfügen(wert);
            } else {
                // Wenn es keinen Knoten links gibt, erstellen Sie einen neuen Knoten mit dem Wert
                self.links = Some(Box::new(Knoten::neu(wert)));
            }

        } else if wert > self.wert {

            // Wenn der einzufügende Wert größer ist als der aktuelle Wert, fügen Sie ihn rechts ein
            if let Some(rechts) = &mut self.rechts {
                rechts.einfügen(wert);
            } else {
                // Wenn es keinen Knoten rechts gibt, erstellen Sie einen neuen Knoten mit dem Wert
                self.rechts = Some(Box::new(Knoten::neu(wert)));
            }
        }
        // Wenn der einzufügende Wert gleich dem aktuellen Wert ist, wird nichts eingefügt (keine Duplikate erlaubt)
    }

    // Eine Methode zum Prüfen, ob ein Wert im BST vorhanden ist
    fn enthält(&self, wert: T) -> bool {

        if wert < self.wert {

            // Wenn der gesuchte Wert kleiner ist als der aktuelle Wert, suchen Sie in der linken Unterstruktur
            if let Some(links) = &self.links {
                links.enthält(wert)
            } else {
                // Wenn es keinen Knoten links gibt, ist der Wert nicht im BST
                false
            }

        } else if wert > self.wert {

            // Wenn der gesuchte Wert größer ist als der aktuelle Wert, suchen Sie in der rechten Unterstruktur
            if let Some(rechts) = &self.rechts {
                rechts.enthält(wert)
            } else {
                // Wenn es keinen Knoten rechts gibt, ist der Wert nicht im BST
                false
            }

        } else {
            
            // Wenn der gesuchte Wert gleich dem aktuellen Wert ist, ist der Wert im BST
            true
        }
    }
}

fn main() {
    // Erstellen Sie einen neuen BST mit dem Knoten '50' als Wurzel
    let mut baum = Knoten::neu(50);

    // Fügen Sie weitere Werte zum BST hinzu
    baum.einfügen(25);
    baum.einfügen(75);
    baum.einfügen(12);
    baum.einfügen(37);
    baum.einfügen(60);
    baum.einfügen(87);
    baum.einfügen(63);
    
    // Überprüfen Sie, ob bestimmte Werte im BST vorhanden sind
    // Wenn sie es sind, wird eine Nachricht gedruckt
    if baum.enthält(60) {
        println!("60 wurde gefunden");
    }
    if !baum.enthält(100) {
        println!("100 wurde nicht gefunden");
    }
}
