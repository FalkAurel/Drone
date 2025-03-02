Dieses Modul dokumentiert die Implementierung einer eigenen Speicherverwaltung. Die Speicherverwaltung wird als [linked list alloctator](https://os.phil-opp.com/allocator-designs/#linked-list-allocator) implementiert. Weiterführend wird das ESP32 Technical Reference Manual ver. 5.3 verwendet. 

![[datasheets/ESP32_technical_reference.pdf]]


## Linked List Allocator

### Grundlegende Überlegungen

Der Linked List Allocator nutzt eine linked List, um die freien Speicherregionen zu verwalten. Die Idee ist, dass der Speicher sich selbst verwaltet, indem jeder Block auf den nächsten freien Block zeigt. Sobald kein Block mehr auf einen anderen zeigt, muss der Speicher voll sein. 

Zunächst beginnt man mit einem Block, der den ganzen Speicher abdeckt. Sobald Speicher benötigt wird, wird aus dem Block ein kleiner rausgeschnitten, der nun reserviert für eine Ressource genutzt wird. Das *"Rausschneiden"* beginnt immer am Anfang, sodass der Kopf der linked List auf immer auf den ersten freien Block zeigt.

Bei der Freigabe der speicherkonsumierenden Ressource wird auch der Speicher an den Allocator zurückgegeben. Das zurückgeben kann auf verschiedene Weisen funktionieren. Mann könnte versuchen den Block an seinen ursprünglichen Platz einzufügen oder ihn einfach an den Anfang der `free list` schieben. Beides hat Vor -und Nachteile.
Ersteres wäre der naive Ansatz, der theoretisch weniger Speicher pro Node header verbraucht, da die absolute Position nicht definiert werden muss, sondern durch die relative Lage in der `free list` bestimmt werden kann. Dieser Vorteil kommt auf Kosten der Laufzeit, da bei jedem `free` eine konstante Suche laufen muss (Theoretisch ist es eine O(*n*) Operation, aber da die Größe der `free_list` bereits durch den ersten Block definiert ist, hat sie eine feste obere Grenze, die sich nicht mehr verändern kann (definiert durch die Speichergröße)).
Wenn man aber jedem Block seine Adresse neben der Größe mitgibt (system width integer), dann kann man den Block einfach an den Anfang anhängen, sodass die Operation O(1) ist. Aus diesem Grund werden wir den letzteren Ansatz implementieren.

### Speicherfragmentierung

Jeder der eine linked List implementiert hat, weiß, dass sie kein kontinuierliches Speichersegment nutzt, sondern fragmentartig im Speicher lebt. Das ist zugleich die Stärke und Schwäche dieser Datenstruktur, es ermöglicht uns Sachen zu speichern, die nicht mehr in einem Stück in den Speicher passen, allerdings fragmentiert es auch den Speicher. Ein fragmentierter Speicher könnte theoretisch noch Elemente speichern, hätte er genug kontinuierlichen Speicher am Stück.
Die Fragmentierung hat zwei Ursachen: Alignment und unterschiedliche Objekte mit unterschiedlichen Lebenszeiten. 
Alignment ist nicht (leicht) zu beheben. Moderene Prozessoren sind darauf ausgelegt Daten von Adressen auszulesen, die ein vielfaches von 2 sind. Aus diesem Grund werden wir nicht dieses Problem angehen. 
Die Speicherfragmentierung durch Objekte mit verschiedenen Lebenszeit können wir softwaretechnisch angehen. Wir können nicht die Lebenszeiten der Objekte bestimmen, aber wir können die freigegebenen Speicherblöcke manipulieren. Wenn man zwei oder mehr Blöcke findet, die ein kontinuierliches Speichersegment beschreiben, kann man diese Blöcke in einen größeren zusammenführen. Eine weitere Vorgehensweise ist, `best-fit` Allokation zu nutzen. Wir suchen die ganze `free list` nach dem kleinsten passenden Block ab. Diese Optimierung sollte aber optional sein, da sie nicht effektiv in einem hot codepath ist.

