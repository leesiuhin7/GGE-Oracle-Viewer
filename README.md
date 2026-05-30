# GGE Oracle Viewer

A web application for viewing data collected by [GGE-Oracle](https://github.com/leesiuhin7/GGE-Oracle).

## Getting Started

### Prerequisites

- Node.js 20.19+
- Rust (via rustup)
- wasm-pack

### Installing

1. Clone the repo
```sh
git clone https://github.com/leesiuhin7/GGE-Oracle-Viewer.git
```
2. Install project dependencies
```sh
npm install
```
3. Compile Rust to WebAssembly
```sh
npm run build:wasm
```
4. Run development server
```sh
npm run dev
```

Alternatively, for production, replace step 3 and 4 with
```sh
npm run build:wasm:release
npm run build
```
Then preview using
```sh
npm run preview
```

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.