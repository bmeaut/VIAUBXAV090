# A Rust telepítése

Már meglévő telepítés esetén ajánlott az eszközkészlet frissítése a `rustup update` paranccsal.

Új telepítés során szükségünk lesz:
- Windows+msvc toolchain használata esetén a Visual Studio 2026 „dektop development with C++” workloadjára
  - Ezt már a Rustup is felajánlja, ha mégsem sikerülne a telepítése, kézzel kell telepíteni
  - A laborgépeken ez adott
  - Alternatívaként használható a GNU toolchain, a következő lépésben leírt módon
- a Rust eszközkészletére (`cargo`, `rustc`, toolchainek, stb.)
  -  Ezeket a [`rustup`](https://rustup.rs) eszközzel telepítsük
  -  A telepítés során az alapértelmezett beállítások alapvetően jók
     -  Ha Windows telepítés során valaki nem szeretne MSVC-t telepíteni, a `x86_64-pc-windows-gnu` host triple segítségével telepítheti a gnu windows toolchaint
- Egy tetszőleges IDE-re, laborban ez a VS Code és a [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) kiegészítő párosa lesz
  - Saját gépen ajánlott a [RustRover](https://www.jetbrains.com/rust/), hallgatói és community licensszel is ingyenesen használható