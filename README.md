![yk](yk.png)

# mjtools

## How to build

```
git clone https://github.com/aikiriao/mjtools-rs
cd mjtools-rs
cargo build
```

## Example

### `shanten`（向聴数計算）

```
> ./target/debug/mjtools shanten 5M6m7m2p3p4p3s3s5s6s7s7s
Shanten: 1
> ./target/debug/mjtools shanten 5M6m7m2p3p4p3s3s5s6s7s7s8s
Shanten: 0 (tenpai)
Effective tiles: 6s, 9s,
```

### `score`（点数計算）

```
> ./target/debug/mjtools score --player pee --round ton 5M6m7m3p4p3s3s5s6s7s 2p --pung 8s8s8s
2 han 30 fu, 2000 point
yaku: ドラ,  断么九,
feed: 2000
> ./target/debug/mjtools score --player pee --round ton 3m4m5M6m6m6m8m9m発発 7m --pung 白白白
4 han 40 fu, 8000 point
yaku: 白, 混一色, ドラ,
feed: 8000
```

## 参考文献

- [麻雀C言語プログラム集(web魚拓)](https://web.archive.org/web/20190402234201/http://cmj3.web.fc2.com/index.htm)

## LICENSE

Copyright (c) 2022 aikiriao Licensed under the Apache-2.0 license.
