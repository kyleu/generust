# Generust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](License)

## A Rust project template for dynamic web applications

`Generust` is a [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) template that provides a Rust web server and WASM client with some [interesting features](doc/features.md).

It uses [actix-web](https://actix.rs), [maud](https://maud.lambda.xyz), [UIKit](https://getuikit.com), and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to serve your app as a server or webview, via HTTP and WebSockets.

## Example Projects

[generust-example-project](https://github.com/kyleu/generust-example-project) - Basically just the result of running this template

## Use `cargo generate` to clone this template and make it your own

```script
cargo install cargo-generate
cargo generate --git https://github.com/kyleu/generust.git --name my-project
cd my-project
./bootstrap
```

This will package the WASM client, compile the UIKit SCSS, and build the main application.
You can execute `cargo run` to start the server and open a system webview pointing to it, or run `{{project-name}} -h` to see the CLI options.
[Scripts](doc/scripts.md) are provided in `./bin` that will help you build and publish the app.

Good luck!

## Your Project

After you've created your project, the variables in this file will be replaced with your project's information, just remove this section and the lines above it.

# {{project-name}}

[![Build Status](https://travis-ci.org/{{project-name}}/{{project-name}}.svg?branch=master)](https://travis-ci.org/{{project-name}}/{{project-name}})
[![Crate](https://meritbadge.herokuapp.com/{{project-name}})](https://crates.io/crates/{{project-name}})
[![Docs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![Dependencies](https://deps.rs/repo/github/{{project-name}}/{{project-name}}/status.svg)](https://deps.rs/repo/github/{{project-name}}/{{project-name}})

{{project-name}} is a work in progress...

Running as a client application or shared server, `{{project-name}}` has a focus on performance, correctness, and developer comfort.

See [installing.md](doc/installing.md) for installation guidance. After installing, run `{{project-name}} -h` to get started.

See [scripts.md](doc/scripts.md) for available tools for building, running, and packaging the app.

## Crates

`{{project-name}}` splits its code into several library crates:

- `{{project-name}}-assets`: Contains embedded static files intended to be served from the web application
- `{{project-name}}-client`: Run in the client's browser as a WebAssembly package, includes templates
- `{{project-name}}-controllers`: Contains actix-web HTTP controllers, usually calling methods from `{{project-name}}-service`
- `{{project-name}}-core`: Contains definitions that are shared between server and client
- `{{project-name}}-service`: Contains the primary logic for the application. It receives RequestMessages and emits ResponseMessages
- `{{project-name}}-templates`: Contains Maud templates used by the server to render responses
- `{{project-name}}`: Stored in the root of the project, this is the app's main library and binary

## Config Directory

By default, {{project-name}} stores config files in your system's user configuration directory.

- macOS: ~/Library/Application Support/{{project-name}}
- Linux: ~/.config/{{project-name}}
- Windows: %APPDATA%/{{project-name}}/{{project-name}}
