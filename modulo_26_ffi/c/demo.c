/* Programa C que usa la biblioteca Rust.
 *
 * Linux / macOS:
 *   cargo build -p m26_ffi
 *   cc c/demo.c -Iinclude -L../target/debug -lm26_ffi -o demo && LD_LIBRARY_PATH=../target/debug ./demo
 *
 * Windows (MSVC, desde una "Developer Command Prompt"):
 *   cargo build -p m26_ffi
 *   cl c\demo.c /Iinclude ..\target\debug\m26_ffi.dll.lib
 *   copy ..\target\debug\m26_ffi.dll . && demo.exe
 */
#include <stdio.h>
#include "m26_ffi.h"

static void acumular(int valor, void *datos) {
    *(int *)datos += valor;
}

int main(void) {
    printf("sumar(2, 3) = %d\n", m26_sumar(2, 3));

    Punto a = {0, 0}, b = {3, 4};
    printf("distancia = %.1f\n", m26_distancia(a, b));

    char *s = m26_saludar("C");
    printf("%s\n", s);
    m26_liberar_cadena(s); /* NO usar free(): la memoria es de Rust */

    Contador *c = m26_contador_nuevo(2);
    m26_contador_incrementar(c);
    printf("contador = %lld\n", (long long)m26_contador_incrementar(c));
    m26_contador_liberar(c);

    int total = 0;
    m26_para_cada(10, acumular, &total);
    printf("total = %d\n", total);
    return 0;
}
