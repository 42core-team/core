CC = gcc
CFLAGS = -Wall -Wextra -Werror
SRC_DIR = src
SRC_FILES = $(wildcard $(SRC_DIR)/*.c)
HEADER_FILES = $(wildcard $(SRC_DIR)/*.h)
OBJECTS = $(SRC_FILES:.c=.o)
EXECUTABLE = player

run: all
	RUST_BACKTRACE=1 cargo run --bin game | ./player | cargo run --bin visualizer

all: $(EXECUTABLE)
	cargo build --bin visualizer
	cargo build --bin game
	cp target/debug/game game
	cp target/debug/visualizer visualizer

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $(OBJECTS)

%.o: %.c $(HEADER_FILES)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJECTS)

fclean: clean
	rm -f ${EXECUTABLE}

re: fclean all
