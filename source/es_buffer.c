
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "es.h"
#include "es_window.h"

void es_buffer_setup(es_editor *es) {
    es->windows[es->window_current].buffers = malloc(sizeof(es_buffer) * 1);

    es->windows[es->window_current].buffer_count = 1;
    es->buffer_current = 0;
    es->windows[es->window_current].buffers[es->buffer_current].saved = false;
    es->windows[es->window_current].buffers[es->buffer_current].id = 0;

    // temporary
    //es->windows[es->window_current].buffers[es->buffer_current].content = malloc(sizeof(char) * 30);
    //es->windows[es->window_current].buffers[es->buffer_current].filename = malloc(sizeof(char) * 300);
    //strcpy(es->windows[es->window_current].buffers[es->buffer_current].content, "testarino\nthis is a test");
    //strcpy(es->windows[es->window_current].buffers[es->buffer_current].filename, "filename yo");
}

void es_buffer_filename_set(es_editor *es, char *new) {
    free(es->windows[es->window_current].buffers[es->buffer_current].filename);
    es->windows[es->window_current].buffers[es->buffer_current].filename = new;
}

es_buffer es_buffer_current(es_editor *es) {
    return es_window_current(es).buffers[es->buffer_current];
}

