build: 
	cargo build --release
install:
	cp target/release/vocal /usr/bin
	chmod +x target/release/vocal
clean:
	rm -rf target
