.PHONY: run visualizer game all clean fclean

run: all
	cargo run --bin game | cargo run --bin visualizer

visualizer:
	cargo run --bin visualizer

game:
	cargo build --bin game
	./target/debug/game 10 20

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

doc:
	cargo doc --no-deps

test:
	cargo test
