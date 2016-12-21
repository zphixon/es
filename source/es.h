
#include <stdint.h>
#include <stdbool.h>

#ifndef ES_HEADER
#define ES_HEADER
typedef struct {
    uint64_t x;
    uint64_t y;
    uint64_t pos;
} es_cursor;

typedef struct {
    char *filename;
    char *content;
    uint64_t length;
    bool saved;
    uint64_t id;
    // micro mode
} es_buffer;

typedef struct {
    es_buffer *buffers;
    uint64_t bufnum;
    uint64_t id;
    // macro mode
    // position
} es_window;

typedef struct {
    es_cursor cursor;
    uint64_t winnum;
    es_window *windows;
    // global mode
    // keystroke buffer?
    // options?
    // status line?
} es_editor;

#else
#endif

void es_editor_setup(es_editor *es);
void es_editor_free(es_editor *es);

