{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy openssl openssl.dev ];
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
            echo "Entering devShell"
            export PS1="\[\033[4;31m\](devSheel)\[\033[0;00m\] $PS1"
          '';
        };
      }
    );
}
