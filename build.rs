fn main() {
    // Sprawdzamy, czy docelowym systemem kompilacji jest Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();

        // Jeśli kompilujemy skrośnie na Linuksie z użyciem MinGW (gnu),
        // musimy wskazać pełną nazwę kompilatora zasobów.
        if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "gnu" {
            res.set_windres_path("x86_64-w64-mingw32-windres");
        }

        // Ustawienie ikony (upewnij się, że plik icon.ico leży w tym samym folderze co build.rs!)
        res.set_icon("icon.ico");

        // Ustawienie metadanych dla Windowsa
        res.set("CompanyName", "Bartosz Szczeciński");
        res.set("FileDescription", "WinUltimateToolkit");
        res.set("ProductName", "WinUltimateToolkit");
        res.set("LegalCopyright", "© Bartosz Szczeciński");

        // Kompilacja zasobów z lepszym komunikatem błędu
        res.compile().expect("Błąd: Nie można skompilować zasobów! Upewnij się, że plik icon.ico istnieje i zainstalowano mingw-w64.");
    }
}
