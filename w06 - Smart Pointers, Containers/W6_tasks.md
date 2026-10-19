
# # W6T1: Táblák (első verzió)

A cél egy relációs táblákat kezelni képes egyszerű in-memory tároló struktúra megvalósítása

A rendszerrel szemben támasztott követelmények:
- A cellák típusa lehet 
  - Integer(i64)
  - Text(String)
  - Number(f64)
  - Bool(bool)
  - UUID(uuid)
- A táblák sémáját úgy tároljuk, hogy több tábla is hivatkozhasson rájuk
- sorok beszúrása, lekérdezése, (+3 zh pontért táblák illesztése) cél
- egy adott azonosító alapján lehessen sort lekérdezni, az értéket generikusan kinyerni
- Egy oszlop mediánja, átlaga legyen kiszámítható

## Tippek
- Javaslom a cellák értékéhez egy enum bevezetését (CellType)
- A sémák tárolására vezessünk be egy struktúrát
  - Ez tároljon az oszlopokat leíró struktúrákat
- A táblák kezelésére is vezessünk be struktúrát
- Az értékek kinyerésére vezessünk be generikus függvényt
- Az oszlopok átlagolásához, medián kiszámításához vezessünk be külön függvényeket

Checkpointok:
- A szükséges adatstruktúrák létrehozása
- Sor beszúrása
- Medián kiszámítása
