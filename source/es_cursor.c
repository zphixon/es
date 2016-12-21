
#include "es.h"

void es_cursor_setup(es_editor *es) {
    es->cursor.x = 1;
    es->cursor.y = 1;
    es->cursor.pos = 0;
}
