{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    naerskLib = pkgs.callPackage naersk {};
  in {
    packages.x86_64-linux.default = naerskLib.buildPackage {
      src = ./.;
      buildInputs = with pkgs; [
        libGL
        wayland
        libxkbcommon
        alsa-lib
        vulkan-loader
        kdePackages.wayland-protocols
        xkeyboard-config
      ];

      nativeBuildInputs = with pkgs; [
        pkg-config
        makeWrapper
      ];

      postInstall = ''
        wrapProgram $out/bin/stride \
          --prefix LD_LIBRARY_PATH : "${pkgs.wayland}/lib" \
          --set XKB_CONFIG_ROOT "${pkgs.xkeyboard-config}/share/X11/xkb"
      '';
    };
  };
}
