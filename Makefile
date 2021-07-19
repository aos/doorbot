.PHONY: all build copy run

all: build copy run

build:
	cargo build --target=arm-unknown-linux-gnueabihf

copy:
	scp target/arm-unknown-linux-gnueabihf/debug/doorbot pi@192.168.20.254:

run:
	ssh -t pi@192.168.20.254 ./doorbot
