
#include <stdlib.h>
#include "es_buffer.h"
#include "es_window.h"
#include "es_cursor.h"

void es_editor_setup(es_editor *es) {
    es_window_setup(es);
    es_buffer_setup(es);
    es_cursor_setup(es);
}

void es_editor_free(es_editor *es) {
    for (uint64_t i = 0; i < es->window_count; i++) {
        for (uint64_t j = 0; j < es->windows[i].buffer_count; j++) {
            free(es->windows[i].buffers[j].filename);
            for (size_t k = 0; k <= es->windows[i].buffers[j].lines_last; k++)
                free(es->windows[i].buffers[j].lines[k]);
            free(es->windows[i].buffers[j].lines);
        }
        free(es->windows[i].buffers);
    }
    free(es->windows);
    free(es);
}

