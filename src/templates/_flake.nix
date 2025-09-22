{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nix-darwin, ... }:
    let
      configuration =
        { pkgs, ... }:
        {
          environment.systemPackages = [
            pkgs.vim
          ];

          nix.package = pkgs.nixVersions.latest;
          nix.settings = {
            experimental-features = [
              "nix-command"
              "flakes"
            ];

            extra-platforms = [
              "aarch64-darwin"
              "x86_64-darwin"
            ];

            trusted-users = [ "<USER>" ];
          };

          nixpkgs.hostPlatform = "aarch64-darwin";

          system.configurationRevision = self.rev or self.dirtyRev or null;
          system.stateVersion = 6;
        };
    in
    {
      darwinConfigurations.HOSTNAME = nix-darwin.lib.darwinSystem {
        modules = [ configuration ];
      };
    };
}
