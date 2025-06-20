builddir = build
objects = $(builddir)/main.o $(builddir)/term.o

CFLAGS = -Wall -Wextra -pedantic -DDEBUG
execname = tapestry

vpath %.c src/
vpath %.h src/

$(execname): $(objects) | $(builddir)
	$(CC) -o $(execname) $(objects) 

# Pattern rule
$(objects): $(builddir)/%.o : %.c | $(builddir)
	$(CC) $(CFLAGS) -c $< -o $@

$(builddir):
	mkdir -p $(builddir)

main.o: term.h
term.o: escape.h

.PHONY: clean
clean:
	rm -rf $(builddir)

.PHONY: clobber
clobber: clean
	rm -f $(execname)
