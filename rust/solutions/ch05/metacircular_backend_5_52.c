/* SPDX-License-Identifier: GPL-3.0-only */
/* Exercise 5.52 run-time support: the tagged-word store the typed
   compiler's C translation runs in.

   Words are `long`. Bit 0 set marks an integer, holding the value
   shifted left by one; a zero word is null; any other word points at
   a heap object whose first field names its kind. Pairs carry the
   option/result encoding the section fixes: `Some` is the pair
   (1 . payload), `None` is (0 . ()), `Ok` is (2 . payload), and
   `Err` is (3 . payload), with the tag an integer word and `()`
   the null word. Vectors grow geometrically under `mc_vec_push`.
   Printing renders integers plainly, null as `()`, pairs dotted,
   vectors bracketed, and strings raw, matching `println!` display.

   Two emitted operations have no runnable lowering yet, because the
   translation unit does not convey what they need: `mc_bind` is
   called without its pattern and `mc_apply_closure` without a code
   table for the body id. Both die loudly naming the missing emitter
   surface instead of guessing; no guest program run here reaches
   them. `main` enters the generated `mc_main`. */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum { K_PAIR, K_VEC, K_STR, K_CLOS } Kind;

typedef struct {
    Kind kind;
} Head;

typedef struct {
    Kind kind;
    long car;
    long cdr;
} Pair;

typedef struct {
    Kind kind;
    size_t length;
    size_t capacity;
    long *items;
} Vec;

typedef struct {
    Kind kind;
    char *text;
} Str;

typedef struct {
    Kind kind;
    long body;
    long arity;
} Clos;

void mc_main(void);

static void die(const char *message) {
    fprintf(stderr, "sicp-5-52: %s\n", message);
    exit(1);
}

static void *xmalloc(size_t size) {
    void *block = malloc(size == 0 ? 1 : size);
    if (block == NULL) {
        die("out of memory");
    }
    return block;
}

static void *xgrow(void *old, size_t size) {
    void *block = realloc(old, size == 0 ? 1 : size);
    if (block == NULL) {
        die("out of memory");
    }
    return block;
}

static int is_int(long word) { return (word & 1) != 0; }

static Head *as_head(long word, const char *who) {
    Head *head;
    if (word == 0 || is_int(word)) {
        die(who);
    }
    head = (Head *)(void *)word;
    return head;
}

static void *as_object(long word, Kind kind, const char *who) {
    Head *head = as_head(word, who);
    if (head->kind != kind) {
        die(who);
    }
    return head;
}

long mc_alloc_int(long value) { return (long)(((unsigned long)value << 1) | 1UL); }

long mc_int_value(long word) {
    if (!is_int(word)) {
        die("mc_int_value: not an integer");
    }
    return word >> 1;
}

long mc_alloc_pair(long car, long cdr) {
    Pair *cell = xmalloc(sizeof *cell);
    cell->kind = K_PAIR;
    cell->car = car;
    cell->cdr = cdr;
    return (long)cell;
}

long mc_car(long pair) {
    Pair *cell = as_object(pair, K_PAIR, "mc_car: not a pair");
    return cell->car;
}

long mc_cdr(long pair) {
    Pair *cell = as_object(pair, K_PAIR, "mc_cdr: not a pair");
    return cell->cdr;
}

long mc_set_car(long pair, long value) {
    Pair *cell = as_object(pair, K_PAIR, "mc_set_car: not a pair");
    cell->car = value;
    return value;
}

long mc_set_cdr(long pair, long value) {
    Pair *cell = as_object(pair, K_PAIR, "mc_set_cdr: not a pair");
    cell->cdr = value;
    return value;
}

long mc_is_pair(long word) {
    if (word == 0 || is_int(word)) {
        return 0;
    }
    return ((Head *)(void *)word)->kind == K_PAIR ? 1 : 0;
}

long mc_is_null(long word) { return word == 0 ? 1 : 0; }

/* The section's option/result encoding over pairs. */
long mc_compile_error_ctor_opt_some(long payload) {
    return mc_alloc_pair(mc_alloc_int(1), payload);
}

long mc_compile_error_ctor_opt_none(void) {
    return mc_alloc_pair(mc_alloc_int(0), 0);
}

long mc_compile_error_ctor_res_ok(long payload) {
    return mc_alloc_pair(mc_alloc_int(2), payload);
}

