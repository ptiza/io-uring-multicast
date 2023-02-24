# io-uring-multicast
io_uring proof of concept for UDP multicast with glommio and tokio-uring

---

Features a client (receiver) and a server (sender).

Transmits a number of packets which contain a string representation of a number, which is verified at the receiver.

```sh
# build
cargo build --release
# binaries in:
cd target/release
```


### Example:
- In one terminal window, to start client, execute:
    - `./tokio-uring-mc -m 239.0.123.4`
- In another terminal window, to start server, execute:
    - `.tokio-uring-mc -m 239.0.123.4 -s`
    

### CLI arguments:
```
Options:
  -s, --server                 
  -b, --bind-addr <BIND_ADDR>  [default: 0.0.0.0]
  -m, --mc-addr <MC_ADDR>      
  -d, --data-size <DATA_SIZE>  [default: 128]
  -c, --count <COUNT>          [default: 10]
  -h, --help                   Print help
  -V, --version                Print version
 ```
