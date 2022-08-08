# rust-blinky-pac-tokio

基板上のLEDを点滅させます。[Tokio][2]ランタイム上で動作し、遅延を発生させるためにTokioライブラリの非同期関数 [`tokio::time::sleep`][1] を使用します。GPIOレジスタの操作に[peripheral access crate](../common/bcm2711_pac)を使用します。

UDPポート52000にパケットを送ることで点滅周期を変更できます。例:

```
PowerShell:
PS1> ./Set-Blinky-Period.ps1 192.168.1.23 50000

POSIX shell + Socat:
$ echo 50000 | socat - udp:192.168.1.23:52000
```

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://docs.rs/tokio/1.20.1/tokio/time/fn.sleep.html
[2]: https://tokio.rs
