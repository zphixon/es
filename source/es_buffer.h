
#include <stdbool.h>
#include "es.h"

void es_buffer_setup(es_editor *es);

void es_buffer_filename_set(es_editor *es, char *filename);
void es_buffer_content_set(es_editor *es, char *content);
void es_buffer_set_real(es_editor *es, bool real);
void es_buffer_save_file(es_editor *es);
int es_buffer_open_file(es_editor *es);

es_buffer es_buffer_current(es_editor *es);

