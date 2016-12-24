
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>

#ifndef ES_HEADER
#define ES_HEADER
typedef struct {
    uint64_t x;
    uint64_t y;
    uint64_t pos;
} es_cursor;

typedef struct {
    char *filename;
    char **lines;
    uint64_t lines_last;
    uint64_t length;
    bool saved;
    bool real;
    uint64_t id;
} es_buffer;

typedef struct {
    es_cursor cursor;
    uint64_t window_count;
    size_t buffer_current;
    es_buffer *buffers;
    uint64_t buffer_count;
    // global mode
    // macro mode
    // micro mode
    // keystroke buffer?
    // options?
    // status line?
} es_editor;

#else
#endif

// es_editor
void es_editor_setup(es_editor *es);
void es_editor_free(es_editor *es);

// es_buffer
void es_buffer_setup(es_editor *es);

void es_buffer_filename_set(es_editor *es, char *filename);
void es_buffer_set_real(es_editor *es, bool real);
void es_buffer_save_file(es_editor *es);
int es_buffer_open_file(es_editor *es);
void es_buffer_append_line(es_editor *es, char *line);
void es_buffer_set_line(es_editor *es, char *line, uint64_t which);
char **es_buffer_tokenize(const char *str);

es_buffer es_buffer_current(es_editor *es);

// es_cursor
void es_cursor_setup(es_editor *es);

void es_cursor_up(es_editor *es);
void es_cursor_down(es_editor *es);
void es_cursor_left(es_editor *es);
void es_cursor_right(es_editor *es);
