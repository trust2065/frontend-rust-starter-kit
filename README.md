# Rust WebAssembly (Wasm) 入門指南

本專案展示了如何將 Rust 程式碼編譯為 WebAssembly 並在瀏覽器中執行。

## 🚀 核心開發流程

### 1. 環境配置：避開 Homebrew 陷阱
> **重要**：請勿使用 Homebrew 安裝 Rust。Homebrew 版本缺乏 `rustup` 工具，會導致無法管理 Wasm 編譯目標（Target）。

**正確安裝方式**：使用官方指令
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**增加 Wasm 支援**：
```bash
rustup target add wasm32-unknown-unknown
```

### 2. 安裝打包工具 (`wasm-pack`)
`wasm-pack` 是 Rust 轉 Wasm 的「瑞士軍刀」，負責編譯、封裝並產生 npm 套件格式。
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 3. 設定編譯類型 (指定連接 JS)
在 `Cargo.toml` 中，必須明確告訴 Rust 這是一個要給外部（JS）呼叫的動態連結庫。
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### 4. 安裝 JS/Rust 連接器 (`wasm-bindgen`)
`wasm-bindgen` 是兩門語言間的「翻譯官」，負責處理記憶體與型別對接。

**專案中安裝**：
```bash
cargo add wasm-bindgen
```

**Rust 程式碼標記**：在要公開給 JS 呼叫的函式上方加上 `#[wasm_bindgen]`。
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_easy(a: i32, b: i32) -> i32 {
    a + b
}
```

### 5. 產生封裝產物與引用
**執行打包指令**：
```bash
wasm-pack build --target web
```
*   **產物**：會生成一個 `pkg/` 資料夾，內容包含 `.wasm` 二進位檔、`.js` 膠水程式碼，以及 `.d.ts` 定義檔。

**JS 引用方式**：
```javascript
import init, { add_easy } from './pkg/hello_world.js';

async function run() {
    await init(); // 初始化傳送門
    console.log(add_easy(10, 20)); // 直接呼叫 Rust 邏輯
}
run();
```
