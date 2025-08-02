{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        libGL
        pkg-config
        wayland
        libxkbcommon
      ];

      LD_LIBRARY_PATH =
        builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" [pkgs.libGL pkgs.wayland pkgs.libxkbcommon];

      shellHook = ''
        echo "Entered Shell"
        exec nu
      '';
    };
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      name = "stride";
      src = ./.;
      buildInputs = with pkgs; [
        libGL
        pkg-config
        wayland
        libxkbcommon
      ];
      cargoBuildFlags = "--release";
      cargoLock.lockFile = ./Cargo.lock;
    };
  };
}
