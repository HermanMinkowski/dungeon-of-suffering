# Dungeon of suffering

A text adventure for the adventurous.



## Building and Running web

`cargo build --target wasm32-unknown-unknown --release`                                                   
`wasm-bindgen .\target\wasm32-unknown-unknown\release\dungeon_of_suffering.wasm --out-dir web --target web`
`python -m http.server --directory web`


