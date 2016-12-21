
#include <stdlib.h>
#include <stdio.h>

#include <ncurses.h>
#include "lua.h"
#include "lauxlib.h"

#include "es.h"

int main(int argc, char **argv) {
    // init es stuff
    es_editor *es = (es_editor*) malloc(sizeof(es_editor));
    es_editor_setup(es);

    // clean up es stuff
    es_editor_free(es);
    return 0;
}

