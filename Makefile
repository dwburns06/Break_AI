run:
	cargo run --bin wordfinder > output_me_unoptimized.txt
	cargo run --bin wordfinder --release > output_me_optimized.txt
	cargo run --bin ai > output_ai_unoptimized.txt
	cargo run --bin ai --release > output_ai_optimized.txt
