# Passwort-Manager

## Liste mit abzudeckenden Handlungszielen

### 165 NoSQL-Datenbanken einsetzen

- Wählt eine für den Anwendungsfall geeignete NoSQL-Datenbank aus (z.B. Document-Store, Graphen-basiert, Key-Value-Store, Wide-Column-Store, Objekt-orientiert, in-Memory).

- Implementiert eine NoSQL-Datenbank, befüllt sie mit Daten.

- Definiert Zugriffsberechtigungen und setzt diese in der NoSQL-Datenbank um.

- Sichert eine NoSQL-Datenbank in einem Backup und prüft die Wiederherstellung.

- Skaliert eine NoSQL-Datenbank, z.B. durch Replikation.

- Nutzt die NoSQL-Datenbank lesend und schreibend aus einer Anwendung.



### 450 Applikationen testen

- Erstellt ein Testkonzept anhand von einem Praxisbeispiel inklusive Anforderungen (Testbasis).

- Beschreibt ein Testumfeld umfassend.

- Definiert innerhalb verschiedenen Teststufen/-arten (Unit Tests, Integrationstests, E2E-Tests, Systemtests, Last-/ Performancetests, Security-Tests, Benutzerakzeptanztests, Konformitätstests) Tests und deren Testmittel.

- Erarbeitet Vorschläge zur Verbesserung von Mängeln innerhalb von Code Reviews.

- Beschreibt aufgrund von Anforderungen wiederholbare Testfälle inklusive der erwarteten Resultate.

- Implementiert und führt automatisierte Testfälle aus und dokumentiert dessen Resultate nachvollziehbar.

- Definiert anhand von beobachteten/gegebenen Abweichungen Korrekturmassnahmen und setzt diese um (z.B. innerhalb Test Driven Development TDD).

- Testet gemäss Sicherheitskonzept Schnittstellen.



### 320 Objektorientiert Programmieren

- Analysiert Anwendungsprobleme zur Erstellung von objektorientierten Programmen. [g4.1, g4.4]

- Modelliert und dokumentiert objektorientierte Programme. [g4.4]

- Implementiert objektorientiertes Design. [g5.2, g5.5]

- Überprüft die Implementierung auf Korrektheit und Qualität. [g5.4, g6.3, g6.5, g6.6]



### 347 Dienst mit Container anwenden

- Identifiziert Auswirkungen von Virtualisierung auf den beruflichen Alltag.

- Wählt eine geeignete Containerkomposition (Architektur) situationsbezogen aus. [h1.1, h1.2, h1.4]

- Wählt geeignete Containerdienstleister gemäss Anforderungen aus. [h1.5, h1.6]

- Virtualisiert eine Applikation mit der gewählten Containerkomposition sowohl zu Entwicklungszwecken als auch für Auslieferung/Betrieb. [h3.1, h3.2, h3.4, h3.5]

- Plant Methoden zur Qualitätskontrolle und setzt diese um. [h3.7]

### 346 Cloud Lösungen konzipieren und realisieren

- Beurteilt die Eignung von On-Premise und Cloudlösungen abgestimmt auf die Zielsetzungen des Unternehmens und leitet draus eine Empfehlung für die Umsetzung ab.

- Spezifiziert die Kosten einschliesslich Betriebsaufwand der vorgeschlagenen Lösung und bestimmt die zweckmässige Cloud Adoption.

- Entwickelt unter Berücksichtigung der technischen Rahmenbedingungen und Anforderungen des entsprechenden Anwendungsbereichs ein technisches Konzept für die Integration der ausgewählten Cloudlösung.

- Installiert und konfiguriert die vordefinierten Services in der Cloud.

## 

## Projektbeschreibung

Das Projekt besteht aus der Entwicklung eines Passwortmanagers, in der Sprache Rust geschrieben. Das GUI wird mit dem Iced-Framework umgesetzt. Ziel ist es eine Software zu entwickeln, in welcher Passwörter lokal und in der Cloud verwaltet werden können. Dabei werden moderne Softwarearchitekturprinzipien, Sicherheit, Testbarkeit und Skalierbarkeit berücksichtigt. Der Dienst soll containerisiert betrieben werden können.

## ToDos

#### 1. Projektplanung & Konzeption

##### 1.1 Anforderungsanalyse

- [ ] Detaillierte Anforderungen definieren (funktional & nicht-funktional)
  - **Handlungsziel:** 450 Applikationen testen (Testbasis erstellen)
- [ ] Use Cases und User Stories dokumentieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Anwendungsprobleme analysieren)

##### 1.2 Cloud-Strategie

- [ ] On-Premise vs. Cloud-Lösung evaluieren und Empfehlung ableiten
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (Eignung beurteilen)
- [ ] Kostenanalyse inkl. Betriebsaufwand erstellen
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (Kosten spezifizieren)
- [ ] Cloud Adoption Strategie festlegen
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (Cloud Adoption bestimmen)

#### 2. Architektur & Design

