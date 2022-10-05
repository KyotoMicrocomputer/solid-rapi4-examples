# rust-server-gotham

[Gotham Webフレームワーク](https://gotham.rs/)を使用してSOLID上でWebアプリケーションをホストします。

アプリケーションを起動後、 <http://raspberrypi.local:8080> (`raspberrypi` をRaspberry Piのホスト名に置き換えてください) からアプリケーションにアクセスできます。また、Linuxファイルシステムの `/var/www/solid-example` 以下にファイルを配置すると、<http://raspberrypi.local:8080/v0/fs> 以下からアクセスできるようになります。

主要なコードは[`rustapp/src/lib.rs`](./rustapp/src/lib.rs)にあります。
