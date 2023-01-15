build: 
	cargo build --release
install:
	sudo cp target/release/vocal /usr/bin
	sudo chmod +x target/release/vocal
clean:
	rm -rf target
