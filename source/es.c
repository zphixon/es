
#include "es.h"
#include "buffer.h"
#include "cursor.h"
#include "stretchy_buffer.h"

void es_setup(es_editor *es) {
    es_buffer_default(es);
    es_cursor_default(es);
}

void es_editor_free(es_editor *es) {
    sb_free(es->buffer.content);
}

void es_line_add(es_editor *es, char *line) {
    sb_push(es->buffer.content, line);
}

