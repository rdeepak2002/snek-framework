all: clean build install

clean:
	cd snek-core; rm -rf build;

build:
	cd snek-core; wasm-pack build --target web --out-dir build --release;

install:
	cp -R snek-core/build playground/build
