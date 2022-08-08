# rust-blinky-pac-rtos

基板上のLEDを点滅させます。遅延を発生させるためにTOPPERSカーネル関数 [`dly_tsk`][1] を [`itron`][2] パッケージの [`itron::task::delay`][3] 関数を経由して使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://toppers.jp/docs/tech/tgki_spec-350.pdf#page=145
[2]: https://crates.io/crates/itron
[3]: https://docs.rs/itron/0.1.9/itron/task/fn.delay.html
