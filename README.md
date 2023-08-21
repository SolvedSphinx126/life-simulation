Build Instructions:
1. Ensure you have the prerequisites listed here: https://rustwasm.github.io/docs/book/game-of-life/setup.html
2. Build the Rust into wasm:
  a. Ensure you are in the root directory of the project
  b. run `wasm-pack build`
3. Install npm dependencies
  a. cd into the www directory
  b. run `npm install`
4. Run the web server
  a. Still in the www directory, run `npm run start`