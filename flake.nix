{
  description = "Keystroke - Most beautiful Keystroke visualizer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    systems,
    rust-overlay,
    ...
  }: let
    withOverlay = pkgs: pkgs.extend (import rust-overlay);
    eachSystem = fn:
      nixpkgs.lib.genAttrs
      (import systems)
      (system: fn (withOverlay nixpkgs.legacyPackages.${system}));
  in {
    packages = eachSystem (pkgs: let
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
          "clippy"
          "rustfmt"
        ];
      };
    in {
      default = pkgs.callPackage ./nix/package.nix {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      };
    });
    devShells = eachSystem (pkgs: {
      default = pkgs.callPackage ./nix/shell.nix {
        keystroke = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      };
    });
  };
}
# utils.lib.eachDefaultSystem
# (
#   system: let
#     pkgs = import nixpkgs {inherit system;};
#     toolchain = pkgs.rustPlatform;
#
#     runtimeDeps = with pkgs; [
#       gtk4
#       gtk4-layer-shell
#       libinput
#       wayland
#       wayland-protocols
#       dbus
#       libappindicator-gtk3
#       libxkbcommon
#       alsa-lib
#       libpulseaudio
#       pipewire
#       libjack2
#     ];
#
#     buildInputs = with pkgs; [
#       gtk4
#       gtk4-layer-shell
#       libinput
#       wayland
#       wayland-protocols
#       pkg-config
#       dbus
#       libappindicator-gtk3
#       libxkbcommon
#       alsa-lib
#       libpulseaudio
#       pipewire
#       libjack2
#     ];
#
#     nativeBuildInputs = with pkgs; [
#       pkg-config
#       wrapGAppsHook4
#     ];
#   in rec
#   {
#     packages.default = toolchain.buildRustPackage {
#       pname = "keystroke";
#       version = "0.1.0";
#       src = ./.;
#       cargoLock.lockFile = ./Cargo.lock;
#
#       inherit buildInputs nativeBuildInputs;
#
#       preFixup = ''
#         gappsWrapperArgs+=(
#           --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath runtimeDeps}"
#           --set ALSA_PLUGIN_DIR "${pkgs.pipewire}/lib/alsa-lib"
#         )
#       '';
#     };
#
#     apps.default = utils.lib.mkApp {drv = packages.default;};
#
#     devShells.default = pkgs.mkShell {
#       nativeBuildInputs = [
#         pkgs.rustc
#         pkgs.cargo
#         pkgs.clippy
#         pkgs.rustfmt
#         pkgs.rust-analyzer
#         pkgs.pkg-config
#       ];
#       buildInputs = buildInputs ++ nativeBuildInputs;
#       RUST_SRC_PATH = "${toolchain.rustLibSrc}";
#       PKG_CONFIG_PATH = "${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}";
#
#       shellHook = ''
#         export ALSA_PLUGIN_DIR="${pkgs.pipewire}/lib/alsa-lib"
#       '';
#     };
#   }
# );

