.PHONY: all build copy run

TARGET=arm-unknown-linux-gnueabihf

all: build-release copy-release run

dev: build-dev copy-dev run

build-dev:
	cargo build --target=${TARGET}

build-release:
	cargo build --target=${TARGET} --release

copy-dev:
	scp target/arm-unknown-linux-gnueabihf/debug/doorbot pi@192.168.20.254:

copy-release:
	scp target/arm-unknown-linux-gnueabihf/release/doorbot pi@192.168.20.254:

run:
	ssh -t pi@192.168.20.254 PORT=9000 ./doorbot
