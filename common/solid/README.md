# solid

[SOLID-OS API][1]のRustバインディングです。使い方に関しては[rust-blinky-pac-cs][2]を参考にしてください。

## 使用法

このパッケージをSOLID-Rustプロジェクトに追加するには `Cargo.toml` に次の記述を追加してください。

```diff
  [dependencies]
+ solid = { git = "https://github.com/KyotoMicrocomputer/solid-rapi4-examples.git", features = ["std"] } 
```

その後、`.ptrsproj` ファイルをテキストエディタで開き、`ProjectGuid` 要素の次に次の内容の `CargoEnvironmentVariables` 要素を追加してください (`( PROJECT )` はC++プロジェクト名で置き換えてください)。

```xml
<CargoEnvironmentVariables>
    BUILD_INCLUDE_DIRS=$expand:{"projectName":"( PROJECT )", "type": "property", "query": "IncludePath"}
    BUILD_CFLAGS=$expand:{"projectName":"( PROJECT )", "type": "property", "query": "GCCSW"}
</CargoEnvironmentVariables>
```

[1]: http://solid.kmckk.com/doc/skit/current/os/core-service.html
[2]: ../../rust-blinky-pac-cs/rustapp/src/lib.rs
