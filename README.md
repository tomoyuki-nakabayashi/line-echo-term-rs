# line-echo-term-rs

シリアルポートからのエコーバックを行単位で表示します。

## 実行方法

シリアルデバイス (シリアルUSB変換モジュールなど) を PC に接続し、デバイスパスを確認して下さい。
プログラムの引数に、デバイスパスを与えます。

`echo: ` の後に、シリアルデバイスからエコーバックされたデータが表示されます。

```shell
$ cargo run /dev/ttyUSB0
Connected to /dev/ttyUSB0 at 115200 baud
Ctrl+D (*nix) or Ctrl+Z (Win) to stop.
hello
echo: hello
wio terminal returs echo in lines.
echo: wio terminal returs echo in lines.
Stopping.
```

エコーバック以外でシリアルデバイスから受信したデータは、そのまま表示します。
例えば、シリアルデバイスが　`hello world` を受信すると、それがそのまま表示されます。

```
hello world
```
