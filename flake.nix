{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nix-filter.url = "github:numtide/nix-filter";
  };

  nixConfig = {
    extra-substituters = [ "https://ryanccn.cachix.org" ];
    extra-trusted-public-keys = [ "ryanccn.cachix.org-1:Or82F8DeVLJgjSKCaZmBzbSOhnHj82Of0bGeRniUgLQ=" ];
  };

  outputs =
    {
      self,
      nixpkgs,
      nix-filter,
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
              nativeBuildInputs ? [ ],
              command,
            }:
            pkgs.stdenv.mkDerivation {
              name = "check-${name}";
              inherit nativeBuildInputs;
              inherit (self.packages.${system}.morlana) src cargoDeps;

              buildPhase = ''
                ${command}
                touch "$out"
              '';

              doCheck = false;
              dontInstall = true;
              dontFixup = true;
            };
        in
        {
          nixfmt = mkFlakeCheck {
            name = "nixfmt";
            nativeBuildInputs = with pkgs; [ nixfmt-rfc-style ];
            command = "nixfmt --check .";
          };

          rustfmt = mkFlakeCheck {
            name = "rustfmt";

            nativeBuildInputs = with pkgs; [
              cargo
              rustfmt
            ];

            command = "cargo fmt --check";
          };

          clippy = mkFlakeCheck {
            name = "clippy";

            nativeBuildInputs = with pkgs; [
              rustPlatform.cargoSetupHook
              cargo
              rustc
              clippy
              clippy-sarif
              sarif-fmt
            ];

            command = ''
              cargo clippy --all-features --all-targets \
                --offline --message-format=json \
                | clippy-sarif | tee $out | sarif-fmt
            '';
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

              cargo-audit
              cargo-bloat
              cargo-expand

              libiconv
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
        morlana = prev.callPackage ./default.nix { inherit nix-filter self; };
      };
    };
}
