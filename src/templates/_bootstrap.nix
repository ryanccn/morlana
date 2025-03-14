{
  pkgs,
  inputs,
  config,
  ...
}:
{
  environment.systemPackages = [
    pkgs.vim
  ];

  # services.nix-daemon.enable = true;
  nix.package = pkgs.nix;
  nix.nixPath = [ "nixpkgs=${inputs.nixpkgs.outPath}" ];

  nix.settings = {
    experimental-features = [
      "nix-command"
      "flakes"
    ];

    extra-platforms = [
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    build-users-group = "nixbld";
    trusted-users = [ "<USER>" ];

    nix-path = config.nix.nixPath;
  };

  programs.bash.enable = true;
  programs.zsh.enable = true;
  # programs.fish.enable = true;

  nixpkgs.hostPlatform = "<HOST_PLATFORM>";

  system.stateVersion = 5;
}
