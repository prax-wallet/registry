{
  description = "Dev shell for Prax wallet asset registry";
  # inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.crane.url = "github:ipetkov/crane";

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = (crane.mkLib pkgs);
      in
      {

        devShells.default = craneLib.devShell {
          name = "devShell";
          nativeBuildInputs = [ pkgs.bashInteractive ];
          buildInputs = with pkgs; [
            fd
            file
            jq
            just
            openssl
            pnpm
            nodejs_22
          ];
        };
      });
}
