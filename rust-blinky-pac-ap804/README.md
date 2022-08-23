# rust-blinky-pac-ap804

基板上のLEDを点滅させます。遅延を発生させるためにBCM2711のペリフェラルの一つである[Arm AP804タイマー][1]を使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A162%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
