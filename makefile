all: cleanplayground cleansnek build install

cleanplayground:
	cd playground; rm -rf build;

clean:
	cd snek-core; rm -rf build;

cleansnek:
	cd snek-core; rm -rf build;

build:
	cd snek-core; wasm-pack build --target web --out-dir build --release;

install:
	cp -R snek-core/build playground/build
