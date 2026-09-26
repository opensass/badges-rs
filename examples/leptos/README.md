# 📚 Badges RS Leptos Example

## 🛠️ Pre-requisites:

### 🐧 **Linux Users**

1. **Install [`rustup`](https://www.rust-lang.org/tools/install)**:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **Install [`trunk`](https://trunkrs.dev/)**:

   ```sh
   cargo install --locked trunk
   ```

1. **Add the Wasm target**:

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

### 🪟 **Windows Users**

1. **Download and install `rustup`**: Follow the installation instructions [here](https://www.rust-lang.org/tools/install).

1. **Install Linux packages in WSL**:

   ```sh
   sudo apt update
   sudo apt install build-essential pkg-config libudev-dev
   ```

1. **Install `trunk`** and **add the Wasm target**:

   ```sh
   cargo install --locked trunk
   rustup target add wasm32-unknown-unknown
   ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

   ```sh
   git clone https://github.com/opensass/badges-rs
   ```

1. Navigate to the application directory.

   ```sh
   cd badges-rs/examples/leptos
   ```

1. Run the client:

   ```sh
   trunk serve --port 3000
   ```

Navigate to http://localhost:3000 to explore all available components.
