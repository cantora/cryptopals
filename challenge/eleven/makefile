
.PHONY: all
all: words.txt
	cargo run

words.txt:
	curl 'http://georgeflanagin.com/dictionaries/mit.10000.words.txt' > $@
