# Installing

## Github Releases

Compiled binaries are available at [Github Releases](https://github.com/{{project-name}}/{{project-name}}/releases). Simply download and extract your platform's distribution, unzip, and place on your path.

## Homebrew

`brew install {{project-name}}/{{project-name}}/{{project-name}}`

## App Stores

Mobile apps are coming soon!

## Cargo

`cargo install {{project-name}}`

## Build From Source

Clone this repository, and use the provided [scripts](scripts.md) to build and run it.

## Fresh Ubuntu

```sudo apt-get install -y build-essential git curl nodejs npm libwebkit2gtk-4.0-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh```
