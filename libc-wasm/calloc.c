// https://github.com/migueldeicaza/mono-wasm-libc/blob/master/src/malloc/calloc.c

#include <stdlib.h>
#include <errno.h>

void *malloc(size_t);

void *calloc(size_t m, size_t n)
{
	if (n && m > (size_t)-1/n) {
		errno = ENOMEM;
		return 0;
	}
	return malloc(n * m);
}
