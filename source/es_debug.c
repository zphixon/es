
#include <stdlib.h>
#include "es.h"

void *es_debug_wrap_malloc(size_t c, char *file, int lnum) {
    void *tmp = malloc(c);
    printf("malloc:  %p -> %zu\t\t%s:%d\n", tmp, c, file, lnum);
    return tmp;
}

void *es_debug_wrap_calloc(size_t c, size_t s, char *file, int lnum) {
    void *tmp = calloc(c, s);
    printf("calloc:  %p -> %zu : %zu\t%s:%d\n", tmp, c, s, file, lnum);
    return tmp;
}

void *es_debug_wrap_realloc(void *c, size_t s, char *file, int lnum) {
    printf("realloc: %p -> %zu\t%s:%d\n", c, s, file, lnum);
    return realloc(c, s);
}

void es_debug_wrap_free(void* s, char *file, int lnum) {
    printf("free:    %p\t%s:%d\n", s, file, lnum);
    free(s);
}

