
# bcm2711_pac

<a href="https://kyotomicrocomputer.github.io/solid-rapi4-examples/rustdoc/bcm2711_pac/" label="API docs"><img src="https://img.shields.io/badge/API%20docs-bcm2711__pac-green?style=for-the-badge&logo=Rust"></a>

BCM2711 SoC向けの[peripheral access crate][1]です。使い方に関しては[`tock-registers`の`README.md`][2]や[rust-blinky-pac-std](../../rust-blinky-pac-std/rustapp/src/lib.rs)を参考にしてください。

## 使用法

このパッケージをSOLID-Rustプロジェクトに追加するには `Cargo.toml` に次の記述を追加してください。

```diff
  [dependencies]
+ bcm2711_pac = { git = "https://github.com/KyotoMicrocomputer/solid-rapi4-examples.git" } 
```

[1]: https://doc.rust-lang.org/stable/embedded-book/start/registers.html#using-a-peripheral-access-crate-pac
[2]: https://crates.io/crates/tock-registers/0.7.0#user-content-example-using-registers-and-bitfields
