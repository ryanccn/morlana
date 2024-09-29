# morlana

[nix-darwin](https://github.com/LnL7/nix-darwin) utilities, implemented in Rust

```sh
nix run github:ryanccn/morlana
```

## Features

- Support for better build logs with [nix-output-monitor](https://github.com/maralorn/nix-output-monitor)
- Support for diffing with [nvd](https://gitlab.com/khumba/nvd) before switching configurations
- Confirmation prompts for important actions
- Flakes-first (_does not work with channels setups at the moment_)
- Improved uninstaller logic
  - Addresses https://github.com/NixOS/nix/issues/3261
  - Restores `.before-nix-darwin` files automagically
- Works as a standalone binary
- More aesthetic logging

## Getting started

morlana is capable of initializing a nix-darwin system using flakes by itself. In order to get started, run

```sh
nix run github:ryanccn/morlana -- init
```

Alternatively, if you have an existing nix-darwin configuration you want to switch to:

```sh
nix run github:ryanccn/morlana -- switch --flake "<path_to_flake>"
```

To remove nix-darwin from your system:

```sh
nix run github:ryanccn/morlana -- uninstall
```

For more detailed information on available commands and options, run `morlana --help`.

## License

GPLv3
