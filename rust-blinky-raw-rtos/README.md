# rust-blinky-raw-rtos

基板上のLEDを点滅させます。遅延を発生させるためにTOPPERSカーネル関数 [`dly_tsk`][1] を [`extern` ブロック][2]で宣言して使用します。他の rust-blinky-\* サンプルとは異なり、このクレート単体で完結していますが、その代わりアプリケーションコードで多くの `unsafe` 操作が必要になります。

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。[c-blinky-rtos](../c-blinky-rtos/c-blinky-rtos/main.cpp) (同等のC++コード) や [rust-blinky-pac-std](../rust-blinky-pac-std/rustapp/src/lib.rs) (安全なラッパーを使用したRustコード) と比べてみてください。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://toppers.jp/docs/tech/tgki_spec-350.pdf#page=145
[2]: https://doc.rust-lang.org/1.58.1/reference/items/external-blocks.html#external-blocks
