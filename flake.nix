{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    futils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, futils } @ inputs:
    let
      inherit (nixpkgs) lib;
      inherit (lib) recursiveUpdate;
      inherit (futils.lib) eachDefaultSystem defaultSystems;

      nixpkgsFor = lib.genAttrs defaultSystems (system: import nixpkgs {
        inherit system;
      });
    in
    (eachDefaultSystem (system:
      let
        pkgs = nixpkgsFor.${system};
      in
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            gcc
          ];
          buildInputs = with pkgs; [
            clippy
            cargo-audit
            cargo-tarpaulin
            git
            rustfmt
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    ));
}
