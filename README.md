# Starting the server-side
```
./target/debug/rust --server --port 25565 --address "127.0.0.1"
```
# Starting the client-side
```
./target/debug/rust --client --port 25565 --address "127.0.0.1"
```

# If compiling is needed
## Install rustc and cargo
```
sudo apt-get install rustc
sudo apt-get install cargo
```

## Install libssl
```
sudo apt-get install libssl-dev
```

## Install pkgconfig
```
sudo apt-get install pkgconfig
```

##Compile
```
cargo build
```

# Notes
1. The server must start before the client!
2.  The Diffie–Hellman key exchange algorithm is used for the key exchange.
3.  Generate a private key and public key based on the p and g.
4.  Compute key after getting client -> (g^clients private key)^our private key == shared secret
5.  Passes the key to the argon2id function which is used as a KDF.
6. When the iv has been used, the program generates a new IV using the cryptographically secure StdRng provided in the library https://docs.rs/rand/latest/rand/
