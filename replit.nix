{pkgs}: {
  deps = [
    pkgs.atk
    pkgs.at-spi2-atk
    pkgs.libsoup_3
    pkgs.pango
    pkgs.cairo
    pkgs.gdk-pixbuf
    pkgs.gtk4
    pkgs.glib
    pkgs.webkitgtk_4_1
    pkgs.libGL
    pkgs.mesa
    pkgs.xorg.libxcb
    pkgs.xorg.libXi
    pkgs.xorg.libXrandr
    pkgs.xorg.libXcursor
    pkgs.xorg.libX11
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.fontconfig
    pkgs.openssl
    pkgs.pkg-config
  ];
}
