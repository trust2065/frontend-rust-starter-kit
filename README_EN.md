# Rust WebAssembly (Wasm) Getting Started Guide

This project demonstrates how to compile Rust code into WebAssembly and run it in the browser.

## 🚀 Core Development Flow

### 1. Environment Setup: Avoid the Homebrew Trap
> **Important**: Do NOT use Homebrew to install Rust. The Homebrew version lacks the `rustup` tool, which is necessary for managing Wasm targets.

**Correct Installation**: Use the official script
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Add Wasm Support**:
```bash
rustup target add wasm32-unknown-unknown
```

### 2. Install Packaging Tool (`wasm-pack`)
`wasm-pack` is the "Swiss Army knife" for Rust-to-Wasm conversion, handling compilation, packaging, and generating the npm package format.
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 3. Configure Build Type (Linking to JS)
In `Cargo.toml`, you must explicitly tell Rust that this is a dynamic library meant for external (JS) calls.
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### 4. Install JS/Rust Connector (`wasm-bindgen`)
`wasm-bindgen` acts as the "translator" between the two languages, handling memory and type conversions.

**Install in Project**:
```bash
cargo add wasm-bindgen
```

**Tag Rust Code**: Add `#[wasm_bindgen]` above functions you want to expose to JS.
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_easy(a: i32, b: i32) -> i32 {
    a + b
}
```

### 5. Build and Import
**Execute Build Command**:
```bash
wasm-pack build --target web
```
*   **Artifacts**: A `pkg/` folder will be generated containing the `.wasm` binary, `.js` glue code, and `.d.ts` definitions.

**JS Usage**:
```javascript
import init, { add_easy } from './pkg/hello_world.js';

async function run() {
    await init(); // Initialize the Wasm module
    console.log(add_easy(10, 20)); // Call Rust logic directly
}
run();
```
