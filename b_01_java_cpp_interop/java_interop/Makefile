all: rust
	javac -d . src/Clock.java
	java -Djava.library.path=target/debug/ Clock
rust:
	cargo build