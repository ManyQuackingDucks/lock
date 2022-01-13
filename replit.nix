{ pkgs }: {
	deps = [
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
        pkgs.clippy
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}