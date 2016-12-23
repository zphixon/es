
#include <stdlib.h>
#include "es_buffer.h"
#include "es_cursor.h"

void es_editor_setup(es_editor *es) {
    es_buffer_setup(es);
    es_cursor_setup(es);
}

void es_editor_free(es_editor *es) {
    for (uint64_t j = 0; j < es->buffer_count; j++) {
        free(es->buffers[j].filename);
        for (size_t k = 0; k <= es->buffers[j].lines_last; k++)
            free(es->buffers[j].lines[k]);
        free(es->buffers[j].lines);
    }
    free(es->buffers);
    free(es);
}

