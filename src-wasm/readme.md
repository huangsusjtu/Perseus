#### wasm 模块
用rust构建wasm模块，给src目录下的前端使用

每一个wasm模块 在此目录用 如下命令创建
```bash
cargo new --lib libexample   
```

### 编译
```bash
wasm-pack build  --release --target web 
```

### 压缩
```bash
wasm-opt -Oz -o output.wasm input.wasm
```