{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in
    {
      overlays.default = final: prev: {
        rustToolchain = prev.rust-bin.rust.fromRustupToolchainFile ./rust-toolchain.toml;
      };

      devShells = forAllSystems (pkgs: {
        default =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rust-analyzer-unwrapped
              rustfmt
              clippy
              nixd
              vscode-langservers-extracted
              typescript-language-server
              pkg-config
              openssl
              sqlite
              docker
              just
              bun
              svelte-language-server
              tailwindcss-language-server
              eslint_d
              nixfmt-rfc-style
              prettierd
              docker-compose-language-service
            ];
          };
      });
    };
}
