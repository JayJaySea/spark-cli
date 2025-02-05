let 
    sources = import ./nix/sources.nix;
    rust = import ./nix/rust.nix { inherit sources; };
    pkgs = import sources.nixpkgs {};
    deps = [
        pkgs.hello

        # keep this line if you use bash
        pkgs.bashInteractive
        pkgs.sqlite
        rust
      ];
in
pkgs.mkShell {
  buildInputs = deps;
  HELLO="world";
  LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath deps}";
}
