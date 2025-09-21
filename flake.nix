# LLM-generated with some manual tweaks based on SO and blogs to resolve GUI deps. Result: bunch of random dependencies. wontfix
{
  description = "A Nix-flake-based Java development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      javaVersion = 21;
      overlays = [
        (final: prev: rec {
          jdk = prev."jdk${toString javaVersion}".override {
             enableJavaFX = true;
          };
          gradle = prev.gradle.override { java = jdk; };
          maven = prev.maven.override { inherit jdk; };
        })
      ];
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" "x86_64-windows" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            gradle
            jdk
            maven
            jdt-language-server
            ant
            xorg.libX11
            xorg.libXrender
            xorg.libXext
            xorg.libXinerama
            xorg.libXrandr
            xorg.libXi
            xorg.libXcursor
            xorg.libXcomposite
            xorg.libXdamage
            xorg.libXtst
            xorg.libXxf86vm
            glibcLocales
            libGL
            libGLU
            gtk3
            libcanberra-gtk3
                        alsaLib
                        pulseaudio
                        atk
                        at-spi2-core
                        dbus-glib
                        mesa
                        mesa_drivers
          ];
          shellHook = ''
            export JAVA_HOME=${pkgs.jdk}/lib/openjdk
            export LD_LIBRARY_PATH=${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXrender}/lib:${pkgs.xorg.libXext}/lib:${pkgs.xorg.libXinerama}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXi}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXcomposite}/lib:${pkgs.xorg.libXdamage}/lib:${pkgs.xorg.libXtst}/lib:${pkgs.xorg.libXxf86vm}/lib:${pkgs.libGL}/lib:${pkgs.libGLU}/lib:${pkgs.gtk3}/lib:${pkgs.libcanberra-gtk3}/lib:$LD_LIBRARY_PATH
            export LC_ALL=en_US.UTF-8
            export LANG=en_US.UTF-8
            export LANGUAGE=en_US.UTF-8
            export GTK_MODULES=canberra-gtk-module
            export GDK_BACKEND=x11
          '';
        };
      });
    };
}
