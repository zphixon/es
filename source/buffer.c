
#include "es.h"
#include "buffer.h"
#include "stretchy_buffer.h"

void es_buffer_default(es_editor *es) {
    es->buffer.content = NULL;
    sb_push(es->buffer.content, (char*)"");
    es->buffer.lines = 1;
}

