# rust-blinky-pac-std

基板上のLEDを点滅させます。遅延を発生させるためにRust標準ライブラリ関数 [`std::thread::sleep`][1] を使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://doc.rust-lang.org/1.62.0/std/thread/fn.sleep.html
