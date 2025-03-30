{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  nixConfig = {
    extra-substituters = [ "https://ryanccn.cachix.org" ];
    extra-trusted-public-keys = [ "ryanccn.cachix.org-1:Or82F8DeVLJgjSKCaZmBzbSOhnHj82Of0bGeRniUgLQ=" ];
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
    in
    {
      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};

          mkFlakeCheck =
            {
              name,
              command,
              ...
            }@args:
            pkgs.stdenv.mkDerivation (
              {
                name = "check-${name}";
                inherit (self.packages.${system}.morlana) src;

                buildPhase = ''
                  ${command}
                  touch "$out"
                '';

                doCheck = false;
                dontInstall = true;
                dontFixup = true;
              }
              // (removeAttrs args [
                "name"
                "command"
              ])
            );
        in
        {
          nixfmt = mkFlakeCheck {
            name = "nixfmt";
            command = "find . -name '*.nix' -exec nixfmt --check {} +";

            src = self;
            nativeBuildInputs = with pkgs; [ nixfmt-rfc-style ];
          };

          rustfmt = mkFlakeCheck {
            name = "rustfmt";
            command = "cargo fmt --check";

            nativeBuildInputs = with pkgs; [
              cargo
              rustfmt
            ];
          };

          clippy = mkFlakeCheck {
            name = "clippy";
            command = ''
              cargo clippy --all-features --all-targets \
                --offline --message-format=json \
                | clippy-sarif | tee $out | sarif-fmt
            '';

            nativeBuildInputs = with pkgs; [
              rustPlatform.cargoSetupHook
              cargo
              rustc
              clippy
              clippy-sarif
              sarif-fmt
            ];

            inherit (self.packages.${system}.morlana) cargoDeps;
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustfmt
              clippy
              rust-analyzer
            ];

            inputsFrom = [ self.packages.${system}.morlana ];

            env = {
              RUST_BACKTRACE = 1;
              RUST_SRC_PATH = toString pkgs.rustPlatform.rustLibSrc;
            };
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages = self.overlays.default null pkgs;
        in
        {
          inherit (packages) morlana;
          default = packages.morlana;
        }
      );

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      overlays.default = _: prev: {
        morlana = prev.callPackage ./package.nix { inherit self; };
      };
    };
}
