// Unsere Funktion namens `average` nimmt einen Iterator `values` vom generischen Typ 'T'
// die Elemente des Iterators sind vom generischen Typ 'U'
 
fn average<T, U>(values: T) -> U
    // Typbeschränkungen für T und U
where
  // T ist ein Iterator mit Elementen vom Typ U
  T: Iterator<Item = U>,
  
  // U ist eine Art von nummerischen Daten (Kann addiert und dividiert werden) und hat die Fähigkeit, von einer u8 konvertiert zu werden.
  // U braucht auch die Fähigkeit, kopiert zu werden (da wir die Originalwerte nicht verbrauchen wollen)
  // Default wird benötigt, um einen Ausgangswert für die Summe zu bieten (0 für numerische Typen)
  U: std::ops::Add<Output = U> + std::ops::Div<Output = U> + Copy + From<u8> + Default,
{
  // Die fold-Funktion wird benutzt, um durch die Werte des Iterators zu iterieren und Werte zu akkumulieren
  // Die fold-Funktion beginnt mit einem Anfangswert (in unserem Fall das Paar von Summe und Anzahl, die beide 0 sind)
  // und wendet eine Funktion an, die das aktuelle Paar und den aktuellen Wert nimmt 
  // und ein neues Paar erzeugt, das die akkumulierte Summe und Anzahl enthält
  let (sum, count) = values.fold((U::default(), U::from(0)), 
    
|(sum, count), value| {
        // In jedem Schritt erhöhen wir die Summe um den Wert und die Anzahl um 1 
        (sum + value, count + U::from(1))
  });

  // Der Durchschnitt wird berechnet als die Summe dividiert durch die Anzahl
  sum / count
}


// Verwendung der `average` Funktion
fn main() {
  // Definition einer Liste von nummerischen Werten
  let nums = [1, 2, 3, 4, 5];
  
  // Aufruf der Funktion `average` mit einer Kopie des Iterators über nums
  // Die Kopie wird benötigt, da die Funktion den Iterator verbrauchen würde, wenn wir den originalen Iterator übergeben
  let avg = average(nums.iter().copied());
  
  // Ausgabe des berechneten Durchschnitts
  println!("Durchschnitt: {}", avg);
}
