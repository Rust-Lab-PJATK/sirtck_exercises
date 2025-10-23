# Opis poziomów

## Poziom A — Absolutni początkujący
Na tym poziomie znajdują się krótkie zadania w jednym pliku, oparte wyłącznie na bibliotece standardowej i prostych konstrukcjach sterujących. Ćwiczenia obejmują operacje na łańcuchach i wektorach, parsowanie danych wejściowych z CLI oraz pierwsze spotkanie z typami `Option` i `Result`.
Realizując te zadania nauczysz się korzystać z narzędzia `cargo`, pisać poprawną obsługę błędów bez `unwrap()`, budować proste pętle i dopasowania `match`, a także czytać i zapisywać dane z konsoli.

## Poziom B — Programiści z doświadczeniem w innych językach
Ten poziom wprowadza idiomatyczne wzorce Rusta: moduły, struktury, enumy oraz lekkie testowanie. Projekty są najczęściej prostymi aplikacjami CLI, czasem z zapisem do plików lub wykorzystaniem serde, ale nadal bez `unsafe`.
Wykonując zadania nauczysz się systematycznie zwracać `Result` i używać operatora `?`, organizować kod w modułach, pisać testy jednostkowe i doctesty oraz świadomie unikać zbędnych klonów dzięki referencjom i iteratorom.

## Poziom C — Zaawansowani w innych językach
Zadania wymagają projektowania generycznych API, określania ograniczeń traitów i zarządzania jawniejszymi lifetimami. Budujesz niewielkie biblioteki z własnymi iteratorami, typami błędów i modułową architekturą.
Przechodząc przez ten poziom nauczysz się tworzyć wielokrotnego użytku komponenty, definiować konwersje `From`, zarządzać borrowingiem w bardziej złożonych scenariuszach oraz pisać testy właściwości dla struktur danych.

## Poziom RB — Rustowicze na poziomie podstawowym
Projekty na tym poziomie obejmują wielowątkowe narzędzia, współdzielone zasoby `Arc/Mutex/RwLock`, kanały oraz pierwsze kroki w asynchroniczności. Kod nadal korzysta głównie z bezpiecznych abstrakcji i rozsądnie dobranych crate’ów.
Realizując zadania nauczysz się projektować API zgodne z zasadami pożyczania, unikać zakleszczeń, świadomie pracować z `Send` i `Sync`, porównywać implementacje sekwencyjne z iteratorowymi oraz przygotowywać podstawowe benchmarki.

## Poziom RC — Rustowicze zaawansowani
Tutaj pojawia się produkcyjna asynchroniczność (np. tokio), zaawansowane lifetimy, `Pin`, makra oraz elementy niskopoziomowej optymalizacji. Zadania często wymagają dokumentowania inwariantów i uzasadniania wyborów projektowych.
W trakcie pracy nauczysz się pisać bezpieczne otoczki wokół `unsafe`, projektować elastyczne API oparte na traitach, tworzyć makra `macro_rules!` i proste procmakra, a także profilować kod i mierzyć wydajność.

## Poziom X — Zadania olimpijskie
Na najwyższym poziomie znajdują się złożone problemy algorytmiczne o surowych ograniczeniach czasowych i pamięciowych. Wymagana jest perfekcyjna kontrola nad alokacjami, szybkie I/O oraz znajomość zaawansowanych struktur danych.
Podejmując te wyzwania nauczysz się implementować segment tree, struktury do zapytań offline, suffix automaty oraz algorytmy przepływu, optymalizować złożoność i dokumentować koszty obliczeniowe każdej funkcji.
