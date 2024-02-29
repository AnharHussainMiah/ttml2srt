echo "==> 🔨 lets compile the binary"
docker run \
    -v cargo-cache:/root/.cargo/registry \
    -v "$PWD:/volume" \
    --rm -it clux/muslrust:1.72.1 cargo build --release
sudo mv target/x86_64-unknown-linux-musl/release/ttml2srt .