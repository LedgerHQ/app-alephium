docker-image:
	@docker build -t ledger-alephium-app-builder:latest configs

build:
	@docker run --rm -v $(shell pwd):/app -v ledger-alephium-cargo:/opt/.cargo ledger-alephium-app-builder:latest \
		bash -c " \
			cd rust-app && \
			echo 'Building nanos app' && \
			cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=../configs/nanos.json && \
			echo 'Building nanosplus app' && \
			cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=../configs/nanosplus.json && \
			echo 'Building nanox app' && \
			cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=../configs/nanox.json \
		"

check:
	@docker run --rm -v $(shell pwd):/app -v ledger-alephium-cargo:/opt/.cargo ledger-alephium-app-builder:latest \
		bash -c " \
			cd rust-app && \
			echo 'Cargo fmt' && \
			cargo fmt --all -- --check && \
			echo 'Cargo clippy' && \
			cargo clippy -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=../configs/nanos.json \
		"

debug:
	@docker run --rm -it -v $(shell pwd):/app -v ledger-alephium-cargo:/opt/.cargo ledger-alephium-app-builder:latest

update-configs:
	curl https://raw.githubusercontent.com/LedgerHQ/ledger-nanos-sdk/master/nanos.json --output configs/nanos.json
	curl https://raw.githubusercontent.com/LedgerHQ/ledger-nanos-sdk/master/nanosplus.json --output configs/nanosplus.json
	curl https://raw.githubusercontent.com/LedgerHQ/ledger-nanos-sdk/master/nanox.json --output configs/nanox.json
