# kerf-adjust

This is going to be a web application where you can upload a DXF into your web browser and offset contours
in order to adjust for the kerf of your laser.


The UI elements are made using React and TypeScript. The DXF manipulation logic is written using Rust and compiled
to WebAssembly so that it can run in the browser. 


## Building

Both Rust and NodeJS are needed to build this project

### Setting up Rust 

1. Install Rust with [`rustup`](https://rustup.rs/)
2. Install [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

The `wasm-pack` executable should be in your `PATH` for the webpack `WasmPackPlugin` to find it

### Setting up JS

Besides the Rust/Wasm bits, this is your run of the mill React app

1. Have `yarn` or `npm` installed
2. `yarn install` or `npm install`

## Running
Use `yarn start` or `npm run start` based on your package manager

## Building

`yarn build` or `npm run build`

