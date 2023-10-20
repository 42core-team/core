CC = gcc
CFLAGS = -Wall -Wextra -Werror
SRC_DIR = src
SRC_FILES = $(wildcard $(SRC_DIR)/*.c)
HEADER_FILES = $(wildcard $(SRC_DIR)/*.h)
OBJECTS = $(SRC_FILES:.c=.o)
EXECUTABLE = player

all: $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $(OBJECTS)

%.o: %.c $(HEADER_FILES)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJECTS)

fclean: clean
	rm -f ${EXECUTABLE}

re: fclean all
