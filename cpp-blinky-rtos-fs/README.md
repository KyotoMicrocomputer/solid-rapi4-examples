# cpp-blinky-rtos-fs

基板上のLEDを点滅させます。`/etc/led-power.txt` ファイルが存在する場合、そこから輝度値 (0–100) を読み込みます。

主要なコードは[`cpp-blinky-rtos-fs/main.cpp`](./cpp-blinky-rtos-fs/main.cpp)にあります。

```bash
# 固定輝度
sudo sh -c 'echo 30 > /etc/led-power.txt'

# 点滅
sudo sh -c 'rm -f /etc/led-power.txt'
```

## 準備

[Lチカの準備](../doc/blinky-prepare.md)を参照してください。

[1]: https://toppers.jp/docs/tech/tgki_spec-350.pdf#page=145
