{
  lib,
  rustPlatform,
  pkg-config,
  clang,
  makeBinaryWrapper,
  mold,
  wrapGAppsHook4,
  gtk4,
  gtk4-layer-shell,
  libinput,
  wayland,
  wayland-protocols,
  dbus,
  libappindicator-gtk3,
  libxkbcommon,
  alsa-lib,
  libpulseaudio,
  pipewire,
  libjack2,
}:
rustPlatform.buildRustPackage (final: let
  inherit (lib.fileset) toSource unions;
  inherit (lib) licenses platforms;
in {
  pname = "keystroke";
  version = "0.1.5";

  src = toSource {
    root = ../.;
    fileset = unions [
      ../src
      ../assets
      ../style
      ../benches
      ../Cargo.lock
      ../Cargo.toml
    ];
  };
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    mold
    clang
    pkg-config
    makeBinaryWrapper
    wrapGAppsHook4
  ];

  buildInputs = [
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

  # Fix for runtime dependencies
  preFixup = ''
    gappsWrapperArgs+=(
      --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath (final.buildInputs ++ [ pipewire libjack2 alsa-lib libpulseaudio ])}"
    )
  '';

  meta = {
    description = "Keystroke - Most beautiful Keystroke visualizer";
    homepage = "https://github.com/linuxmobile/keystroke";
    license = licenses.mit;
    mainProgram = "keystroke";
    platforms = platforms.unix;
  };
})
