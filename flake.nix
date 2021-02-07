{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs {
         inherit system;
         overlays = [
           rust-overlay.overlay
           (self: super: {
             rustc = self.latest.rustChannels.nightly.rust;
             cargo = self.latest.rustChannels.nightly.rust;
           })
         ];
       };
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.vimbot = naersk-lib.buildPackage {
        pname = "vimbot";
        root = ./.;
        buildInputs = with pkgs; [libopus ffmpeg pkgconfig];
      };
      defaultPackage = packages.vimbot;

      apps.vimbot = utils.lib.mkApp {
        drv = packages.nix-vimbot;
      };
      defaultApp = apps.nix-vimbot;

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo libopus ffmpeg pkgconfig];
      };
    });
}
