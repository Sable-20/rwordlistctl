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
        defaultPackage = with pkgs; naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy openssl ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        devShell = with pkgs; mkShell {
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy openssl man ];
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
	    export PS1="[\[\033[4;31m\](devShell)\[\033[0;00m\] \$] "
          '';
        };
      }
    );
}
