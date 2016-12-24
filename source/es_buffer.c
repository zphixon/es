
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "es.h"

void es_buffer_setup(es_editor *es) {
    es->buffers = malloc(sizeof(es_buffer) * 1);
    if (es->buffers == NULL) {
        perror("couldn't malloc buffers array");
        exit(1);
    }

    es->buffer_count = 1;
    es->buffer_current = 0;
    es->buffers[es->buffer_current].saved = false;
    es->buffers[es->buffer_current].real = false;
    es->buffers[es->buffer_current].id = 0;
    es->buffers[es->buffer_current].filename = malloc(sizeof(char) * 1);
    es->buffers[es->buffer_current].lines = malloc(sizeof(char*) * 1);
    es->buffers[es->buffer_current].lines_last = 0;
}

void es_buffer_filename_set(es_editor *es, char *filename) {
    free(es_buffer_current(es).filename);
    es->buffers[es->buffer_current].filename = filename;
}

void es_buffer_lines_set(es_editor *es, char **lines) {
    free(es_buffer_current(es).lines);
    es->buffers[es->buffer_current].lines = lines;
}

void es_buffer_real_set(es_editor *es, bool real) {
    es->buffers[es->buffer_current].real = real;
}

void es_buffer_file_save(es_editor *es) {
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

int es_buffer_file_open(es_editor *es) {
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
        char *nl = calloc(strlen(lines[0]) + 2, 1);
        strncpy(nl, lines[0], strlen(lines[0]));
        nl[strlen(lines[0])] = '\n';
        nl[strlen(lines[0]) + 1] = '\0';
        es_buffer_line_set(es, nl, 0);
        free(*lines);
    }
    for (size_t i = 1; lines[i] != NULL; i++) {
        if (lines[i + 1] != NULL) {
            char *nl = calloc(strlen(lines[i]) + 2, 1);
            strncpy(nl, lines[i], strlen(lines[i]));
            nl[strlen(lines[i])] = '\n';
            nl[strlen(lines[i]) + 1] = '\0';
            es_buffer_text_append(es, nl);
        } else {
            es_buffer_text_append(es, strdup(lines[i]));
        }
        free(lines[i]);
    }
    free(buffer);
    free(lines);

    return 0;
}

void es_buffer_text_append(es_editor *es, char *line) {
    es->buffers[es->buffer_current].lines
        = realloc(es->buffers[es->buffer_current].lines,
                  sizeof(char*) * (es->buffers[es->buffer_current].lines_last + 2));
    es->buffers[es->buffer_current].lines_last++;
    uint64_t l = es->buffers[es->buffer_current].lines_last;
    es->buffers[es->buffer_current].lines[l] = line;
}

void es_buffer_line_append(es_editor *es, char *line) {
    char *nl = calloc(2, 1);
    strcpy(nl, "\n");
    es_buffer_char_append(es, nl);
    es_buffer_text_append(es, line);
}

void es_buffer_line_set(es_editor *es, char *line, uint64_t which) {
    es->buffers[es->buffer_current].lines[which] = line;
}

void es_buffer_char_append(es_editor *es, char *c) {
    // crikey
    size_t lastci = strlen(es_buffer_current(es).lines[es_buffer_current(es).lines_last]);
    if (es_buffer_current(es).lines[es_buffer_current(es).lines_last][lastci] == '\n')
    {
        es_buffer_text_append(es, c);
        return;
    }

    char *nl = calloc(lastci + 2, 1);

    strncpy(nl, es->buffers[es->buffer_current].lines[es_buffer_current(es).lines_last], lastci);

    nl[lastci] = *c;
    nl[lastci + 1] = '\0';

    es->buffers[es->buffer_current].lines[es_buffer_current(es).lines_last] = nl;
}

void es_buffer_char_set(es_editor *es, char c, uint64_t which) {
    uint64_t current = 0;
    for (uint64_t i = 0; i < es_buffer_current(es).lines_last; i++) {
        for (uint64_t j = 0; es_buffer_current(es).lines[i][j] != '\0'; j++) {
            if (current == which) {
                es->buffers[es->buffer_current].lines[i][j] = c;
                return;
            }
            current++;
        }
    }
}

es_buffer es_buffer_current(es_editor *es) {
    return es->buffers[es->buffer_current];
}

// thanks SO
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

