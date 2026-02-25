install:
	cargo build --release
	sudo cp target/release/git-conflict-cli /usr/local/bin/gcm

uninstall:
	sudo rm /usr/local/bin/gcm
