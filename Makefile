.PHONY: all build copy run

TARGET=arm-unknown-linux-gnueabi
REMOTE_HOST=door@192.168.20.17

all: build-release copy-release run

dev: build-dev copy-dev run

build-dev:
	cargo build --target=${TARGET}

build-release:
	cargo build --target=${TARGET} --release

copy-dev:
	scp target/${TARGET}/debug/doorbot ${REMOTE_HOST}:

copy-release:
	scp target/${TARGET}/release/doorbot ${REMOTE_HOST}:

run:
	ssh -t ${REMOTE_HOST} PORT=9000 ./doorbot
