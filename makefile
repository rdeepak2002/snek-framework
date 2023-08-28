all: clean build

clean:
	cd playground; cd lib; rm -rf snek-core;

build:
	cd snek-core; wasm-pack build --target web --out-dir ../playground/lib/snek-core --release;
