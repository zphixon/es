
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
    for (int i = 0; es->windows[i].id != 0; i++) {
        for (int j = 0; es->windows[i].buffers[j].id != 0; j++) {
            free(es->windows[i].buffers[j].filename);
            free(es->windows[i].buffers[j].content);
        }
        free(es->windows[i].buffers);
    }
    free(es->windows);
    free(es);
}

