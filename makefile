SOURCE = main.c

TARGET = naklo

CC = gcc

.PHONY: all 
all: $(TARGET)

$(TARGET): $(SOURCE)
	$(CC) -Wall -g $< -o $@

clean: 
	rm -f $(TARGET)
