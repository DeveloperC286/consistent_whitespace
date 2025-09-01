let
  # https://github.com/NixOS/nixpkgs/tree/nixpkgs-unstable
  spec = { commit = "c5e2e42c112de623adfd662b3e51f0805bf9ff83";
           sha = "05licmf7rasjks1fv4zv5rsq9b1642mpdccw0nlfd0jnw8gal3hi"; };
  pkgs = import (builtins.fetchTarball {
           url = "https://github.com/NixOS/nixpkgs/archive/${spec.commit}.tar.gz";
           sha256 = spec.sha;
         }) {};
in
pkgs.mkShell {
  buildInputs = [
    # Rust.
    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
    # Shell scripts.
    pkgs.shfmt
    # GitHub Action Workflows.
    pkgs.yamlfmt
    pkgs.actionlint
    # Deploying.
    pkgs.gh
    # End to end tests.
    pkgs.python313
    pkgs.python313Packages.autopep8
    pkgs.python313Packages.behave
  ];
}
