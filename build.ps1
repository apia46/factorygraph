cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir site/pkg ./target/wasm32-unknown-unknown/debug/factorygraph.wasm
