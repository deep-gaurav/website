curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

rustup target add wasm32-unknown-unknown

curl --proto '=https' --tlsv1.2 -LsSf https://leptos-rs.artifacts.axodotdev.host/cargo-leptos/v0.2.20/cargo-leptos-installer.sh | sh

cargo leptos serve --release