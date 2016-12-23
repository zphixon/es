
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "es.h"
#include "es_buffer.h"
#include "es_window.h"

void es_buffer_setup(es_editor *es) {
    es->windows[es->window_current].buffers = malloc(sizeof(es_buffer) * 1);
    if (es->windows[es->window_current].buffers == NULL) {
        perror("couldn't malloc buffers array");
    }

    es->windows[es->window_current].buffer_count = 1;
    es->buffer_current = 0;
    es->windows[es->window_current].buffers[es->buffer_current].saved = false;
    es->windows[es->window_current].buffers[es->buffer_current].real = false;
    es->windows[es->window_current].buffers[es->buffer_current].id = 0;
    es->windows[es->window_current].buffers[es->buffer_current].filename = malloc(sizeof(char) * 1);
    es->windows[es->window_current].buffers[es->buffer_current].lines = malloc(sizeof(char*) * 1);
    es->windows[es->window_current].buffers[es->buffer_current].lines_last = 0;
    es->windows[es->window_current].buffers[es->buffer_current].lines[es_buffer_current(es).lines_last] = calloc(40, 1);
}

void es_buffer_filename_set(es_editor *es, char *filename) {
    free(es_buffer_current(es).filename);
    es->windows[es->window_current].buffers[es->buffer_current].filename = filename;
}

void es_buffer_lines_set(es_editor *es, char **lines) {
    free(es_buffer_current(es).lines);
    es->windows[es->window_current].buffers[es->buffer_current].lines = lines;
}

void es_buffer_set_real(es_editor *es, bool real) {
    es->windows[es->window_current].buffers[es->buffer_current].real = real;
}

void es_buffer_save_file(es_editor *es) {
    FILE *fp;
    fp = fopen(es_buffer_current(es).filename, "w");
    if (fp == NULL) {
        perror("couldn't open file");
        exit(1);
    }

    uint64_t i = 0;
    while (i <= es_buffer_current(es).lines_last) {
        fwrite(es_buffer_current(es).lines[i], sizeof(char), strlen(es_buffer_current(es).lines[i]), fp);
        i++;
    }

    fclose(fp);
}

int es_buffer_open_file(es_editor *es) {
    FILE *fp;
    long lSize;
    char *buffer;

    fp = fopen(es_buffer_current(es).filename, "r");
    if (fp == NULL)
        return 1;

    fseek(fp, 0L, SEEK_END);
    lSize = ftell(fp);
    rewind(fp);

    buffer = calloc(1, lSize + 1);
    if (buffer == NULL) {
        fclose(fp);
        return 1;
    }

    if (fread(buffer, lSize, 1, fp) != 1) {
        fclose(fp);
        free(buffer);
        return 1;
    }

    fclose(fp);

    char **lines = es_buffer_tokenize(buffer);
    if (*lines != NULL) {
        char *nl = calloc(1, strlen(lines[0]) + 1);
        strncpy(nl, lines[0], strlen(lines[0]));
        nl[strlen(lines[0])] = '\n';
        nl[strlen(lines[0]) + 1] = '\0';
        es_buffer_set_line(es, nl, 0);
        free(*lines);
    }
    for (size_t i = 1; lines[i] != NULL; i++) {
        char *nl = calloc(1, strlen(lines[i]) + 1);
        strncpy(nl, lines[i], strlen(lines[i]));
        nl[strlen(lines[i])] = '\n';
        nl[strlen(lines[i]) + 1] = '\0';
        es_buffer_append_line(es, nl);
        free(lines[i]);
    }
    free(buffer);

    return 0;
}

void es_buffer_append_line(es_editor *es, char *line) {
    es->windows[es->window_current].buffers[es->buffer_current].lines
        = realloc(es->windows[es->window_current].buffers[es->buffer_current].lines,
                  sizeof(char*) * (es->windows[es->window_current].buffers[es->buffer_current].lines_last + 1) + 1);
    es->windows[es->window_current].buffers[es->buffer_current].lines_last++;
    uint64_t l = es->windows[es->window_current].buffers[es->buffer_current].lines_last;
    es->windows[es->window_current].buffers[es->buffer_current].lines[l] = line;
}

void es_buffer_set_line(es_editor *es, char *line, uint64_t which) {
    es->windows[es->window_current].buffers[es->buffer_current].lines[which] = line;
}

es_buffer es_buffer_current(es_editor *es) {
    return es_window_current(es).buffers[es->buffer_current];
}

es_buffer *es_buffer_current_p(es_editor *es) {
    return &es->windows[es->window_current].buffers[es->buffer_current];
}

// ~~thanks SO~~
// fuck you SO, you segfaulted valgrind somehow
char **es_buffer_tokenize(const char *str) {
    int count = 0;
    int capacity = 10;

    char **result = malloc(capacity * sizeof(*result));

    const char *e = str;
    if (e) do {
        const char *s = e;

        e = strpbrk(s, "\n");

        if (count >= capacity)
            result = realloc(result, (capacity *= 2) * sizeof(*result));

        result[count++] = e ? strndup(s, e - s) : strdup(s);
    } while (e && *(++e));

    if (count >= capacity)
        result = realloc(result, (capacity += 1) * sizeof(*result));

    result[count++] = 0;

    return result;
}

