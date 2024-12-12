
CXX = gcc -std=c99

PROGRAM = seasonalemoji


# implementation
all:    $(PROGRAM)

$(PROGRAM):
	@if [ $$USER = root ];\
	then \
		echo "Do not run make to build $(PROGRAM) as root.\nInstalling with make as root is ok.";\
	else \
		cargo build --release;\
	fi

clean: 
	rm -rf ./target

uninstall:
	rm /usr/local/bin/$(PROGRAM)
	rm -rf /usr/local/share/seasonalemoji

install: $(PROGRAM)
	cp target/release/$(PROGRAM) /usr/local/bin
	chmod 755 /usr/local/bin/$(PROGRAM)
	mkdir -p /usr/local/share/seasonalemoji
	chmod 755 /usr/local/share/seasonalemoji
	cp seasonalemoji.json /usr/local/share/seasonalemoji/seasonalemoji.json
	chmod 644 /usr/local/share/seasonalemoji/seasonalemoji.json
