{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    libGL
    pkg-config
    wayland
    libxkbcommon
    alsa-lib
    vulkan-loader
  ];

  LD_LIBRARY_PATH =
    builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" [pkgs.libGL pkgs.wayland pkgs.libxkbcommon];

  shellHook = ''
    echo "Entered Shell"
  '';
}
