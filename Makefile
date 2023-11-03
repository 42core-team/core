run: all
	RUST_BACKTRACE=1 cargo run --bin game | cargo run --bin visualizer

visualizer:
	cargo run --bin visualizer

game:
	cargo run --bin game

all: $(EXECUTABLE)
	cargo build --bin visualizer
	cargo build --bin game

clean:
	rm -f src/game/game
	rm -f src/visualizer/visualizer

fclean: clean
	rm -f src/game/game
	rm -f src/visualizer/visualizer

re: fclean all

test:
	cargo test
