# ChaCha20 and integration with Poly1305

Please check the latest version on [GitHub](github.com/leo-leesco/Crypto-TD3).

## Build

`cargo build` produces `chacha20` <!--, `aead_wrap` and `aead_unwrap`--> in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`chacha20` expects :
- `path_to_keyfile`, the keyfile being a 64-byte binary file
- a 12-byte hexadecimal nonce (24 characters)
- `path_to_input`, the input being binary data
- `path_to_output`, creating a new file if it does not exist, and overwriting an existing one
