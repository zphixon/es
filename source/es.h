
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

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
} es_buffer;

typedef struct {
    es_buffer *buffers;
    uint64_t buffer_count;
    uint64_t id;
    // position
} es_window;

typedef struct {
    es_cursor cursor;
    uint64_t window_count;
    size_t window_current;
    size_t buffer_current;
    es_window *windows;
    // global mode
    // macro mode
    // micro mode
    // keystroke buffer?
    // options?
    // status line?
} es_editor;

#else
#endif

void es_editor_setup(es_editor *es);
void es_editor_free(es_editor *es);

