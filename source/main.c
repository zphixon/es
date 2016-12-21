
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include <ncurses.h>
#include <lua50/lua.h>
#include <lua50/lauxlib.h>

#include "es.h"
#include "es_buffer.h"

int main(int argc, char **argv) {
    // init es stuff
    es_editor *es = malloc(sizeof(es_editor));
    es_editor_setup(es);

    char *filename = malloc(sizeof(char) * 9);
    strcpy(filename, "Untitled");

    if (argv[1] != NULL) {
        filename = realloc(filename, strlen(argv[1]));
        strcpy(filename, argv[1]);
    }

    es_buffer_filename_set(es, filename);

    printf("%s\n", es_buffer_current(es).filename);

    // clean up es stuff
    es_editor_free(es);
    return 0;
}

