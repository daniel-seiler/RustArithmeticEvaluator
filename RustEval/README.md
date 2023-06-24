# Arithmetische Ausdrücke in Rust

## Darstellung des Haskell-Datentyps in Rust

In haskell werden die Datentypen mithilfe des keywords 'data' initialisiert: Die Datentypen werden in Rust hier mithilfe eines Enums namens 'Token' repräsentiert. Dieser besitzt verschiedene Typen:
- *Int*: repräsentiert ein Integer Wert durch das Feld 'val'
- *Plus*, *Minus*, *Mult*, *Div*, *Mod* repräsentieren die mathematische Operatoren (+, -, *, /, %) und enthalten zwei Token (linker und rechter Operand) als Felder. Der linke und der rechte Operand werden als Box\<Token> gespeichert und können somit Werte des Datentyps 'Token' enthalten.
 
## Implementierung des Evaluators in Rust einschließlich Vereinfachung

### Vereinfachung:

Die Methode *simplify(e: Token) -> Token* bekommt einen Eingabetoken welcher vereinfacht werden soll, ohne die arithmetik zu verändern. Die Rekursion läuft erst von der Wurzel zu den Blättern, bis nur noch *Int* Token gefunden werden. Dann findet jeweils die Vereinfachung statt. Der vereinfachte Wurzel-Token wird wieder von der Funktion zurückgegeben. Die folgenden Regeln werden bei der Vereinfachung angewandt:

#### Mult

1. linke Seite der Multiplikation ist 0, dann wird für die gesamte Multiplikation 0 zurückgegeben ```0 * 5 = 0```
2. rechte Seite der Multiplikation ist 0, dann wird für die gesamte Multiplikation 0 zurückgegeben ```5 * 0 = 0```
3. linke Seite der Multiplikation ist 1, dann wird nur die rechte Seite der Multiplikation zurückgegeben ``1 * 5 = 5``
4. rechte Seite der Multiplikation ist 1, dann wird nur die linke Seite der Multiplikation zurückgegeben ``5 * 1 = 5``

#### Div

1. linke Seite der Division ist 0, dann wird für die gesamte Division 0 zurückgegeben ``0 / 5 = 0``
2. rechte Seite der Division ist 1, dann wird nur die linke Seite der Division zurückgegeben ``5 / 1 = 5``

#### Plus

1. linke Seite der Addition ist 0, dann wird nur die rechte Seite der Addition zurückgegeben ``0 + 5 = 5``
2. rechte Seite der Addition ist 0, dann wird nur die linke Seite der Addition zurückgegeben ``5 + 0 = 5``

#### Minus

1. rechte Seite der Subtraktion ist 0, dann wird nur die linke Seite der Subtraktion zurückgegeben ``5 - 0 = 5``

## Ziehen Sie einen Vergleich zwischen Haskell und Rust

### Datentypen:

#### Haskell:

Haskell verfügt über ein Typsystem, das Typinferenz größtenteils unterstützt, sowie ADT (Algebraic data types), dass Datentypen aus summieren oder ver-odern zusammenbaut. Es ist einfach auf diesen durch das keyword 'data' definierten Datentypen Funktionen für Typklassen und Pattern Matching zu schreiben. 

#### Rust

Rust verfügt über ein starkes Typsystem, geht jedoch im Vergleich zu Haskell anders vor. Es verwendet eine Kombination aus algebraischen Datentypen (Enums und Structs) und Traits (ähnlich wie Schnittstellen), um Datenstrukturen und Verhalten zu definieren. Rust legt Wert auf Sicherheit, Speicherverwaltung und feingranulare Kontrolle.

### Arithmetische Evaluatoren:

#### Haskell

Haskell eignet sich aufgrund seiner funktionalen Programmierweise sehr gut für den Aufbau arithmetischer Evaluatoren. Es unterstützt Pattern Matching, Rekursion und Funktionen höherer Ordnung. Wie in der Vorlesung gesehen lassen sich durch das Keyword *eval* einfach und mit wenig Zeichen evaluatoren zusammenstellen.

#### Rust

Rust ist als systemnahe Programmiersprache darauf ausgerichtet, Kontrolle und Performance zu bieten, erfordert jedoch explizites Handling von Speicher und Mutabilität. Durch die Verwendung von Enums als Datentypen, sowie Rekursive Methoden bei der Evaluierung, lassen sich in Rust mit mehr Zeilen Code typsichere Evaluatoren bauen.

### Unterschiede:

- Haskell legt einen stärkeren Fokus auf funktionale Programmierparadigmen und Lazy Evaluation, was zu prägnanterem und ausdrucksstärkerem Code führt. Rust hingegen konzentriert sich auf Performance, Speichersicherheit und explizite Kontrolle, wodurch es für systemnahe Programmierung geeignet ist.


- Das Typsystem und die Typinferenz in Haskell sind fortgeschrittener, was ausdrucksstarke und prägnante Typdefinitionen ermöglicht. Das Typsystem von Rust ist zwar leistungsfähig, erfordert jedoch mehr explizite Annotationen.

### Gemeinsamkeiten:

- Sowohl Haskell als auch Rust unterstützen algebraische Datentypen (Data, Enums) und Pattern Matching (eval, match), was die Definition und Verarbeitung solcher Daten als Tokens einfach ermöglicht.


- Haskell und Rust verfügen über ausdrucksstarke Typsysteme, die dazu beitragen, typbezogene Fehler bereits zur Kompilierzeit zu erkennen und Laufzeitfehler zu reduzieren.