CC = gcc
CFLAGS = -Wall -Werror -fPIC -Iinclude -IC:/Python312/include
LDFLAGS = -shared -LC:/Python312/libs
LIBS = -lpython312 -lm
TARGET = fft.pyd
SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)
INSTALL_DIR ?= ../../build/lib

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(OBJ) $(LDFLAGS) $(LIBS) -o $@

%.o: %.c $(wildcard include/*.h)
	$(CC) $(CFLAGS) -c $< -o $@

install: $(TARGET)
	@if not exist "$(INSTALL_DIR)" @mkdir "$(INSTALL_DIR)"
	@copy /Y $(TARGET) "$(INSTALL_DIR)/$(TARGET)"

clean:
	del /Q $(OBJ) "$(TARGET)"