# Scripts

There's scripts in the `./bin` directory to help you build, run, test, and publish {{project-name}}.
They're designed for macOS, but should work on Linux or Windows (via WSL).

- `build`: Runs the code formatter, checks all the projects, then builds binaries for Linux, macOS, Windows, and Docker.
- `build-client`: Runs `wasm-pack` for the client Rust code, and `npm install` to bundle it to JavaScript.
- `build-client-watch`: Builds the client app using `build-client`, then watches for changes in `client`'s code.
- `build-css`: Uses `scss` to compile the stylesheets in `assets/stylesheets`.
- `build-css-watch`: Builds the css resources using `build-css`, then watches for changes in `stylesheets`.
- `build-docker`: Makes a release build, builds a docker image, then exports and zips the output.
- `build-ios`: Creates a `cargo-lipo` universal library for iOS.
- `build-linux`: Build a release-mode binary for Linux.
- `build-mac`: Build a release-mode binary for macOS.
- `build-windows`: Build a release-mode binary for Windows.
- `check`: Runs code statistics, checks for outdated dependencies, then runs cargo-audit and clippy.
- `dev`: Watches the project directories, and runs the main application, restarting when changes are detected.
- `doc`: Runs rustdoc for all projects, linking between projects and using custom logos and styling.
- `flamegraph`: Uses `cargo-flamegraph` to produce an SVG flamegraph of the projects compilation.
- `format`: Runs `rustfmt` on all projects.
- `publish`: Publishes all projects (in a very specific order) to crates.io. Usually used after `version-bump`.
- `run-docker`: Runs the Docker image produced by `build-docker`, exposing an HTTP port.
- `run-release`: Builds the project in release mode and runs it.
- `rustup`: Installs Rust, additional toolchains, and required components. Good for bootstrapping a new machine.
- `version-bump`: Passed two arguments, this script will change all references of the old version to a new version.
