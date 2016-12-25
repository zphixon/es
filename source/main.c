
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include <ncurses.h>
//#include "lua.h"
//#include "lauxlib.h"

#include "es.h"

int main(int argc, char **argv) {
    // init es stuff
    es_editor *es = malloc(sizeof(es_editor));
    if (es == NULL) {
        perror("couldn't malloc es_editor");
        exit(1);
    }

    es_editor_setup(es);

    char *filename = malloc(sizeof(char) * 9);
    if (filename == NULL) {
        perror("couldn't malloc filename");
        exit(1);
    }

    if (argv[1] != NULL) {
        filename = realloc(filename, sizeof(char) * (strlen(argv[1]) + 1));
        strcpy(filename, argv[1]);
        es_buffer_filename_set(es, filename);
        es_buffer_real_set(es, true);
    } else {
        strcpy(filename, "Untitled");
        es_buffer_filename_set(es, filename);
    }

    if (es_buffer_current(es).real) {
        if (es_buffer_file_open(es)) {
            perror("couldn't open file");
            exit(1);
        }
    }

    char *nl = calloc(11, 1);
    strcpy(nl, "testaroni");
    es_buffer_line_append(es, nl);

    char *nl2 = calloc(4, 1);
    strcpy(nl2, "pbn");
    es_buffer_text_append(es, nl2);

    char *nl1 = calloc(2, 1);
    strcpy(nl1, "j");
    es_buffer_char_append(es, nl1);

    es_buffer_char_set(es, 'v', 15);

    es_buffer_file_save(es);

    // do stuff with es

    // clean up es stuff
    es_editor_free(es);

    free(nl1);
    free(nl);
    free(nl2);
    return 0;
}

