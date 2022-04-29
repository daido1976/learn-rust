# Socket

- [Rust で始めるネットワークプログラミング](https://www.amazon.co.jp/dp/B07SW2GXVF)
- https://github.com/teru01/socket-programming/tree/web-version

```sh
# with tcp protocol
$ cargo run tcp server 127.0.0.1:3333
# open another terminal
$ cargo run tcp client 127.0.0.1:3333
# input value

# with udp protocol
$ cargo run udp server 127.0.0.1:3333
# open another terminal
$ cargo run udp client 127.0.0.1:3333
# input value
```
