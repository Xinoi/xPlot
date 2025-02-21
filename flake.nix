{

  description = "Dev Shell forr macroquad";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {self, nixpkgs}:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShell.${system} = pkgs.mkShell {
        buildInputs = [
          pkgs.pkg-config 
          pkgs.xorg.libX11 
          pkgs.xorg.libXi 
          pkgs.libGL
          pkgs.libxkbcommon
        ];
      LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
        "${pkgs.xorg.libX11}/lib"
        "${pkgs.xorg.libXi}/lib"
        "${pkgs.libGL}/lib"
        "${pkgs.libxkbcommon}/lib"
      ]; 
      };
    };
}
