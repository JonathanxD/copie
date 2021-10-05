{ pkgs ? import <nixpkgs> {},
  copie ? import ./copie.nix
}:
pkgs.mkShell {
  name="dev-environment";
  buildInputs = [
    copie
  ];
}