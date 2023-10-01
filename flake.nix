{
    description = "leptos by example website";

    inputs = {
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs = {
                flake-utils.follows = "flake-utils";
            };
        };
        crane = {
          url = "github:ipetkov/crane";
          inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, rust-overlay, nixpkgs, flake-utils, crane }: 
        flake-utils.lib.eachDefaultSystem (system:
        let 
            pkgs = import nixpkgs {
                inherit system;
                overlays = [ (import rust-overlay) ];
            };
            inherit (pkgs) lib;

            rustToolchain = pkgs.rust-bin.selectLatestNightlyWith(
                toolchain: toolchain.default.override 
                {
                    # Set the build targets supported by the toolchain,
                    # wasm32-unknown-unknown is required for trunk.
                    targets = [ "wasm32-unknown-unknown" ];
                }
            );
            craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            in
            {
                checks = {};
                packages = {
                    default = craneLib.buildTrunkPackage {
                        inherit CARGO_BUILD_TARGET;
                        src=./.;
                        pname = "leptos-by-example";
                        trunkIndexPath = "./index.html";
                        trunkExtraBuildArgs = "--public-url=/leptos-by-example";
                    };
                };

                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        rustToolchain
                        binaryen
                        openssl 
                        pkg-config
                        trunk
                        rust-analyzer
                    ];
                };
            }
    );
}
