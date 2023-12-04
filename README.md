Build Instructions:
1. Ensure you have the prerequisites listed here: https://rustwasm.github.io/docs/book/game-of-life/setup.html
2. Install
  a. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  b. curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
  c. curl https://sh.rustup.rs -sSf | sh
  d. cargo install cargo-generate
  e. sudo apt install npm
  e. npm install npm@6.14.15
4. Build the Rust into wasm:
  a. Ensure you are in the root directory of the project
  b. run `wasm-pack build`
5. Install npm dependencies
  a. cd into the www directory
  b. run `npm install`
6. Run the web server
  a. Still in the www directory, run `npm run start`


if you see the linker cc not found error run 'sudo apt install build-essential' and restart at cargo install cargo-generate

