
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include <ncurses.h>
//#include "lua.h"
//#include "lauxlib.h"

#include "es.h"
#include "es_buffer.h"

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
        strcpy(filename, argv[1]);
        es_buffer_filename_set(es, filename);
        es_buffer_set_real(es, true);
    } else {
        strcpy(filename, "Untitled");
        es_buffer_filename_set(es, filename);
    }

    if (es_buffer_current(es).real) {
        if (es_buffer_open_file(es)) {
            perror("couldn't open file");
            exit(1);
        }
    }

    char *nl = calloc(1, 11);
    strcpy(nl, "testaroni\n");
    es_buffer_append_line(es, nl);

    es_buffer_save_file(es);

    // do stuff with es

    // clean up es stuff
    es_editor_free(es);
    return 0;
}

