# YVA - YAML Validator App

A client-side WebAssembly application for validating YAML using `serde-saphyr` and the Dioxus framework.

## Prerequisites

To build and run this project, you need to have Rust installed. Additionally, you need the WebAssembly target and the `trunk` build tool.

1.  **Install Rust**: [rustup.rs](https://rustup.rs/)
2.  **Add WebAssembly target**:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **Install Trunk**:
    ```bash
    cargo install trunk
    ```

## Development

To run the application locally with hot-reloading:

```bash
trunk serve
```

This will start a local server, usually at `http://localhost:8080`.

## Building for Production

To generate the static assets (HTML, JS, and Wasm) for deployment:

```bash
trunk build --release
```

The output will be in the `dist` directory, which can be hosted on any static file server.

## Features

- **Split-screen UI**: Input on the left, output on the right.
- **Client-side validation**: No data is sent to a server.
- **Powered by Dioxus**: High-performance reactive UI in Rust.
