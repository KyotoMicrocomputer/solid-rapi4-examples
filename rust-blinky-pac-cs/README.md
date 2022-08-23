# rust-blinky-pac-cs

基板上のLEDを点滅させます。遅延を発生させるためにSOLID Core Service[タイマAPI][1]を[ラッパーライブラリ](../common/solid)を経由して使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: http://solid.kmckk.com/doc/skit/current/os/cs/timer.html
