def ist_primzahl(zahl):
    if zahl <= 1:
        return False  # Zahlen kleiner oder gleich 1 sind keine Primzahlen

    for i in range(2, int(zahl ** 0.5) + 1):
        if zahl % i == 0:
            return False  # Wenn zahl durch i ohne Rest teilbar ist, ist sie keine Primzahl

    return True  # Wenn keine Teiler gefunden wurden, ist es eine Primzahl

def finde_teiler(zahl):
    teiler = []
    for i in range(1, zahl + 1):
        if zahl % i == 0:
            teiler.append(i)
    return teiler

# Benutzereingabe lesen
zahl = int(input("Geben Sie eine Zahl ein: "))

# Überprüfen, ob es eine Primzahl ist und die Ausgabe entsprechend generieren
if ist_primzahl(zahl):
    print(zahl, "ist eine Primzahl.")
else:
    print(zahl, "ist keine Primzahl.")

# Teiler finden und ausgeben
teiler = finde_teiler(zahl)
print("Die Teiler von", zahl, "sind:", teiler)