{
  description = "Dev flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixops-flake.url = "github:input-output-hk/nixops-flake";
  };

  outputs = { self, nixpkgs, flake-utils, nixops-flake }:
    flake-utils.lib.eachDefaultSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              gcc
              clang-tools

              cargo
              rustc
              rust-analyzer
              # cmake
              # pkg-config
              nixops-flake.defaultPackage.${system}
            ];
          };
        }
      );
}
