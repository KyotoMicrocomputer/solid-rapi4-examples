# c-blinky-rtos

基板上のLEDを点滅させます。遅延を発生させるためにTOPPERSカーネル関数 [`dly_tsk`][1] を使用します。

## 準備

Linuxが起動した直後はSDカードアクセスの度にLED(緑)が点滅するようになっています。
<div><p><details>
<summary>初期状態</summary>
<div><p>
<code>
pi@raspberrypi:~$ cat /sys/class/leds/led0/trigger
none rc-feedback kbd-scrolllock kbd-numlock kbd-capslock kbd-kanalock kbd-shiftlock kbd-altgrlock kbd-ctrllock kbd-altlo
ck kbd-shiftllock kbd-shiftrlock kbd-ctrlllock kbd-ctrlrlock timer oneshot heartbeat backlight gpio cpu cpu0 cpu1 cpu2 c
pu3 default-on input panic actpwr mmc1 [mmc0] rfkill-any rfkill-none rfkill0 rfkill1
</code>
</p></div><br>
</details></p></div>

SOLID-OS側で操作したいので、Linuxでの点滅イベントを無し（`none`）に設定します。

Linuxのターミナルで以下のように入力します。
```
$ sudo sh -c 'echo none > /sys/class/leds/led0/trigger'
```

<div><p><details>
<summary>設定変更後の状態</summary>
<div><p>
<code>
pi@raspberrypi:~$ cat /sys/class/leds/led0/trigger
[none] rc-feedback kbd-scrolllock kbd-numlock kbd-capslock kbd-kanalock kbd-shiftlock kbd-altgrlock kbd-ctrllock kbd-alt
lock kbd-shiftllock kbd-shiftrlock kbd-ctrlllock kbd-ctrlrlock timer oneshot heartbeat backlight gpio cpu cpu0 cpu1 cpu2
 cpu3 default-on input panic actpwr mmc1 mmc0 rfkill-any rfkill-none rfkill0 rfkill1
</code>
</p></div>
</details></p></div>

これで、LED(緑)をSOLID-OS側で操作する準備が完了します。

