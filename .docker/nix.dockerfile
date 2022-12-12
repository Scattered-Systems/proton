FROM nixos/nix

COPY ../scripts ./

RUN nix-shell 