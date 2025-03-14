{
  lib,
  stdenv,
  rustPlatform,
  makeBinaryWrapper,
  installShellFiles,
  nix-output-monitor,
  nvd,
  nix-filter,
  self,
  enableLTO ? true,
  enableOptimizeSize ? false,
  withNom ? true,
  withNvd ? true,
}:
let
  year = builtins.substring 0 4 self.lastModifiedDate;
  month = builtins.substring 4 2 self.lastModifiedDate;
  day = builtins.substring 6 2 self.lastModifiedDate;
in
rustPlatform.buildRustPackage rec {
  pname = passthru.cargoToml.package.name;
  version = "${passthru.cargoToml.package.version}-unstable-${year}-${month}-${day}";

  strictDeps = true;

  src = nix-filter.lib.filter {
    root = self;
    include = [
      "src"
      "Cargo.lock"
      "Cargo.toml"
    ];
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    makeBinaryWrapper
    installShellFiles
  ];

  postInstall =
    let
      wrapBins = (lib.optional withNom nix-output-monitor) ++ (lib.optional withNvd nvd);
    in
    ''
      ${lib.optionalString (withNom || withNvd) ''
        wrapProgram $out/bin/${pname} \
          --suffix PATH : ${lib.makeBinPath wrapBins}
      ''}

      ${lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
        installShellCompletion --cmd ${pname} \
          --bash <("$out/bin/${pname}" completions bash) \
          --zsh <("$out/bin/${pname}" completions zsh) \
          --fish <("$out/bin/${pname}" completions fish)
      ''}
    '';

  doCheck = false;

  env =
    lib.optionalAttrs enableLTO {
      CARGO_PROFILE_RELEASE_LTO = "fat";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
    }
    // lib.optionalAttrs enableOptimizeSize {
      CARGO_PROFILE_RELEASE_OPT_LEVEL = "z";
      CARGO_PROFILE_RELEASE_PANIC = "abort";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
      CARGO_PROFILE_RELEASE_STRIP = "symbols";
    };

  passthru = {
    cargoToml = lib.importTOML ./Cargo.toml;
  };

  meta = with lib; {
    description = "nix-darwin utilities";
    maintainers = with maintainers; [ ryanccn ];
    license = licenses.gpl3Only;
    mainProgram = "morlana";
  };
}
