{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        pkgs.config.allowUnfree = true;
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            bacon
            bun
            redis
          ];
          nativeBuildInputs = with pkgs; [
          ];

          buildInputs = with pkgs; [
          ];

          shellHook = '''';

        };
      }
    );
}
