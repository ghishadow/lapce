{ stdenv, libxkbcommon, libxcb, pkgconfig, cmake, vulkan-loader, assimp, python39, libwayland }:

with import <nixpkgs> {}; {
  vulkanEnv = stdenv.mkDerivation {
    name = "vulkan";
    nativeBuildInputs = [ pkg-config python39 ];
    buildInputs = [ stdenv libxkbcommon cmake libxcb pkgconfig fontconfig vulkan-loader assimp python39 freetype libwayland ];
  };
}
