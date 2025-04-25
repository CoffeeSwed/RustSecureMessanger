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
2. BigNumContext is used to calculate big numbers efficently, uses the cheats included in Crypto 101 i guess.
3.  openssl::dh::Dh::get_2048_256() -- Instance used to generate keys, generates a DH instance that has the parameters modulous p given and base g. 
4.  Generate a private key and public key based on the p and g.
5.  Compute key after getting client -> (g^clients private key)^our private key == shared secret
6.  Passes the key to the argon2id function which is used as a KDF -> Parameters pulled from https://en.wikipedia.org/wiki/Argon2 Has to be 32 at memory size since this is the amount of bytes the KDF function creates and uses
7. When the iv has been used, generate a new IV using the cryptographically secure StdRng provided in the library https://docs.rs/rand/latest/rand/