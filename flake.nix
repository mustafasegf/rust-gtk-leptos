{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            clang-tools
            clang
            pkg-config
            gtk3
            gtk4
            glib
            cmake
            pcre
            xorg.libpthreadstubs
            xorg.libXdmcp
            libuuid
            libselinux
            libsepol
            libxkbcommon
            epoxy
            at-spi2-core
            dbus
            (rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.complete.override { }))
          ];
          shellHook = ''
            export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk4}/share/gsettings-schemas/${gtk4.name}:$XDG_DATA_DIRS
          '';
        };
      }
    );
}

