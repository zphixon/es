
#include <stdlib.h>
#include "es.h"
#include <stdio.h>

void es_window_setup(es_editor *es) {
    es->windows = (es_window*) malloc(sizeof(es_window) * 1);
    es->window_current = 0;
    es->window_count = 1;
    es->windows[es->window_current].buffer_count = 0;
    es->windows[es->window_current].id = 0;
}
