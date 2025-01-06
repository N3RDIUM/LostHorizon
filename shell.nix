{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    clang lld # Faster compiles
    udev alsa-lib-with-plugins vulkan-loader # Deps
    libxkbcommon wayland # Wayland
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
