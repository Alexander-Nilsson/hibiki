{
  description = "Wayland keypress display using GTK4 and libinput";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {inherit system;};
        toolchain = pkgs.rustPlatform;

        runtimeDeps = with pkgs; [
          gtk4
          gtk4-layer-shell
          libinput
          wayland
          wayland-protocols
          dbus
          libappindicator-gtk3
          libxkbcommon
          alsa-lib
          libpulseaudio
          pipewire
          libjack2
        ];

        buildInputs = with pkgs; [
          gtk4
          gtk4-layer-shell
          libinput
          wayland
          wayland-protocols
          pkg-config
          dbus
          libappindicator-gtk3
          libxkbcommon
          alsa-lib
          libpulseaudio
          pipewire
          libjack2
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
        ];
      in rec
      {
        packages.default = toolchain.buildRustPackage {
          pname = "keystroke";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          inherit buildInputs nativeBuildInputs;

          preFixup = ''
            gappsWrapperArgs+=(
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath runtimeDeps}"
              --set ALSA_PLUGIN_DIR "${pkgs.pipewire}/lib/alsa-lib"
            )
          '';
        };

        apps.default = utils.lib.mkApp {drv = packages.default;};

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.pkg-config
          ];
          buildInputs = buildInputs ++ nativeBuildInputs;
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
          PKG_CONFIG_PATH = "${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}";

          shellHook = ''
            export ALSA_PLUGIN_DIR="${pkgs.pipewire}/lib/alsa-lib"
          '';
        };
      }
    );
}
