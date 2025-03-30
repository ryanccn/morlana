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
    {
      self,
      nix-darwin,
      ...
    }@inputs:
    {
      darwinConfigurations.HOSTNAME = nix-darwin.lib.darwinSystem {
        modules = [
          ./system.nix
        ];

        specialArgs = {
          inherit self inputs;
        };
      };
    };
}