##### 2.1 Systemarchitektur

- [ ] Technisches Konzept für Cloud-Integration entwickeln
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (technisches Konzept entwickeln)
- [ ] Geeignete Container-Architektur auswählen
  - **Handlungsziel:** 347 Dienst mit Container (Containerkomposition auswählen)
- [ ] Objektorientierten Design modellieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (modellieren und dokumentieren)

##### 2.2 Datenbank-Design

- [ ] Geeignete NoSQL-Datenbank auswählen (empfohlen: Document-Store wie MongoDB oder Key-Value-Store)
  - **Handlungsziel:** 165 NoSQL-Datenbanken (geeignete Datenbank auswählen)
- [ ] Datenbankschema für Passwort-Einträge entwerfen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank implementieren)
- [ ] Zugriffsberechtigungen konzipieren
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Zugriffsberechtigungen definieren)

#### 3. Entwicklungsumgebung & Setup

##### 3.1 Entwicklungsumgebung

- [ ] Rust-Entwicklungsumgebung aufsetzen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Implementation)
- [ ] Iced-Framework integrieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Implementation)
- [ ] Testumfeld definieren und aufsetzen
  - **Handlungsziel:** 450 Applikationen testen (Testumfeld beschreiben)

##### 3.2 NoSQL-Datenbank Setup

- [ ] NoSQL-Datenbank lokal installieren und konfigurieren
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank implementieren)
- [ ] Testdaten für Entwicklung erstellen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank mit Daten befüllen)

#### 4. Core-Funktionalität (Backend)

##### 4.1 Datenbank-Integration

- [ ] Rust-Treiber für NoSQL-Datenbank implementieren
  - **Handlungsziel:** 165 NoSQL-Datenbanken (lesend und schreibend nutzen)
- [ ] CRUD-Operationen für Passwort-Einträge entwickeln
  - **Handlungsziel:** 165 NoSQL-Datenbanken (lesend und schreibend nutzen)
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Zugriffsberechtigungen in der Datenbank umsetzen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Zugriffsberechtigungen umsetzen)

##### 4.2 Verschlüsselung & Sicherheit

- [ ] Verschlüsselungslogik für Passwörter implementieren (AES-256)
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Master-Passwort-System entwickeln
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Sicherheitskonzept für Schnittstellen erstellen
  - **Handlungsziel:** 450 Applikationen testen (Schnittstellen gemäss Sicherheitskonzept testen)

##### 4.3 Synchronisations-Logik

- [ ] Lokale Speicherung implementieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Cloud-Synchronisation entwickeln
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Konfliktauflösung bei Sync implementieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)

#### 5. GUI-Entwicklung (Frontend)

##### 5.1 Iced-GUI Komponenten

- [ ] Login-Screen mit Master-Passwort entwickeln
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Hauptansicht mit Passwort-Liste erstellen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Formular zum Hinzufügen/Bearbeiten von Passwörtern
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Suchfunktion implementieren
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)
- [ ] Einstellungen-Dialog (Cloud-Konfiguration, Backup, etc.)
  - **Handlungsziel:** 320 Objektorientiert Programmieren (objektorientiertes Design implementieren)

#### 6. Testing

##### 6.1 Testkonzept

- [ ] Testkonzept mit allen Teststufen erstellen
  - **Handlungsziel:** 450 Applikationen testen (Testkonzept erstellen)
- [ ] Testfälle basierend auf Anforderungen definieren
  - **Handlungsziel:** 450 Applikationen testen (wiederholbare Testfälle beschreiben)

##### 6.2 Unit Tests

- [ ] Unit Tests für Verschlüsselungslogik schreiben
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Unit Tests)
- [ ] Unit Tests für Datenbank-Operationen schreiben
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Unit Tests)
- [ ] Unit Tests für Business-Logik implementieren
  - **Handlungsziel:** 450 Applikationen testen (automatisierte Testfälle implementieren)

##### 6.3 Integrationstests

- [ ] Integrationstests für Datenbank-Integration schreiben
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Integrationstests)
- [ ] Integrationstests für Cloud-Synchronisation
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Integrationstests)

##### 6.4 E2E & System Tests

- [ ] End-to-End Tests für komplette User-Flows
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - E2E-Tests)
- [ ] Systemtests durchführen
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Systemtests)

##### 6.5 Security Tests

- [ ] Security Tests für Verschlüsselung durchführen
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Security-Tests)
- [ ] Schnittstellentests gemäss Sicherheitskonzept
  - **Handlungsziel:** 450 Applikationen testen (Schnittstellen gemäss Sicherheitskonzept testen)
- [ ] Penetrationstests für API-Endpoints
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Security-Tests)

##### 6.6 Performance & Last Tests

- [ ] Performance-Tests für Datenbank-Operationen
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Last-/Performancetests)
- [ ] Last-Tests für Skalierbarkeit
  - **Handlungsziel:** 450 Applikationen testen (Tests definieren - Last-/Performancetests)

