# Features

## Shared Code

The application is split into five crates, with `core` being shared between the server binary and WASM. Shared messages are passed across a websocket in text or binary form.

## Desktop Apps

On MacOS, Linux, and Windows, the app starts the server on an ephemeral port and creates a native webview pointed to it (or you can start the bare server via CLI argument).

## Cross-built

The app is compiled by [TravisCI](https://travis-ci.org) and [AppVeyor](https://appveyor.com), publishing binaries for a variety of desktop and mobile architectures.

## Versioned Publishing

Scripts provided to publish to [crates.io](https://crates.io) and trigger CI publishing to [Github Releases](https://github.com).

## JavaScript Interop

WASM-to-JavaScript helpers provided by [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), allowing DOM manipulation and simple event handlers.

## HTML Templating

Loads of [maud](https://maud.lambda.xyz) templates, integrating with the user's session and theme. Helpers provided for UI components, flash messages, reverse routing, and backtraces.

## WebSocket

WebSocket connections (using [serde_json](https://serde.rs) in debug mode, [bincode](https://docs.rs/bincode) in release), passing messages defined by shared code.

## HTTP Server

High-performance server implementation provided by [actix-web](https://actix.rs), with integrations to pass app configuration and user data to each request.

## Multi-project Build

Code is spread across several crates, splitting shared code and templates to different published libraries.

## UIKit Integration

SCSS stylesheets extending [UIKit](https://getuikit.com), which is included as a git submodule.

## User Theme

Light mode, dark mode, and color selection make a good starting point for user sessions, supported by all the template components.

## Logging

File and console-based logging using [slog](https://github.com/slog-rs/slog) on the server (including backtraces and source location), and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) on the client.