long mc_compile_error_ctor_res_err(long payload) {
    return mc_alloc_pair(mc_alloc_int(3), payload);
}

long mc_alloc_vec(long capacity) {
    Vec *vec = xmalloc(sizeof *vec);
    vec->kind = K_VEC;
    vec->length = 0;
    vec->capacity = capacity > 0 ? (size_t)capacity : 0;
    vec->items = NULL;
    if (vec->capacity > 0) {
        vec->items = xmalloc(vec->capacity * sizeof *vec->items);
    }
    return (long)vec;
}

long mc_vec_push(long vec_word, long value) {
    Vec *vec = as_object(vec_word, K_VEC, "mc_vec_push: not a vector");
    if (vec->length == vec->capacity) {
        size_t capacity = vec->capacity == 0 ? 4 : vec->capacity * 2;
        vec->items = xgrow(vec->items, capacity * sizeof *vec->items);
        vec->capacity = capacity;
    }
    vec->items[vec->length++] = value;
    return vec_word;
}

long mc_vec_get(long vec_word, long index) {
    Vec *vec = as_object(vec_word, K_VEC, "mc_vec_get: not a vector");
    if (index < 0 || (size_t)index >= vec->length) {
        die("mc_vec_get: index out of bounds");
    }
    return vec->items[index];
}

long mc_text(const char *text) {
    Str *str = xmalloc(sizeof *str);
    size_t length = strlen(text) + 1;
    str->kind = K_STR;
    str->text = xmalloc(length);
    memcpy(str->text, text, length);
    return (long)str;
}

long mc_make_closure(long body_id, long arity) {
    Clos *clos = xmalloc(sizeof *clos);
    clos->kind = K_CLOS;
    clos->body = body_id;
    clos->arity = arity;
    return (long)clos;
}

/* Unrunnable until the translation names a code table for the body
   id: entering a label address from the store needs emitter help. */
long mc_apply_closure(long closure, long args) {
    (void)args;
    as_object(closure, K_CLOS, "mc_apply_closure: not a closure");
    die("mc_apply_closure: the emitter provides no body-id dispatch table");
    return 0;
}

/* Unrunnable until the translation passes the pattern: the emitted
   call carries only the value register. */
int mc_bind(long value) {
    (void)value;
    die("mc_bind: the emitter provides no pattern to bind against");
    return 0;
}

static void mc_print_list(long word) {
    long cursor = word;
    putchar('(');
    for (;;) {
        Head *head;
        if (cursor == 0) {
            putchar(')');
            return;
        }
        if (is_int(cursor)) {
            fputs(" . ", stdout);
            printf("%ld", mc_int_value(cursor));
            putchar(')');
            return;
        }
        head = (Head *)(void *)cursor;
        if (head->kind != K_PAIR) {
            fputs(" . ", stdout);
            if (head->kind == K_VEC) {
                Vec *vec = (Vec *)head;
                putchar('[');
                for (size_t i = 0; i < vec->length; i++) {
                    if (i > 0) {
                        fputs(", ", stdout);
                    }
                    mc_print_list(vec->items[i]);
                }
                putchar(']');
            } else if (head->kind == K_STR) {
                fputs(((Str *)head)->text, stdout);
            } else {
                printf("#[closure %ld/%ld]", ((Clos *)head)->body, ((Clos *)head)->arity);
            }
            putchar(')');
            return;
        }
        if (cursor != word) {
            putchar(' ');
        }
        mc_print_list(((Pair *)head)->car);
        cursor = ((Pair *)head)->cdr;
    }
}

void mc_print(long word) {
    if (word == 0) {
        fputs("()", stdout);
        return;
    }
    if (is_int(word)) {
        printf("%ld", mc_int_value(word));
        return;
    }
    switch (((Head *)(void *)word)->kind) {
    case K_PAIR:
        mc_print_list(word);
        break;
    case K_VEC: {
        Vec *vec = (Vec *)(void *)word;
        putchar('[');
        for (size_t i = 0; i < vec->length; i++) {
            if (i > 0) {
                fputs(", ", stdout);
            }
            mc_print(vec->items[i]);
        }
        putchar(']');
        break;
    }
    case K_STR:
        fputs(((Str *)(void *)word)->text, stdout);
        break;
    case K_CLOS:
        printf("#[closure %ld/%ld]", ((Clos *)(void *)word)->body,
            ((Clos *)(void *)word)->arity);
        break;
    }
}

int main(void) {
    mc_main();
    return 0;
}
