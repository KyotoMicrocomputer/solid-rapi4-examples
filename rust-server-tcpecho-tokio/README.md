# rust-server-tcpecho-tokio

[TCP echoサーバー](https://ja.wikipedia.org/wiki/ECHO%E3%83%97%E3%83%AD%E3%83%88%E3%82%B3%E3%83%AB)の実装です。[Tokio][1]非同期ランタイムを経由してSOLID Sockets APIを使用してTCPポート7777でリッスンし、受信したデータを返します。

```
PowerShell:
PS> ..\Test-Tcp-Echo-Server.ps1 192.168.1.23 7777

POSIX shell + Socat:
$ echo test | socat - tcp:192.168.1.23:7777
```

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。

[1]: https://tokio.rs
