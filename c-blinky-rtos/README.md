# c-blinky-rtos

基板上のLEDを点滅させます。遅延を発生させるためにTOPPERSカーネル関数 [`dly_tsk`][1] を使用します。

主要なコードは[`c-blinky-rtos/main.cpp`](./c-blinky-rtos/main.cpp)にあります。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://toppers.jp/docs/tech/tgki_spec-350.pdf#page=145
