## nix/create_nix_flake.sh

```sh
#!/bin/bash

# nix flake init # To create a flake from scratch, or:
cat << EOF > ../flake.nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem ( # Each default system replicates this configuration
      system: let
        pkgs = import nixpkgs {inherit system;}; # Import nixpkgs for the current system
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [rustup rust-bindgen elfutils bc bison flex ncurses gnumake libc clang linuxHeaders openssl]
            ++ (with llvmPackages_21; [clang-tools bintools]); # For building
          packages = with pkgs; [gnugrep ripgrep qemu debootstrap]; # For developing
        };
      }
    );
}
EOF

# To ignore Nix's integration with git
git add --intent-to-add flake.*
git update-index --assume-unchanged flake.*
echo "flake.*" >> ../.git/info/exclude

# cd .. && nix develop path:. # To enter devShell
```

