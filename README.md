## Getting Started

### Dependance:

```bash
pnpm i
```

### Run the development server:

```bash
pnpm dev
```

## WASM
在package.json文件里添加自己的wasm模块编译:  

```bash
  "scripts": {
    ...
    "build:wasm": "cd src-wasm/libexample && wasm-pack build --target web --out-dir pkg"
  },
```

编译
```bash
pnpm build:wasm
```

安装
```bash
pnpm install ./src-wasm/libexample/pkg  --ignore-scripts
```
## Other

  "lib3dgs": "link:src-wasm/lib3dgs/pkg",
"libexample": "link:src-wasm/libexample/pkg",


### proto 使用

安装依赖

npm install -g protobufjs
npm install -g pbjs
npm install -g protoc-gen-ts

<!-- pbjs -t json src/proto/map.proto > src/app/map/serialize/map_bundle.json -->

protoc -I=./src/proto/ --ts_out=./src/app/map/serialize/  ./src/proto/map.proto 


