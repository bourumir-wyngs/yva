# YVA - YAML Validator App

Yva (Yaml VAlidator) is a client-side WebAssembly application for validating YAML using [`serde-saphyr`](https://github.com/bourumir-wyngs/serde-saphyr) and the [Dioxus](https://dioxuslabs.com/) framework. It validates user YAML inside browser, without sending it to the server.

Yva is available on [https://verdanta.tech/yva/](https://verdanta.tech/yva/). It is used primarily to 
- To estimate budget limits for your YAML.
- To check if your YAML will be parsed by serde-saphyr.
- We also use it to verify and demonstrate serde-saphyr compatibility with WebAssembly frameworks. Notably, budget reporting API was re-designed while checking how easy it is to call from Dioxus.

<p>
    <a href="https://verdanta.tech"><img src="https://verdanta.tech/verdanta_logo_small.jpg"
    alt="Relative median time vs baseline"
    width="100px"/>
    </a>
</p>

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

### Deploying to a Non-Root Path

If you need to deploy the application to a subdirectory (e.g., `https://example.com/yva/` instead of `https://example.com/`), you have two options:

**Option 1: Configure in Trunk.toml**

Edit `Trunk.toml` and set the `public_url`:

```toml
[build]
public_url = "/yva/"
```

Then build normally:

```bash
trunk build --release
```

**Option 2: Use Command-Line Flag**

Build with the `--public-url` flag:

```bash
trunk build --release --public-url /yva/
```

**Important**: The public URL must start and end with a forward slash (e.g., `/yva/`, not `yva` or `/yva`).

## Features

- **Split-screen UI**: Input on the left, output on the right.
- **Client-side validation**: No data is sent to a server.
- **Powered by Dioxus**: High-performance reactive UI in Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
