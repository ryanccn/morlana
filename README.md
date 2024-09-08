# morlana

[nix-darwin](https://github.com/LnL7/nix-darwin) utilities, implemented in Rust

```sh
nix run github:ryanccn/morlana
```

## Features

- Built-in [nix-output-monitor](https://github.com/maralorn/nix-output-monitor) and [nvd](https://gitlab.com/khumba/nvd) support
- Confirmation prompts for important actions
- Works as a standalone binary
- Flakes-first (_does not work with channels setups at the moment_)
- More aesthetic logging
- Improved uninstaller logic

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
