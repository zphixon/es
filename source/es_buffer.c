
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "es.h"
#include "es_buffer.h"
#include "es_window.h"

void es_buffer_setup(es_editor *es) {
    es->windows[es->window_current].buffers = malloc(sizeof(es_buffer) * 1);

    es->windows[es->window_current].buffer_count = 1;
    es->buffer_current = 0;
    es->windows[es->window_current].buffers[es->buffer_current].saved = false;
    es->windows[es->window_current].buffers[es->buffer_current].real = false;
    es->windows[es->window_current].buffers[es->buffer_current].id = 0;
    es->windows[es->window_current].buffers[es->buffer_current].filename = malloc(sizeof(char) * 1);
    es->windows[es->window_current].buffers[es->buffer_current].content = malloc(sizeof(char) * 1);
}

void es_buffer_filename_set(es_editor *es, char *filename) {
    free(es_buffer_current(es).filename);
    es->windows[es->window_current].buffers[es->buffer_current].filename = filename;
}

void es_buffer_content_set(es_editor *es, char *content) {
    free(es_buffer_current(es).content);
    es->windows[es->window_current].buffers[es->buffer_current].content = content;
}

void es_buffer_set_real(es_editor *es, bool real) {
    es->windows[es->window_current].buffers[es->buffer_current].real = real;
}

void es_buffer_save_file(es_editor *es) {
    FILE *fp;
    fp = fopen(es_buffer_current(es).filename, "w");
    fwrite(es_buffer_current(es).content, sizeof(char), strlen(es_buffer_current(es).content), fp);
    fclose(fp);
}

void es_buffer_open_file(es_editor *es) {
    FILE *fp;
    long lSize;
    char *buffer;
    fp = fopen(es_buffer_current(es).filename, "r");
    if (!fp)
        return;
    fseek(fp, 0L, SEEK_END);
    lSize = ftell(fp);
    rewind(fp);
    buffer = calloc(1, lSize + 1);
    if (!buffer) {
        fclose(fp);
        return;
    }
    if (1 != fread(buffer, lSize, 1, fp)) {
        fclose(fp);
        return;
    }
    fclose(fp);

    es_buffer_content_set(es, buffer);
    es_buffer_set_real(es, true);
}

es_buffer es_buffer_current(es_editor *es) {
    return es_window_current(es).buffers[es->buffer_current];
}

