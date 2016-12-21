
#include <stdlib.h>
#include "es.h"
#include <stdio.h>

void es_window_setup(es_editor *es) {
    es->windows = (es_window*) malloc(sizeof(es_window) * 2);
    es->windows[0].id = 1;
    es->windows[1].id = 0;
}
