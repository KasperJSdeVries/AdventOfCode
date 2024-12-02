{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      nativeBuildInputs = with pkgs; [
        gcc
      ];
    in {
      devShells.default = pkgs.mkShell {
        inherit nativeBuildInputs;
        packages = with pkgs; [
          clang-tools
          nil
        ];
      };

      formatter = pkgs.alejandra;
    });
}
