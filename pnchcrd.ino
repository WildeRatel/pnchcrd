#include <SPI.h>
#include <MFRC522.h>
#include <string.h>
#include <LiquidCrystal_I2C.h>
#include <Wire.h>

#define RST_PIN 9  // Configurable, see typical pin layout above
#define SS_PIN 10  // Configurable, see typical pin layout above

MFRC522 mfrc522(SS_PIN, RST_PIN);  // Create MFRC522 instance
String idBuilder;
String combine;

bool tapped = false;

int delim_1, delim_2, delim_3;
LiquidCrystal_I2C lcd(0x27, 20, 4);

void setup() {
  lcd.init();
  lcd.backlight();
  lcd.clear();
  lcd.blink_on();
  lcd.
  lcd.setCursor(0, 0);
  lcd.print("PNCHR");
  lcd.setDelay(0, 0);
  Serial.begin(9600);  // Initialize serial communications with the PC
  while (!Serial)
    ;                  // Do nothing if no serial port is opened (added for Arduinos based on ATMEGA32U4)
  SPI.begin();         // Init SPI bus
  mfrc522.PCD_Init();  // Init MFRC522
  delay(4);            // Optional delay. Some board do need more time after init to be ready, see Readme
}

void loop() {
  // Reset the loop if no new card present on the sensor/reader. This saves the entire process when idle.

  idBuilder = String("");
  if (!mfrc522.PICC_IsNewCardPresent()) {
    return;
  }

  // Select one of the cards
  if (!mfrc522.PICC_ReadCardSerial()) {
    return;
  }

  // Dump debug info about the card; PICC_HaltA() is automatically called
  for (auto i = 0; i < mfrc522.uid.size; i++) {
    idBuilder = idBuilder + String(mfrc522.uid.uidByte[i]);
  }

  Serial.println(idBuilder);
  lcd.clear();
  lcd.setCursor(0, 0);
  delay(10);
  combine = Serial.readString();
  delim_1 = combine.indexOf("|");
  delim_2 = combine.indexOf("|", delim_1 + 1);
  delim_3 = combine.indexOf("|", delim_2 + 1);

  lcd.print(combine.substring(0, delim_1));
  lcd.setCursor(0, 1);
  lcd.print(combine.substring(delim_1 + 1, delim_2));
  lcd.setCursor(0, 2);
  lcd.print(combine.substring(delim_2 + 1, delim_3));
  lcd.setCursor(0, 3);
  lcd.print(combine.substring(delim_3 + 1));
  mfrc522.PICC_HaltA();
  delay(5000);
  lcd.clear();
  lcd.setCursor(0, 0);
  lcd.print("PNCHR");
}