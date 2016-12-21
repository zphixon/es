
#include "es.h"
#include <stdint.h>

void es_buffer_setup(es_editor *es);
void es_buffer_insert(es_editor *es, char *what, uint64_t where);
void es_buffer_free(es_editor *es, uint64_t which);

