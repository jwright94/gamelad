

all: src/load_immediate.s
	rgbasm src/load_immediate.s -o load_immediate.o
	rgblink load_immediate.o -o bin/load_immediate
	rm load_immediate.o

all: src/load_reg.s
	rgbasm src/load_reg.s -o load_reg.o
	rgblink load_reg.o -o bin/load_reg
	rm load_reg.o

jmp_addr: src/jmp_addr.s
	rgbasm src/jmp_addr.s -o jmp_addr.o
	rgblink jmp_addr.o -o bin/jmp_addr
	rm jmp_addr.o