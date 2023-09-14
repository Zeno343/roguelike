{
  description = "A TUI roguelike";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, crane, rust-overlay }: 
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };

    craneLib = (crane.mkLib pkgs).overrideToolchain 
      pkgs.rust-bin.nightly.latest.default;
    
    roguelike = craneLib.buildPackage {
      src = craneLib.path ./.;
    };

  in {
    packages.${system}.default = roguelike;

    devShells.default = craneLib.devShell {
      inputsFrom = [ roguelike ];
    };
  };
}
