# rust-blinky-pac-std

基板上のLEDを点滅させます。遅延を発生させるためにSOLID Core Service[タイマAPI][1]を使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: http://solid.kmckk.com/doc/skit/current/os/cs/timer.html
