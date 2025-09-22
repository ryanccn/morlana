((builtins.getFlake "github:nix-darwin/nix-darwin").lib.darwinSystem {
  modules = [
    (
      { lib, ... }:
      let
        inherit (lib) mkForce;
      in
      {
        nixpkgs.hostPlatform = "<HOST_PLATFORM>";

        assertions = mkForce [ ];
        system.activationScripts.checks.text = mkForce "";

        environment.etc = mkForce { };
        launchd.agents = mkForce { };
        launchd.daemons = mkForce { };
        launchd.user.agents = mkForce { };

        nix.enable = false;
      }
    )
  ];
}).system
