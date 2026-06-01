# WinUltimate Toolkit 🛠️🚀

**WinUltimate Toolkit** to zaawansowane, graficzne narzędzie do optymalizacji, czyszczenia oraz automatyzacji konfiguracji systemu operacyjnego Windows. Aplikacja została napisana w języku **Rust** przy użyciu nowoczesnej, ultra-szybkiej biblioteki GUI **eframe / egui**. Została stworzona z myślą o zaawansowanych użytkownikach i administratorach, którzy chcą błyskawicznie przystosować "świeżo" zainstalowany system Windows do pracy lub gier, eliminując zbędne usługi, telemetrię i tzw. bloatware.

Aplikacja posiada pełne wsparcie **dwujęzyczne (Polski / English)**, automatycznie wykrywając lub pozwalając na przełączenie języka interfejsu.

---

## 📺 Główne Moduły i Funkcje

### 1. 📦 Instalator Oprogramowania (Software Installer)
Masowa, zautomatyzowana instalacja popularnego oprogramowania bez konieczności odwiedzania dziesiątek stron internetowych.
* **Integracja z Winget:** Narzędzie w pełni wykorzystuje oficjalny menedżer pakietów Windows (`winget`).
* **Inteligentne sprawdzanie:** Przed instalacją program weryfikuje, czy dana aplikacja nie znajduje się już w systemie, oszczędzając czas i transfer.
* **Szeroki wybór kategorii:** Przeglądarki, narzędzia programistyczne, odtwarzacze multimedialne, komunikatory i biblioteki uruchomieniowe.

### 2. 🛠️ Narzędzia Systemowe (System Tools)
Centrum dowodzenia naprawą i diagnostyką systemu za pomocą jednego kliknięcia.
* **SFC & DISM Scans:** Automatyczne uruchamianie procedur naprawy uszkodzonych plików systemowych oraz obrazu Windows (`sfc /scannow`, `DISM /Online /Cleanup-Image /RestoreHealth`).
* **Zarządzanie Dyskiem:** Szybkie wywołanie sprawdzania błędów `CHKDSK` oraz wymuszenie optymalizacji i czyszczenia pamięci masowej poleceniem `TRIM` dla dysków SSD.
* **Optymalizacja sieci:** Błyskawiczna zmiana konfiguracji DNS na ultra-szybkie i bezpieczne serwery **Cloudflare (1.1.1.1)** z jednoczesnym czyszczeniem pamięci podręcznej (`ipconfig /flushdns`).

### 3. 🗑️ Deinstalator Śmieci (Debloater)
Pozbądź się aplikacji, których Microsoft nie pozwala łatwo usunąć z poziomu standardowego Panelu Sterowania.
* **Całkowite usunięcie usług Xbox:** Wyłączenie i usunięcie procesów powiązanych z aplikacją Xbox, idealne dla systemów typowo biurowych lub maszyn wirtualnych.
* **Wyłączenie AI Copilot:** Usunięcie integracji ze sztuczną inteligencją Microsoftu z paska zadań i rejestru.
* **Usuwanie wbudowanych aplikacji:** Bezpieczne usuwanie Microsoft Edge, systemowych Widżetów, a także alternatywne czyszczenie klasycznych aplikacji takich jak Notatnik, Kalkulator (jeśli wolisz własne zamienniki).

### 4. ⚙️ Optymalizator Usług i Rejestru (Tweaks)
Zaawansowane modyfikacje systemu mające na celu odzyskanie zasobów RAM/CPU oraz poprawę prywatności.
* **Blokada Telemetrii:** Maksymalne ograniczenie wysyłania danych diagnostycznych i szpiegujących do serwerów Microsoftu.
* **Wstrzymanie Windows Update:** Opcja zamrożenia automatycznych aktualizacji systemu nawet do 180 dni (zapobiega nieoczekiwanym restartom w trakcie pracy).
* **Wydajność Wizualna:** Szybkie przełączenie efektów wizualnych Windows na tryb maksymalnej wydajności (wyłączenie zasobożernych animacji).
* **Modyfikacje UI:** Personalizacja menu Start, włączanie/wyłączanie ukrytych opcji w Eksploratorze Plików (np. pokazywanie rozszerzeń znanych plików).

---

## 🛠️ Wymagania i Technologie

Projekt bazuje na nowoczesnym i bezpiecznym ekosystemie języka Rust:
* **Język:** Rust (edycja 2021 lub nowsza)
* **GUI Framework:** `egui` wraz z backendem `eframe` (zapewniający natywne renderowanie sprzętowe, niskie zużycie RAM-u oraz błyskawiczny czas reakcji).
* **System operacyjny:** Windows 10 / Windows 11 (wymagane uprawnienia administratora do wykonania większości operacji).

---

## 🚀 Jak Uruchomić i Skompilować

Aby samodzielnie skompilować projekt ze źródeł, musisz mieć zainstalowane środowisko Rust (Cargo).

1. Sklonuj to repozytorium:
   ```bash
   git clone https://github.com/bartko4321/win-ultimate-toolkit.git
   cd win-ultimate-toolkit
   ```

2. Uruchom program w trybie developerskim:
   ```bash
   cargo run
   ```

3. Zbuduj zoptymalizowaną wersję produkcyjną (gotowy plik `.exe` znajdziesz w `/target/release/`):
   ```bash
   cargo build --release
   ```

> 💡 **Wskazówka:** Ponieważ aplikacja dokonuje głębokich modyfikacji w rejestrze i usługach systemowych, **zawsze uruchamiaj skompilowany plik `.exe` jako Administrator**.

---

## ⚠️ Zastrzeżenie (Disclaimer)

*Narzędzie dokonuje zaawansowanych zmian w konfiguracji systemu operacyjnego Windows, w tym w rejestrze systemowym oraz usługach systemowych. Autor nie ponosi odpowiedzialności za ewentualne uszkodzenia systemu, utratę danych lub niestabilność działania spowodowaną nieprawidłowym lub nieświadomym użyciem aplikacji. Przed uruchomieniem agresywnych skryptów czyszczących (Tweaks/Debloater) zaleca się utworzenie punktu przywracania systemu (System Restore Point).*

---
Stworzone z pasją w 🦀 **Rust**. Jeśli projekt Ci się podoba, zostaw gwiazdkę! ⭐
