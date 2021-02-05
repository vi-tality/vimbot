{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.vimbot = naersk-lib.buildPackage {
        pname = "vimbot";
        root = ./.;
      };
      defaultPackage = packages.vimbot;

      apps.vimbot = utils.lib.mkApp {
        drv = packages.nix-vimbot;
      };
      defaultApp = apps.nix-vimbot;

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    });
}
