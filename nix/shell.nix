{pkgs}:
pkgs.mkShell rec {
  name = "Waysip-devel";
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy

    # Tools
    scdoc
    cargo-flamegraph
    cargo-audit
    cargo-xbuild
    cargo-deny
  ];
  buildInputs = with pkgs; [
    glib
    pango
    cairo
  ];
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
