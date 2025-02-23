## Überblick

[ESP32 mit CH340 Entwicklungsplatine](https://www.amazon.de/Entwicklungsplatine-Binghe-ESP-WROOM-32-Bluetooth-kompatibel/dp/B0D8635YZ6/ref=sr_1_9?__mk_de_DE=ÅMÅŽÕÑ&crid=U6S6FM8RXKHQ&dib=eyJ2IjoiMSJ9.sNY-5N5qC7Qc40Mxm97ZN4weJ778AWoug6y_-HIs7zJnUx6G5a_Cr7ui9T0AqxXx3C38nRJsddtw9oVz7HkEjisz5nsbp02URqzqz5KKdO1C33Xk-TpJYW610M7W6_PBUIUInRioKLPqgdexH6gLK1X_Ba-g42B5osPiGoGjxDhPiUmD_AIm_dKic2TON03oMuE5Va2wzURmNc_jQgbxTkYLvc-ipVYjj0HAakPXDNg.EPnLuabGVGk9OuytN7jRzqsqm6UfvlIYVpuc6GJIAZM&dib_tag=se&keywords=ESP32+ESP-32S+Development+Board+%283pcs%29&qid=1739624362&sprefix=esp32+esp-32s+development+board+3pcs+%2Caps%2C102&sr=8-9) ist ein Chip mit WiFi -und Bluetoothanbindung. Intern wird ein [ESP32-DOWDQ6-Chip](https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf) verwendet.
Die technischen Hardwaredaten des Chips sind hier gelistet:

| Item                                      | Spezifikation                                  |
| ----------------------------------------- | ---------------------------------------------- |
| Integrierter Kristall                     | 40Mhz                                          |
| SPI flash                                 | 4MB                                            |
| Operating voltage / Power supply          | 3.0 ~3.6V                                      |
| Operating Current                         | Average: 80mA                                  |
| Minimum current delivered by power supply | 500mA                                          |
| Package size                              | 18mm x 25.5mm x 3.10mm                         |
| CPU Architecture                          | dual-core [Xtensa]() 32-bit LX6 microprocessor |

Weitere Daten sind im Datenblatt zu finden, für die jetzige Implementation langen die Daten.


## Pin Definitionen

![[ESP32-Wroom-32-Pinout.jpg]]

| Name      | No. | Type | Function                                                                               |
| --------- | --- | ---- | -------------------------------------------------------------------------------------- |
| GND       | 1   | P    | Ground                                                                                 |
| 3V3       | 2   | P    | Power supply                                                                           |
| EN        | 3   | I    | Module-enable signal. Active high                                                      |
| SENSOR_VP | 4   | I    | GPIO36, ADC1_CH0, RTC_GPIO0                                                            |
| SENSOR_VN | 5   | I    | GPIO39, ADC1_CH3, RTC_GPIO3                                                            |
| IO34      | 6   | I    | GPIO34, ADC1_CH6, RTC_GPIO4                                                            |
| IO35      | 7   | I    | GPIO35, ADC1_CH7, RTC_GPIO5                                                            |
| IO32      | 8   | I/O  | GPIO32, XTAL_32K_P (32.768 kHz crystal oscillator input), ADC1_CH4, TOUCH9, RTC_GPIO9  |
| IO33      | 9   | I/O  | GPIO33, XTAL_32K_N (32.768 kHz crystal oscillator output), ADC1_CH5, TOUCH8, RTC_GPIO8 |
| IO25      | 10  | I/O  | GPIO25, DAC_1, ADC2_CH8, RTC_GPIO6, EMAC_RXD0                                          |
| IO26      | 11  | I/O  | GPIO26, DAC_2, ADC2_CH9, RTC_GPIO7, EMAC_RXD1                                          |
| IO27      | 12  | I/O  | GPIO27, ADC2_CH7, TOUCH7, RTC_GPIO17, EMAC_RX_DV                                       |
| IO14      | 13  | I/O  | GPIO14, ADC2_CH6, TOUCH6, RTC_GPIO16, MTMS, HSPICLK, HS2_CLK, SD_CLK, EMAC_TXD2        |
| IO12      | 14  | I/O  | GPIO12, ADC2_CH5, TOUCH5, RTC_GPIO15, MTDI, HSPIQ, HS2_DATA2, SD_DATA2, EMAC_TXD3      |
| GND       | 15  | P    | Ground                                                                                 |
| IO13      | 16  | I/O  | GPIO13, ADC2_CH4, TOUCH4, RTC_GPIO14, MTCK, HSPID, HS2_DATA3, SD_DATA3, EMAC_RX_ER     |
| SHD/SD2*  | 17  | I/O  | GPIO9, SD_DATA2, SPIHD, HS1_DATA2, U1RXD                                               |
| SWP/SD3*  | 18  | I/O  | GPIO10, SD_DATA3, SPIWP, HS1_DATA3, U1TXD                                              |
| SCS/CMD*  | 19  | I/O  | GPIO11, SD_CMD, SPICS0, HS1_CMD, U1RTS                                                 |
| SCK/CLK*  | 20  | I/O  | GPIO6, SD_CLK, SPICLK, HS1_CLK, U1CTS                                                  |
| SDO/SD0*  | 21  | I/O  | GPIO7, SD_DATA0, SPIQ, HS1_DATA0, U2RTS                                                |
| SDI/SD1*  | 22  | I/O  | GPIO8, SD_DATA1, SPID, HS1_DATA1, U2CTS                                                |
| IO15      | 23  | I/O  | GPIO15, ADC2_CH3, TOUCH3, MTDO, HSPICS0, RTC_GPIO13, HS2_CMD, SD_CMD, EMAC_RXD3        |
| IO2       | 24  | I/O  | GPIO2, ADC2_CH2, TOUCH2, RTC_GPIO12, HSPIWP, HS2_DATA0, SD_DATA0                       |
| IO0       | 25  | I/O  | GPIO0, ADC2_CH1, TOUCH1, RTC_GPIO11, CLK_OUT1, EMAC_TX_CLK                             |
| IO4       | 26  | I/O  | GPIO4, ADC2_CH0, TOUCH0, RTC_GPIO10, HSPIHD, HS2_DATA1, SD_DATA1, EMAC_TX_ER           |
| IO16      | 27  | I/O  | GPIO16, HS1_DATA4, U2RXD, EMAC_CLK_OUT                                                 |
| IO17      | 28  | I/O  | GPIO17, HS1_DATA5, U2TXD, EMAC_CLK_OUT_180                                             |
| IO5       | 29  | I/O  | GPIO5, VSPICS0, HS1_DATA6, EMAC_RX_CLK                                                 |
| IO18      | 30  | I/O  | GPIO18, VSPICLK, HS1_DATA7                                                             |
| IO19      | 31  | I/O  | GPIO19, VSPIQ, U0CTS, EMAC_TXD0                                                        |
| NC        | 32  | -    | -                                                                                      |
| IO21      | 33  | I/O  | GPIO21, VSPIHD, EMAC_TX_EN                                                             |
| RXD0      | 34  | I/O  | GPIO3, U0RXD, CLK_OUT2                                                                 |
| TXD0      | 35  | I/O  | GPIO1, U0TXD, CLK_OUT3, EMAC_RXD2                                                      |
| IO22      | 36  | I/O  | GPIO22, VSPIWP, U0RTS, EMAC_TXD1                                                       |
| IO23      | 37  | I/O  | GPIO23, VSPID, HS1_STROBE                                                              |
| GND       | 38  | P    | Ground                                                                                 |


## Einrichten einer Rust-Entwicklungsumgebung

Der ESP32-WROOM kann in Rust sowohl in `no_std` als auch in `std` beschrieben werden. Die Besonderheit liegt in der Leistung und Entwicklungsgeschwindigkeit. Für schnelle, rapide Entwicklung eignet sich die `std`-Version, während `no_std` für Leistung gedacht ist.
Für dieses Projekt nutzen wir erst die `std`-Version, sollte irgendwann Leistung ein Problem werden, werden wir den Code umschreiben in die `no_std`-Version.
In beiden Fällen müssen wir für die [Xtensa](https://dl.espressif.com/github_assets/espressif/xtensa-isa-doc/releases/download/latest/Xtensa.pdf)-Architektur kompilieren. Dafür nutzt man einen [Cross-Compiler](https://de.wikipedia.org/wiki/Cross-Compiler). Rust kann das automatisch, da es LLVM als Compiler-Backend verwendet, weshalb wir nur noch die Architektur spezifizieren müssen.
Weiterhin werden wir die virtuelle Umgebung [Docker](https://www.docker.com) verwenden, da ich die Toolchain nicht  auf meinem System einrichten kann und man am Ende nur noch das Docker Image entfernen muss, wenn man mit dem Projekt fertig ist.

### Installieren von Rust Templates

Mit den folgenden Modulen lassen sich Rust-Projekt-Templates generieren. Das ist optional, jedoch empfohlen, da diese eine lokale Toolchain spezifizieren. 

- [cargo-generate](https://crates.io/crates/cargo-generate) Templategenerator. Nutzt bereits git repositories, um ein Projekttemplate zu generieren
- [espflash](https://github.com/esp-rs/espflash) ermöglicht das Brennen und Löschen von Code. Weiterhin kann es den Serial-Port in der Konsole ausgeben
- [esp-generate](https://github.com/esp-rs/esp-generate) Templategenerator für `no_std`

```bash
cargo install cargo-generate espflash esp-generate
```


Es gibt zwei Möglichkeiten, um ein Projekt zu generieren:
1. Man nutzt `esp-generate`, um ein `no_std` Projekt herzustellen.
2.  Man nutzt `cargo generate esp-rs/esp-idf-template cargo` um ein `std`Projekt  herzustellen.
In beiden Fälle muss man in den Erstelloptionen `devcontainer` hinzufügen.

In beiden fällen wird im Hauptreiter des Projektes ein Ordner namens `.devcontainer`, welcher unsere `Dockerfile` enthält. 

Um das Dockerimage zu bauen nutze diesen Befehl:

```bash
docker image build -t <name> -f .devcontainer/Dockerfile .
```

Um es auszuführen nutze diesen Befehl:

```
docker run --mount type=bind,source="$(pwd)",target=/workspace,consistency=cached -it <name> /bin/bash
```

Ich empfehle für den letzteren Befehl einen Shortcut im Projekt anzulegen, anstatt den Befehl immer auszuschreiben, nutze ein ShellScript:

```bash
echo "docker run --mount type=bind,source="$(pwd)",target=/workspace,consistency=cached -it <name> /bin/bash" >> start_docker.sh
```

Je nach Dockerfile Konfiguration, hat man einen anderen Startpunkt. Um das Projekt ausführen muss man in den **workspace** Ordner navigieren, sofern man in Docker Zugang zu USB-Serial-Peripherals hat, kann man einfach sein Projekt mit `cargo run` ausführen und auf den Chip brennen. 
Andernfalls muss man mit `cargo build` die ELF-Datei bauen und dann manuell in der natürlichen Umgebung, also außerhalb von Docker, mit `espflash` auf den Chip bringen.

## Pin Mapping

| Pin | Purpose                        |
| --- | ------------------------------ |
| 21  | SDA ([MPU-6050](MPU-6050.md)) |
| 22  | SCL([MPU-6050](MPU-6050.md))  |
|     |                                |

## Nützliche Ressourcen

- https://docs.sunfounder.com/projects/esp32-starter-kit/en/latest/components/component_esp32_extension.html
- 
