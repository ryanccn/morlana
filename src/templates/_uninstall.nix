((builtins.getFlake "github:LnL7/nix-darwin").lib.darwinSystem {
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
      }
    )
  ];
}).system
