run_all:
	echo -e "My output optimized\n" > output_all.txt
	cargo run --bin wordfinder --release >> output_all.txt
	echo -e "\n\nMy output unoptimized\n" >> output_all.txt
	cargo run --bin wordfinder >> output_all.txt
	echo -e "\n\nAI output optimized\n" >> output_all.txt
	cargo run --bin ai --release >> output_all.txt
	echo -e "\n\nAI output unoptimized\n" >> output_all.txt
	cargo run --bin ai >> output_all.txt
	
run_separate:
	cargo run --bin wordfinder --release > output_me_optimized.txt
	cargo run --bin wordfinder > output_me_unoptimized.txt
	cargo run --bin ai --release > output_ai_optimized.txt
	cargo run --bin ai > output_ai_unoptimized.txt
