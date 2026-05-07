# Panoply

A small, fast desktop utility for common developer tasks. Built with [egui](https://github.com/emilk/egui) in Rust.

![Panoply Screenshot](screenshot.png)

## Features

- **Base64 Encode / Decode**
- **URL Encode / Decode**
- **UUID Generate**
- **Password Generate**

Quick search and keyboard navigation (arrow keys + enter) to jump between tools.

## Usage on NixOS (with Flakes)

### Run directly

```bash
nix run github:Code-Growers/panoply
```

### Add to your system flake

```nix
{
  inputs.panoply.url = "github:Code-Growers/panoply";

  outputs = { self, nixpkgs, panoply, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      homeConfigurations.user = home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        modules = [
          {
            home.packages = [ panoply.packages.${system}.default ];
          }
        ];
      };
    };
}
```
