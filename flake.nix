{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          clippy
          rust-analyzer
          rustc
          rustfmt
        ];
      };

      packages.${system}.default = import ./package.nix {
        lib = pkgs.lib;
        fetchFromGitHub = pkgs.fetchFromGitHub;
        rustPlatform = pkgs.rustPlatform;
      };
    };
}
