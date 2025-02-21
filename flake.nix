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
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.pkg-config 
          pkgs.libx11
        ];
      shellHook = ''
        export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:${pkgs.libx11.lib}/lib"
        # ... other environment variables
      '';
      };
    };
}
