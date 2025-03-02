
# WifiError

Modul wifi.

| Error            | Ursache                                                                                                       |
| ---------------- | ------------------------------------------------------------------------------------------------------------- |
| SSIDParsing      | SSID beinhaltet nicht UTF8 konforme Zeichensequenzen oder Limit von 32 Charakteren wurde überschritten.       |
| PasswordParsing  | Password beinhaltet nicht UTF8 konforme Zeichenseqeuenzen oder Limit von 32 Charakteren wurde überschritten.  |
| VectorConversion | Bytekette kann nicht in einen stack-basierten Vec umgesetzt werden. Wahrscheinlich ist die Bytekette zu lang. |
| Internal         | Fehler auf der Hardwareebene. Nutze die Debug-Funktionalität, um den Fehler zu definieren.                    |
