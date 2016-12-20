
#include <stdint.h>

typedef struct {
    // 18446744073709551615 probably isn't enough for some people
    uint64_t x;
    uint64_t y;
} es_cursor;

typedef struct {
    char **content;
    uint64_t lines;
} es_buffer;

typedef struct {
    es_buffer buffer;
    es_cursor cursor;
} es_editor;

void es_setup(es_editor *es);
void es_editor_free(es_editor *es);

