with import <nixpkgs> {};

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    protobuf
    nodejs
  ];
}
