Der ESP32 besitzt ein eingebautes Wifi-Modul. Das Wifi-Modul nutzt intern das Wifi -und Bluetoothradio. Diese Hardwarekomponente **muss** aktiviert werden. Anschließend muss entschieden werden, ob wir in STA oder AP-Modus schalten. Beide Modi haben Vor -und Nachteile, sodass oftmals eine STA/AP-Kombination gewählt wird.
In unserem Fall haben wir nur ein sehr eingeschränktes Nutzungsspektrum. Weshalb wir keine Kombination verwenden werden. 

| Modus | Anwendung                                                                                                                                                                               |
| ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AP    | Der ESP32 stellt ein eigenes WLAN-Netzwerk bereit, mit dem sich andere Geräte verbinden können. Nützlich für lokale Steuerungen oder wenn keine bestehende Infrastruktur vorhanden ist. |
| STA   | Der ESP32 verbindet sich mit einem bestehenden WLAN-Netzwerk, um Daten mit anderen Geräten oder dem Internet auszutauschen. Geeignet für IoT-Anwendungen und Cloud-Kommunikation.       |
In unserem Fall wollen wir, dass die Drohne unabhängig von Netzempfang in der Natur funktioniert. Wichtig hierbei wird also die AP-Funktionalität sein. Die Drohne sollte ihr eigenes WLAN-Netzwerk bereitstellen. 

Um so etwas zu vollbringen, müssen wir die Drohne einrichten. Wir belassen viele Einstellungen bei den Standardeinstellungen.

| AP-Einstellungen  | Werte                    |
| ----------------- | ------------------------ |
| SSID              | Flightcontroller         |
| Password          | FlightcontrollerPassword |
| Sicherheit        | WPA2Personal             |
| max. Verbindungen | 2                        |

Das ESP-Wifi nutzt statische IP-Adressen. Aus diesem Grund müssen die Wifi-Einstellungen von dem Endgerät auch angepasst werden.

| Wifi-Einstellungen | Werte                        |
| ------------------ | ---------------------------- |
| IP                 | Statisch (Standard ist DHCP) |
| IP-Adresse         | 192.168.2.x (1 < x < 256)    |

## Anwendung

Das ESPWifi-Modul muss erst kreiert werden.
```rust
let mut wifi: Wifi<Uninit> = Wifi::new(timg0.timer0, peripherals.RNG, peripherals.RADIO_CLK).unwrap();
```

Das ESPWifi-Modul muss anschließend initialisiert werden. Bei der Initialisierung werden SSID und Password gesetzt und die maximale Anzahl an zugelassenen Verbindungen.
```rust
wifi.init(peripherals.WIFI, "Flightcontroller", "FlightcontrollerPassword", 2).unwrap()
```
## Error

Das ESPWifi-Modul kann auf verschiedene Weisen verfehlen.

| Error                   | Grund                                                    |
| ----------------------- | -------------------------------------------------------- |
| WifiRadioInitialization | Wifi -und Bluetoothradioaktivierung fehlerhaft           |
| AccessPointConfig       | Access Point Konfiguration fehlerhaft in Initialisierung |
| StartAP                 | Access Point start fehlerhaft in Initialisierung         |
| UTF8Parsing             | UTF8-String beinhaltet nicht ASCII-Code                  |



## UDP Layer