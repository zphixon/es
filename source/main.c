
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include <ncurses.h>
#include "lua.h"
#include "lauxlib.h"

#include "es.h"
#include "es_buffer.h"

int main(int argc, char **argv) {
    // init es stuff
    es_editor *es = malloc(sizeof(es_editor));
    es_editor_setup(es);

    char *filename = malloc(sizeof(char) * 9);
    if (argv[1] != NULL) {
        strcpy(filename, argv[1]);
        es_buffer_filename_set(es, filename);
        es_buffer_set_real(es, true);
    } else {
        strcpy(filename, "Untitled");
        es_buffer_filename_set(es, filename);
    }

    if (es_buffer_current(es).real) {
        if (es_buffer_open_file(es))
            perror("oh noes can't open file");
    } else {
        char *new = malloc(sizeof(char) * 1);
        strcpy(new, "");
        es_buffer_content_set(es, new);
    }

    printf("%s", es_buffer_current(es).content);

    char *test = malloc(sizeof(char) * 40);
    strcpy(test, "this is a testarino");
    es_buffer_content_set(es, test);
    es_buffer_save_file(es);

    // clean up es stuff
    es_editor_free(es);
    return 0;
}

