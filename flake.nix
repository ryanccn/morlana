# SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    ferrix.url = "github:ryanccn/ferrix";
  };

  outputs =
    { nixpkgs, ferrix, ... }@inputs:
    ferrix.lib.mkFlake inputs {
      root = ./.;
      systems = nixpkgs.lib.platforms.darwin;
      completions.enable = true;

      buildInputs = pkgs: [ pkgs.sqlite ];

      env = {
        dontCargoUseSystemLibs = "1";
        LIBSQLITE3_SYS_USE_PKG_CONFIG = "1";
      };
    };
}
