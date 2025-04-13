# rust-server

A Rust client/server capable of sending and receiving data.

### How to use

```sh
cargo run --bin client <ip>:<port> <data>
cargo run --bin server <port>
```

Example:

```sh
cargo run --server 2525

cargo run --bin client 0.0.0.0:2525 meow

# Server output
meow
```

## License
The source code for this project is licensed under the MIT license. You may find the conditions of the license [here](LICENSE).