.PHONY: all clean

all:
	$(MAKE) -C src/test
	$(MAKE) -C src/cpu

check: all
	$(MAKE) -C src/test check

clean:
	rm -rf build