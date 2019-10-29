# Building and Running
This project uses Webpack and Rust to execute wasm in the browser. After setting up the Node environment with `npm i`, the `npx webpack` command will build the Rust source and bundle the JS into the `dist` directory.

# Running Locally
Due to CORS restrictions, the file:// protocol is not suitable for testing wasm, and an HTTP server needs to be used instead. `npx live-server` will start a preview.