#include <stdint.h>

void debug(int64_t n);
int64_t collatz_n(int64_t n);

int64_t main() {
	int64_t i = collatz_n(234);
	debug(i);
}

void debug(int64_t n) {
	*((volatile int64_t*) 0x100000000) = n;
}

int64_t collatz_n(int64_t n) {
	debug(n);
	int64_t i = 0;

	while (n != 1) {
		if (n % 2 == 0) {
			n = n / 2;
		} else {
			n = 3*n + 1;
		}
		debug(n);
		i = i + 1;
	}

	return i;
}
