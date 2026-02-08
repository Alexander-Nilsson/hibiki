{
  mkShell,
  stdenvAdapters,
  cargo-watch,
  keystroke,
}:
mkShell.override (old: {
  stdenv = stdenvAdapters.useMoldLinker old.stdenv;
}) {
  inputsFrom = [keystroke];
  packages = [cargo-watch];
}
