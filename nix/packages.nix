{
  self,
  lib,
  inputs,
  ...
}: {
  perSystem = {
    config,
    self',
    inputs',
    pkgs,
    ...
  }: let
    deps = import ./dependencies.nix {inherit pkgs;};
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "penguboks";
      version = "0.1.0";

      src = ../.;
      inherit (deps) buildInputs nativeBuildInputs;
      cargoHash = "sha256-you2xfGUbN8a3IW+IuGwegyZYa/JjUwPotayueEvwIE=";
    };
  };
}
