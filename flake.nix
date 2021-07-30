{
  inputs = {
    utils.url = github:numtide/flake-utils;
    naersk.url = github:nmattia/naersk;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlay
          (final: prev: {
            rustc = final.latest.rustChannels.nightly.rust;
            cargo = final.latest.rustChannels.nightly.rust;
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        naersk-lib = naersk.lib."${system}".override { inherit (pkgs) rustc cargo; };
      in
      rec {
        defaultPackage = packages.vimbot;
        packages.vimbot = naersk-lib.buildPackage {
          pname = "vimbot";
          root = ./.;
          buildInputs = with pkgs; [ libopus ffmpeg pkgconfig ];
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo libopus ffmpeg pkgconfig ];
        };
      });
}
