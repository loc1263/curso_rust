/* Header de la API C de m26_ffi (equivalente a lo que generaría `cbindgen`). */
#ifndef M26_FFI_H
#define M26_FFI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Punto {
    double x;
    double y;
} Punto;

/* Tipo opaco: C solo maneja punteros. */
typedef struct Contador Contador;

typedef void (*Callback)(int valor, void *datos);

int32_t m26_sumar(int32_t a, int32_t b);
double  m26_distancia(Punto a, Punto b);
int32_t m26_escalar(Punto *p, double factor);          /* -1 si p es NULL */

char *m26_saludar(const char *nombre);                   /* liberar con m26_liberar_cadena */
void  m26_liberar_cadena(char *s);

Contador *m26_contador_nuevo(int64_t paso);
int64_t   m26_contador_incrementar(Contador *c);
void      m26_contador_liberar(Contador *c);

int32_t m26_para_cada(int32_t n, Callback cb, void *datos);

#ifdef __cplusplus
}
#endif

#endif /* M26_FFI_H */
