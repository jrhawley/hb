{
  description = "hb development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain =
            let
              rust = super.rust-bin;
            in
            if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
              rust.stable.latest.default;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
      name = "hb";
    in rec {
      packages.${name} = pkgs.rustPlatform.buildRustPackage rec {
        pname = "hb";
        version = "0.3.0";
        src = ./.;
        cargoSha256 = "sha256-kbpOJZaOJF0EVSKqOm72yTsAkd/Xlf2OkYc0eRfR+ts=";
      };

      # `nix build`
      defaultPackage = packages.${name};

      # `nix run`
      apps.${name} = flake-utils.lib.mkApp {
        inherit name;
        drv = packages.${name};
      };
      defaultApp = apps.${name};

      # `nix develop`
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-make
          cargo-nextest
          cargo-watch
          rust-analyzer
        ];
      };      
      devShells.default = devShell;
    });
}
