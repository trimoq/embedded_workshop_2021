all:
	gcc src/main.c -o my_app -Iinclude -L./lib/ -lc_calls_rust

clean:
	rm my_app || true
	rm -rf ./lib
	rm -rf ./include