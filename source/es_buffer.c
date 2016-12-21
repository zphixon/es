
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "es.h"

void es_buffer_setup(es_editor *es) {
    es->windows[0].buffers = (es_buffer*) malloc(sizeof(es_buffer) * 2);

    es->windows[0].buffers[0].id = 1;
    // temporary
    es->windows[0].buffers[0].content = (char*) malloc(sizeof(char) * 30);
    es->windows[0].buffers[0].filename = (char*) malloc(sizeof(char) * 300);

    // temporary
    strcpy(es->windows[0].buffers[0].content, "testarino\nthis is a test");
    strcpy(es->windows[0].buffers[0].filename, "filename yo");

    // last buffer in struct, used for freeing stuff
    // doesn't have any malloc'd elements
    es->windows[0].buffers[1].id = 0;
}
