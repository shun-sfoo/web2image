# web2image

## 功能

命令行工具 将网页生成图片并将网址生成二维码

`web2image <url> --output <path:/tmp/screenshot.jpg>`

## crates

- rust-headless-chrome (截图功能)
  对标 puppeteer
- clap
- rust-qrcode

## Bug fixed

报错：

```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("unknown variant `marker`,
expected one of `first-line`, `first-letter`, `before`, `after`, `backdrop`, `selection`, `first-line-inherited`,
`scrollbar`, `scrollbar-thumb`, `scrollbar-button`, `scrollbar-track`, `scrollbar-track-piece`, `scrollbar-corner`,
`resizer`, `input-list-button`", line: 0, column: 0)',
/Users/bruce/.cargo/registry/src/github.com-1ecc6299db9ec823/headless_chrome-0.9.0/src/protocol/mod.rs:90:70
```

在 github iusse 中 搜索 `unknown variant marker`

- https://github.com/atroche/rust-headless-chrome/issues/227
- https://github.com/atroche/rust-headless-chrome/pull/233/files

解决： 将 headless-chrome crate 换成 github 版本

## TODO

1. 速度比想象的慢， 使用 release mode 速度快很多
2. 计算时间的宏
