build:
	gcc src/main.c -o filer_c
	g++ src/main.cpp -o filer_cpp
	cargo build --release
	cp target/release/filer_rs .
	cargo build --target x86_64-unknown-linux-musl --release
	cp ./target/x86_64-unknown-linux-musl/release/filer_rs ./filer_rs_musl

run: build
	./filer_c
	./filer_cpp
	./filer_rs
	./filer_rs_musl

win:
	x86_64-w64-mingw32-gcc src/main.c -o filer_c_win.exe
	x86_64-w64-mingw32-g++ -o filer_cpp_win.exe -mthreads -static src/main.cpp
	cargo build --target x86_64-pc-windows-gnu
	wine filer_c_win.exe
	wine filer_cpp_win.exe
	cp filer_c_win.exe ~/VirtualBox\ VMs/shared/workshop/
	cp filer_cpp_win.exe ~/VirtualBox\ VMs/shared/workshop/
	cp ./target/x86_64-pc-windows-gnu/debug/filer_rs.exe ~/VirtualBox\ VMs/shared/workshop/filer_rs_win.exe
	cp ./target/x86_64-pc-windows-gnu/debug/filer_rs.exe ~/VirtualBox\ VMs/shared/workshop/filer_rs_win.exe

vps_version:
	./glibc_version.sh
	ssh pfudi "~/glibc_version.sh"

vps: build vps_version
	scp ./filer_rs_musl pfudi:~/workshop
	ssh pfudi "cd workshop&&./filer_rs_musl"

vps_fail: build vps_version
	scp ./filer_rs pfudi:~/workshop
	ssh pfudi "cd workshop&&./filer_rs"

clean:
	-rm filer_c
	-rm filer_cpp
	-rm filer_rs
	-rm filer_c_win.exe
	-rm filer_cpp_win.exe
	-rm filer_rs_musl
	cargo clean

test: run win vps
	make clean