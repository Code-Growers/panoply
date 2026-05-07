{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    make-shell.url = "github:nicknovitski/make-shell";

    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      systems,
      make-shell,
      fenix,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ make-shell.flakeModules.default ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          openssl_pkgs = pkgs.openssl;
          fenixSys = fenix.packages.${system};
          craneLib = (crane.mkLib pkgs).overrideToolchain fenixSys.minimal.toolchain;
          buildInputs = [
            pkgs.wayland
            pkgs.libGL
            pkgs.libxkbcommon
          ];

          package = craneLib.buildPackage rec {
            pname = "panoply";
            src = ./panoply;
            doCheck = true;
            nativeBuildInputs = [
              pkgs.makeWrapper
            ];

            runtimeDeps = buildInputs;
            postInstall = ''
              wrapProgram $out/bin/${pname} \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtimeDeps}
            '';
          };
        in
        {
          checks = {
            inherit package;
          };

          packages.default = package;

          make-shells.default = rec {
            env = {
              PKG_CONFIG_PATH = "${openssl_pkgs.dev}/lib/pkgconfig";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
              LD_LIBRARY_PATH = pkgs.lib.strings.concatStrings (
                pkgs.lib.strings.intersperse ":" [
                  "$LD_LIBRARY_PATH"
                  "${builtins.toString (pkgs.lib.makeLibraryPath packages)}"
                ]
              );
            };

            packages = [
              pkgs.pkg-config
              pkgs.openssl.dev

              (fenixSys.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              fenixSys.rust-analyzer
            ]
            ++ buildInputs;
          };
        };
    };
}
