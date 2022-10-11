# SOLID for Raspberry Pi 4サンプル集

本リポジトリには[SOLID for Raspberry Pi 4][1]のサンプルプログラムが含まれています。

## サンプルプログラム一覧

### Blinky (Lチカ)

ハードウェア入出力・タイミング制御・割込み処理など組込みソフトウェア開発の重要なトピックを押さえていることから、Lチカはサンプルプログラムの定番です。

- **[c-blinky-rtos](./c-blinky-rtos)**: RTOS APIを使用したLチカ
- **[cpp-blinky-ap804](./cpp-blinky-ap804)**: AP804ハードウェアタイマーを使用したLチカ
- **[cpp-blinky-cs](./cpp-blinky-cs)**: SOLID-OSタイマAPIを使用したLチカ
- **[cpp-blinky-rtos](./cpp-blinky-rtos)**: RTOS APIを使用したLチカ
- **[cpp-blinky-rtos-fs](./cpp-blinky-rtos-fs)**: RTOS APIを使用したLチカ (ファイルシステム使用)
- **[rust-blinky-pac-ap804](./rust-blinky-pac-ap804)**: AP804ハードウェアタイマーと[peripheral access crate](./common/bcm2711_pac)を使用したLチカ
- **[rust-blinky-pac-cs](./rust-blinky-pac-cs)**: SOLID-OSタイマAPIと[peripheral access crate](./common/bcm2711_pac)を使用したLチカ
- **[rust-blinky-pac-rtos](./rust-blinky-pac-rtos)**: RTOS APIと[peripheral access crate](./common/bcm2711_pac)を使用したLチカ
- **[rust-blinky-pac-std](./rust-blinky-pac-std)**: Rust標準ライブラリと[peripheral access crate](./common/bcm2711_pac)を使用したLチカ
- **[rust-blinky-pac-tokio](./rust-blinky-pac-tokio)**: [Tokio非同期ランタイム](https://tokio.rs)と[peripheral access crate](./common/bcm2711_pac)を使用したLチカ
- **[rust-blinky-raw-rtos](./rust-blinky-raw-rtos)**: FFI宣言とRTOS APIを使用したLチカ

### ネットワークサーバー

SOLIDネットワークAPIを通じて、Linuxプロセスや外部システムと通信できます。

- **[cpp-server-tcpecho](./cpp-server-tcpecho)**: TCP echoサーバー
- **[rust-server-gotham](./rust-server-gotham)**: [Gotham Webフレームワーク](https://gotham.rs/)ベースのHTTPサーバー
- **[rust-server-rocket](./rust-server-rocket)**: [Rocket Webフレームワーク](https://rocket.rs/)ベースのHTTPサーバー
- **[rust-server-tcpecho-std](./rust-server-tcpecho-std)**: Rust標準ライブラリを使用したTCP echoサーバー
- **[rust-server-tcpecho-tokio](./rust-server-tcpecho-tokio)**: [Tokio非同期ランタイム](https://tokio.rs)を使用したTCP echoサーバー

## ライセンス

本リポジトリに含まれるサンプルプログラムのコード (`common` ディレクトリ以下のライブラリ含む) は[BSD Zero Clause License](LICENSE)に基づいて使用できます。

[1]: https://solid.kmckk.com/SOLID/solid4rpi4

