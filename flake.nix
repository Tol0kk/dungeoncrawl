### Flake template for rust svelkit app
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        libraries = with pkgs;[
          glib.out
          dbus.lib
          gtk3
          cairo
          webkitgtk
          libsoup
          libxkbcommon
           libGL

          # WINIT_UNIX_BACKEND=wayland
          wayland
        ];

        packages = with pkgs; [
          fish
          pkg-config
          webkitgtk
          libsoup
          pkgconfig
          rustup # use for rust
          nodePackages.pnpm # Use for pnpm (tauri-selvet)
          clang
          lldb
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages;

          shellHook =
            let
              joinLibs = libs: builtins.concatStringsSep ":" (builtins.map (x: "${x}/lib") libs);
              libs = joinLibs libraries;
            in
            ''
              export LD_LIBRARY_PATH=${libs}:$LD_LIBRARY_PATH
              fish
            '';
        };
      });
}
