cargo build -p sol_chess --target wasm32-unknown-unknown --release && scp target/wasm32-unknown-unknown/release/sol_chess.wasm potato@neophyte.me:~/site/games/sol_chess/.
