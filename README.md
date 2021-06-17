# web2image

## 功能

命令行工具 将网页生成图片并将网址生成二维码

`web2image <url> --output <path:/tmp/screenshot.jpg>`

## 库

- rust-headless-chrome (截图功能)
  对标 puppeteer
- clap
- rust-qrcode

## TODO

1. 速度比想象的慢， 使用 release mode 速度快很多
2. 计算时间的宏
