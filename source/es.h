
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

#ifdef ES_DEBUG
#define malloc(x) es_debug_wrap_malloc(x, __FILE__, __LINE__)
#define calloc(x, y) es_debug_wrap_calloc(x, y, __FILE__, __LINE__)
#define realloc(x, y) es_debug_wrap_realloc(x, y, __FILE__, __LINE__)
#define free(x) es_debug_wrap_free(x, __FILE__, __LINE__)
#endif

// es_editor
void es_editor_setup(es_editor *es);
void es_editor_free(es_editor *es);

// es_buffer
void es_buffer_setup(es_editor *es);
void es_buffer_filename_set(es_editor *es, char *filename);
void es_buffer_real_set(es_editor *es, bool real);
void es_buffer_file_save(es_editor *es);
int es_buffer_file_open(es_editor *es);
void es_buffer_text_append(es_editor *es, char *line);
void es_buffer_line_append(es_editor *es, char *line);
void es_buffer_lines_set(es_editor *es, char **lines);
void es_buffer_line_set(es_editor *es, char *line, uint64_t which);
char **es_buffer_tokenize(const char *str);
es_buffer es_buffer_current(es_editor *es);
void es_buffer_char_append(es_editor *es, char *c);
void es_buffer_char_set(es_editor *es, char c, uint64_t which);

// es_cursor
void es_cursor_setup(es_editor *es);
void es_cursor_up(es_editor *es);
void es_cursor_down(es_editor *es);
void es_cursor_left(es_editor *es);
void es_cursor_right(es_editor *es);

// es_debug
void *es_debug_wrap_malloc(size_t c, char *file, int lnum);
void *es_debug_wrap_calloc(size_t c, size_t s, char *file, int lnum);
void *es_debug_wrap_realloc(void *c, size_t s, char *file, int lnum);
void es_debug_wrap_free(void* s, char *file, int lnum);