##### 6.7 Code Quality

- [ ] Code Reviews durchführen
  - **Handlungsziel:** 450 Applikationen testen (Verbesserungsvorschläge erarbeiten)
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Implementierung überprüfen)
- [ ] Testresultate dokumentieren
  - **Handlungsziel:** 450 Applikationen testen (Resultate nachvollziehbar dokumentieren)
- [ ] TDD-Zyklus: Korrekturmassnahmen definieren und umsetzen
  - **Handlungsziel:** 450 Applikationen testen (Korrekturmassnahmen definieren und umsetzen)

#### 7. Containerisierung

##### 7.1 Docker Setup

- [ ] Auswirkungen von Virtualisierung auf Projekt analysieren
  - **Handlungsziel:** 347 Dienst mit Container (Auswirkungen identifizieren)
- [ ] Dockerfile für Rust-Anwendung erstellen
  - **Handlungsziel:** 347 Dienst mit Container (Applikation virtualisieren)
- [ ] Docker-Compose für Multi-Container-Setup (App + DB)
  - **Handlungsziel:** 347 Dienst mit Container (Applikation virtualisieren)

##### 7.2 Container-Dienstleister

- [ ] Geeigneten Container-Dienstleister auswählen (Docker Hub, AWS ECR, etc.)
  - **Handlungsziel:** 347 Dienst mit Container (Containerdienstleister auswählen)
- [ ] Container-Registry konfigurieren
  - **Handlungsziel:** 347 Dienst mit Container (Containerdienstleister auswählen)

##### 7.3 Qualitätskontrolle

- [ ] Methoden zur Container-Qualitätskontrolle planen
  - **Handlungsziel:** 347 Dienst mit Container (Qualitätskontrolle planen)
- [ ] Security-Scans für Container-Images durchführen
  - **Handlungsziel:** 347 Dienst mit Container (Qualitätskontrolle umsetzen)

#### 8. Cloud-Integration

##### 8.1 Cloud-Infrastruktur

- [ ] Cloud-Services auswählen (AWS, Azure, Google Cloud)
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (Cloud Adoption bestimmen)
- [ ] Cloud-Services installieren und konfigurieren
  - **Handlungsziel:** 346 Cloud Lösungen konzipieren (Services installieren und konfigurieren)
- [ ] Cloud-Datenbank für Synchronisation einrichten
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank implementieren)

##### 8.2 Deployment

- [ ] CI/CD-Pipeline für Cloud-Deployment aufsetzen
  - **Handlungsziel:** 347 Dienst mit Container (für Auslieferung/Betrieb virtualisieren)
- [ ] Container in Cloud deployen
  - **Handlungsziel:** 347 Dienst mit Container (für Auslieferung/Betrieb virtualisieren)

#### 9. Backup & Skalierung

##### 9.1 Backup-Strategie

- [ ] Backup-Konzept für NoSQL-Datenbank erstellen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank in Backup sichern)
- [ ] Automatische Backup-Lösung implementieren
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank in Backup sichern)
- [ ] Wiederherstellung aus Backup testen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Wiederherstellung prüfen)

##### 9.2 Skalierung

- [ ] Replikations-Strategie für NoSQL-Datenbank planen
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank skalieren)
- [ ] Datenbank-Replikation implementieren
  - **Handlungsziel:** 165 NoSQL-Datenbanken (Datenbank skalieren)
- [ ] Load Balancing für Container einrichten
  - **Handlungsziel:** 347 Dienst mit Container (Containerkomposition auswählen)

#### 10. Benutzerakzeptanz & Dokumentation

##### 10.1 Benutzerakzeptanztests

- [ ] Benutzerakzeptanztests durchführen
  - **Handlungsziel:** 450 Applikationen testen (Benutzerakzeptanztests definieren)
- [ ] Konformitätstests durchführen
  - **Handlungsziel:** 450 Applikationen testen (Konformitätstests definieren)

##### 10.2 Dokumentation

- [ ] Technische Dokumentation erstellen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (modellieren und dokumentieren)
- [ ] Benutzerhandbuch schreiben
  - **Handlungsziel:** 320 Objektorientiert Programmieren (dokumentieren)
- [ ] API-Dokumentation erstellen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (dokumentieren)
- [ ] Deployment-Guide verfassen
  - **Handlungsziel:** 347 Dienst mit Container (dokumentieren)

#### 11. Abschluss

##### 11.1 Qualitätssicherung

- [ ] Finale Code-Review durchführen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Implementierung überprüfen)
- [ ] Alle Tests erneut durchführen
  - **Handlungsziel:** 450 Applikationen testen (automatisierte Testfälle ausführen)
- [ ] Performance-Optimierungen vornehmen
  - **Handlungsziel:** 320 Objektorientiert Programmieren (Implementierung überprüfen)

##### 11.2 Projektabschluss

- [ ] Projektdokumentation finalisieren
- [ ] Präsentation vorbereiten
- [ ] Lessons Learned dokumentieren






