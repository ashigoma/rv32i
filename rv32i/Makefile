.PHONY: all check clean

all:
	$(MAKE) -C src/cpu
	$(MAKE) -C src/test

check: all
	$(MAKE) -C src/test check

clean:
	rm -rf build