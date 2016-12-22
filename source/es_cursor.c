
#include "es.h"
#include "es_buffer.h"

void es_cursor_setup(es_editor *es) {
    es->cursor.x = 1;
    es->cursor.y = 1;
    es->cursor.pos = 0;
}

void es_cursor_up(es_editor *es) {
    es->cursor.y--;
}

void es_cursor_down(es_editor *es) {
    es->cursor.y++;
}

void es_cursor_left(es_editor *es) {
    es->cursor.x--;
}

void es_cursor_right(es_editor *es) {
    es->cursor.x++;
}

