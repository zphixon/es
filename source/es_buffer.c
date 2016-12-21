
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "es.h"

void es_buffer_setup(es_editor *es) {
    es->windows[es->window_current].buffers = malloc(sizeof(es_buffer) * 1);

    es->windows[es->window_current].buffer_count = 1;
    es->buffer_current = 0;

    // temporary
    es->windows[es->window_current].buffers[es->buffer_current].content = malloc(sizeof(char) * 30);
    es->windows[es->window_current].buffers[es->buffer_current].filename = malloc(sizeof(char) * 300);
    strcpy(es->windows[es->window_current].buffers[es->buffer_current].content, "testarino\nthis is a test");
    strcpy(es->windows[es->window_current].buffers[es->buffer_current].filename, "filename yo");
}

