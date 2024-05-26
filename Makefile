profile = RUSTFLAGS='-g' cargo build --release; \
	valgrind --tool=callgrind --callgrind-out-file=data/profiler/callgrind.out	\
		--collect-jumps=yes --simulate-cache=yes		\
		./target/release/huffman-coding

profile:
	$(call profile)
