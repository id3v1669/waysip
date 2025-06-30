{
  description = "Waysip devel";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default-linux";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    systems,
    fenix,
    ...
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);

    pkgsFor = system:
      import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };
  in {
    packages = eachSystem (system: {
      default = nixpkgs.legacyPackages.${system}.callPackage ./nix/package.nix {fenix = fenix;};
    });

    devShells = eachSystem (system: rec {
      default = nightly;
      nightly = (pkgsFor system).callPackage ./nix/shell-nightly.nix {fenix = fenix;};
      stable = (pkgsFor system).callPackage ./nix/shell.nix {};
    });

    formatter.x86_64-linux = inputs.nixpkgs.legacyPackages.x86_64-linux.alejandra;
  };
}
