# SOLID for Raspberry Pi 4 Examples

## サンプル一覧

### Blinky (Lチカ)

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

- **[cpp-server-tcpecho](./cpp-server-tcpecho)**: TCP echoサーバー
- **[rust-server-gotham](./rust-server-gotham)**: [Gotham Webフレームワーク](https://gotham.rs/)ベースのHTTPサーバー
- **[rust-server-rocket](./rust-server-rocket)**: [Rocket Webフレームワーク](https://rocket.rs/)ベースのHTTPサーバー
- **[rust-server-tcpecho-std](./rust-server-tcpecho-std)**: Rust標準ライブラリを使用したTCP echoサーバー
- **[rust-server-tcpecho-tokio](./rust-server-tcpecho-tokio)**: [Tokio非同期ランタイム](https://tokio.rs)を使用したTCP echoサーバー

