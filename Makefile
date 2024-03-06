OUT = a.out
IN = text.txt
FLAGS = -d

$(OUT) : $(IN).ll
	clang $(IN).ll

$(IN).ll :
	cargo run -- $(FLAGS) $(IN)

clean :
	rm -f $(OUT) *.ll

run : $(OUT)
	./$(OUT)
	make -s clean
