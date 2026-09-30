/* SPDX-License-Identifier: GPL-3.0-only */
/* Exercise 5.51: the explicit-control evaluator of section 5.4 translated
   into C.  The translation is the controller itself.  The machine's
   registers are file-scope variables, every `ev-` entry point of 5.4.1 to
   5.4.4 is a C label, `(assign continue (label l))` stores a label address,
   `(goto (reg continue))` is `goto *R_continue` (the GNU computed-goto
   extension the system compiler accepts), and `save`/`restore` are push and
   pop on a growable array.  The object world of pairs, symbols, environment
   frames, and the primitive table is the run-time support the exercise
   requires.  Numbers are long longs, as in exercise 5.52's runtime.

   Read a program from a file named on the command line, or from standard
   input, and run the book's read-eval-print loop over it. */

#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum { T_INT, T_REAL, T_BOOL, T_SYM, T_STR, T_NIL, T_PAIR, T_VEC, T_PRIM, T_COMP, T_ENV } Tag;

typedef struct Val Val;
typedef struct Env Env;

struct Val {
    Tag tag;
    union {
        long long integer;
        double real;
        int boolean;
        const char *text;
        struct { Val *car, *cdr; } pair;
        struct { Val **items; size_t length; size_t capacity; } vec;
        struct { Val *parameters; Val *body; Env *environment; } procedure;
        Env *environment;
    } u;
};

struct Env {
    Val *variables;
    Val *values;
    Env *parent;
};

/* One machine stack slot: a saved register value, or a saved continue
   address.  The two are tagged so a mismatched restore is caught. */
typedef struct {
    Val *value;
    void *address;
    int is_address;
} StackSlot;

typedef struct SymbolNode {
    const char *name;
    struct SymbolNode *next;
} SymbolNode;

typedef struct {
    const char *text;
    size_t length;
    size_t position;
} Reader;

/* The evaluator's seven registers, of which `flag` is the machine's own and
   is never named by a controller instruction. */
static Val *R_exp, *R_env, *R_val, *R_proc, *R_argl, *R_unev;
static void *R_continue;

static Val *V_NIL, *V_TRUE, *V_FALSE;
static Env *global_environment;
static SymbolNode *symbols;

static StackSlot *machine_stack;
static size_t stack_capacity, stack_pointer;
static size_t total_pushes, maximum_depth;

static Reader reader;

static void die(const char *message) {
    fprintf(stderr, "eceval: %s\n", message);
    exit(EXIT_FAILURE);
}

static void *checked_malloc(size_t size) {
    void *result = malloc(size == 0 ? 1 : size);
    if (result == NULL) {
        die("out of memory");
    }
    return result;
}

/* Growth keeps the old block alive until the new one exists, so a failed
   realloc cannot lose the stack. */
static void *checked_grow(void *old, size_t size) {
    void *result = realloc(old, size == 0 ? 1 : size);
    if (result == NULL) {
        die("out of memory");
    }
    return result;
}

static Val *new_value(Tag tag) {
    Val *value = checked_malloc(sizeof *value);
    value->tag = tag;
    return value;
}

static const char *intern(const char *name) {
    for (SymbolNode *node = symbols; node != NULL; node = node->next) {
        if (strcmp(node->name, name) == 0) {
            return node->name;
        }
    }
    size_t length = strlen(name) + 1;
    char *copy = checked_malloc(length);
    memcpy(copy, name, length);
    SymbolNode *node = checked_malloc(sizeof *node);
    node->name = copy;
    node->next = symbols;
    symbols = node;
    return copy;
}

static Val *make_integer(long long number) {
    Val *value = new_value(T_INT);
    value->u.integer = number;
    return value;
}

static Val *make_real(double number) {
    Val *value = new_value(T_REAL);
    value->u.real = number;
    return value;
}

static Val *make_boolean(int truth) { return truth ? V_TRUE : V_FALSE; }

static Val *make_symbol(const char *name) {
    Val *value = new_value(T_SYM);
    value->u.text = intern(name);
    return value;
}

static Val *make_string(const char *text) {
    size_t length = strlen(text) + 1;
    char *copy = checked_malloc(length);
    memcpy(copy, text, length);
    Val *value = new_value(T_STR);
    value->u.text = copy;
    return value;
}

static Val *cons(Val *car, Val *cdr) {
    Val *value = new_value(T_PAIR);
    value->u.pair.car = car;
    value->u.pair.cdr = cdr;
    return value;
}

/* ------------------------------------------------------------------ */
/* The section-5.52 object ABI: vectors and tagged option/result      */
/* pairs. `Some` is the pair (1 . payload), `None` is (0 . ()), `Ok` */
/* is (2 . payload), and `Err` is (3 . payload): the tag an integer, */
/* the shape a pair no user list can be confused with once its `car` */
/* is read as the discriminant.                                       */
/* ------------------------------------------------------------------ */



static Val *make_vector(void) {
    Val *value = new_value(T_VEC);
    value->u.vec.items = NULL;
    value->u.vec.length = 0;
    value->u.vec.capacity = 0;
    return value;
}

/* `mc_vec_push`: append `item` to `vec` and answer the vector. The   */
/* store grows geometrically, so a run of pushes costs linear time.  */
static Val *mc_vec_push(Val *vec, Val *item) {
    if (vec == NULL || vec->tag != T_VEC) {
        die("mc_vec_push: not a vector");
    }
    if (vec->u.vec.length == vec->u.vec.capacity) {
        size_t capacity = vec->u.vec.capacity == 0 ? 4 : vec->u.vec.capacity * 2;
        vec->u.vec.items =
            checked_grow(vec->u.vec.items, capacity * sizeof *vec->u.vec.items);
        vec->u.vec.capacity = capacity;
    }
    vec->u.vec.items[vec->u.vec.length++] = item;
    return vec;
}

static Val *opt_some(Val *payload) {
    return cons(make_integer(1), payload);
}

static Val *opt_none(void) { return cons(make_integer(0), V_NIL); }

static Val *res_ok(Val *payload) {
    return cons(make_integer(2), payload);
}

static Val *res_err(Val *payload) {
    return cons(make_integer(3), payload);
}

static Val *make_primitive(const char *name) {
    Val *value = new_value(T_PRIM);
    value->u.text = intern(name);
    return value;
}

static Val *make_procedure(Val *parameters, Val *body, Env *environment) {
    Val *value = new_value(T_COMP);
    value->u.procedure.parameters = parameters;
    value->u.procedure.body = body;
    value->u.procedure.environment = environment;
    return value;
}

static Val *make_environment_word(Env *environment) {
    Val *value = new_value(T_ENV);
    value->u.environment = environment;
    return value;
}

/* The book's `true?`: every value counts as true except `false`. */
static int is_true(Val *value) { return value != V_FALSE; }

/* ------------------------------------------------------------------ */
/* The machine stack                                                    */
/* ------------------------------------------------------------------ */

static void stack_push(StackSlot slot) {
    if (stack_pointer == stack_capacity) {
        if (stack_capacity > SIZE_MAX / (2 * sizeof *machine_stack)) {
            die("stack capacity overflow");
        }
        size_t next = stack_capacity == 0 ? 256 : stack_capacity * 2;
        machine_stack = checked_grow(machine_stack, next * sizeof *machine_stack);
        stack_capacity = next;
    }
    machine_stack[stack_pointer++] = slot;
    total_pushes++;
    if (stack_pointer > maximum_depth) {
        maximum_depth = stack_pointer;
    }
}

static StackSlot stack_pop(void) {
    if (stack_pointer == 0) {
        die("stack underflow");
    }
    return machine_stack[--stack_pointer];
}

static void push_value(Val *value) {
    StackSlot slot;
    slot.value = value;
    slot.address = NULL;
    slot.is_address = 0;
    stack_push(slot);
}

static void push_address(void *address) {
    StackSlot slot;
    slot.value = NULL;
    slot.address = address;
    slot.is_address = 1;
    stack_push(slot);
}

static Val *pop_value(void) {
    StackSlot slot = stack_pop();
    if (slot.is_address) {
        die("stack type mismatch");
    }
    return slot.value;
}

static void *pop_address(void) {
    StackSlot slot = stack_pop();
    if (!slot.is_address) {
        die("stack type mismatch");
    }
    return slot.address;
}

/* ------------------------------------------------------------------ */
/* List and form helpers                                                */
/* ------------------------------------------------------------------ */

/* The length of a proper list; an improper tail is the error the book's
   accessors would signal. */
static size_t list_length(Val *list) {
    size_t length = 0;
    while (list != NULL && list->tag == T_PAIR) {
        length++;
        list = list->u.pair.cdr;
    }
    if (list != V_NIL) {
        die("expected a proper list");
    }
    return length;
}

static int symbol_is(Val *value, const char *name) {
    return value != NULL && value->tag == T_SYM && strcmp(value->u.text, name) == 0;
}

/* The list items of a `(tag ...)` form, or NULL when the word is not one. */
static Val *tagged_form(Val *expression, const char *tag) {
    if (expression == NULL || expression->tag != T_PAIR) {
        return NULL;
    }
    return symbol_is(expression->u.pair.car, tag) ? expression : NULL;
}

/* The item at `index` of a form. */
static Val *form_item(Val *list, size_t index) {
    while (index > 0 && list != NULL && list->tag == T_PAIR) {
        list = list->u.pair.cdr;
        index--;
    }
    if (index != 0 || list == NULL || list->tag != T_PAIR) {
        die("malformed special form");
    }
    return list->u.pair.car;
}

/* The book's `adjoin-arg`: the argument joins the end of the operand list,
   so argl keeps the left-to-right order the parameters are written in. */
static Val *adjoin_arg(Val *argl, Val *argument) {
    if (argl == V_NIL) {
        return cons(argument, V_NIL);
    }
    Val *head = argl;
    while (argl->u.pair.cdr != V_NIL) {
        argl = argl->u.pair.cdr;
    }
    argl->u.pair.cdr = cons(argument, V_NIL);
    return head;
}

/* ------------------------------------------------------------------ */
/* Environments                                                         */
/* ------------------------------------------------------------------ */

static Env *environment_of(Val *word) {
    if (word != NULL && word->tag == T_ENV) {
        return word->u.environment;
    }
    die("environment operation on a non-environment");
    return NULL;
}

static void env_define(Val *variable, Val *value, Env *environment) {
    if (variable == NULL || variable->tag != T_SYM) {
        die("define needs a variable");
    }
    environment->variables = cons(variable, environment->variables);
    environment->values = cons(value, environment->values);
}

/* Walks a frame's parallel variable and value lists for `name`. */
static Val *frame_find(Env *frame, const char *name) {
    Val *names = frame->variables;
    Val *values = frame->values;
    while (names != V_NIL && names->tag == T_PAIR && values != V_NIL && values->tag == T_PAIR) {
        if (symbol_is(names->u.pair.car, name)) {
            return values;
        }
        names = names->u.pair.cdr;
        values = values->u.pair.cdr;
    }
    return NULL;
}

static Val *env_lookup(Val *variable, Env *environment) {
    if (variable == NULL || variable->tag != T_SYM) {
        die("lookup needs a variable");
    }
    for (Env *frame = environment; frame != NULL; frame = frame->parent) {
        Val *cell = frame_find(frame, variable->u.text);
        if (cell != NULL) {
            return cell->u.pair.car;
        }
    }
    fprintf(stderr, "eceval: unbound variable: %s\n", variable->u.text);
    exit(EXIT_FAILURE);
}

static void env_set(Val *variable, Val *value, Env *environment) {
    if (variable == NULL || variable->tag != T_SYM) {
        die("set! needs a variable");
    }
    for (Env *frame = environment; frame != NULL; frame = frame->parent) {
        Val *cell = frame_find(frame, variable->u.text);
        if (cell != NULL) {
            cell->u.pair.car = value;
            return;
        }
    }
    fprintf(stderr, "eceval: unbound variable in set!: %s\n", variable->u.text);
    exit(EXIT_FAILURE);
}

/* `extend-environment`: one frame binding each parameter to its argument. */
static Val *extend_environment(Val *parameters, Val *argl, Env *base) {
    size_t wanted = list_length(parameters);
    size_t given = list_length(argl);
    if (wanted != given) {
        fprintf(stderr, "eceval: the procedure wants %zu arguments, got %zu\n", wanted, given);
        exit(EXIT_FAILURE);
    }
    Env *frame = checked_malloc(sizeof *frame);
    frame->variables = V_NIL;
    frame->values = V_NIL;
    frame->parent = base;
    for (Val *names = parameters, *values = argl; names != V_NIL;
         names = names->u.pair.cdr, values = values->u.pair.cdr) {
        Val *variable = names->u.pair.car;
        if (variable->tag != T_SYM) {
            die("a parameter is written as a variable");
        }
        env_define(variable, values->u.pair.car, frame);
    }
    return make_environment_word(frame);
}

/* ------------------------------------------------------------------ */
/* Printing                                                             */
/* ------------------------------------------------------------------ */

static void print_string(const char *text) {
    putchar('"');
    for (const unsigned char *p = (const unsigned char *)text; *p != '\0'; p++) {
        switch (*p) {
        case '\\': fputs("\\\\", stdout); break;
        case '"': fputs("\\\"", stdout); break;
        case '\n': fputs("\\n", stdout); break;
        case '\r': fputs("\\r", stdout); break;
        case '\t': fputs("\\t", stdout); break;
        default: putchar(*p); break;
        }
    }
    putchar('"');
}

static void print_value(Val *value) {
    switch (value->tag) {
    case T_INT: printf("%lld", value->u.integer); break;
    case T_REAL: printf("%.15g", value->u.real); break;
    case T_BOOL: fputs(value == V_FALSE ? "#f" : "#t", stdout); break;
    case T_SYM: fputs(value->u.text, stdout); break;
    case T_STR: print_string(value->u.text); break;
    case T_VEC: {
        putchar('#');
        putchar('(');
        for (size_t index = 0; index < value->u.vec.length; index++) {
            if (index > 0) {
                putchar(' ');
            }
            print_value(value->u.vec.items[index]);
        }
        putchar(')');
        break;
    }
    case T_NIL: fputs("()", stdout); break;
    case T_PRIM: printf("#[primitive-procedure %s]", value->u.text); break;
    case T_COMP: fputs("#[compound-procedure]", stdout); break;
    case T_ENV: fputs("#[environment]", stdout); break;
    case T_PAIR: {
        putchar('(');
        Val *cursor = value;
        int first = 1;
        while (cursor->tag == T_PAIR) {
            if (!first) {
                putchar(' ');
            }
            first = 0;
            print_value(cursor->u.pair.car);
            cursor = cursor->u.pair.cdr;
        }
        if (cursor != V_NIL) {
            fputs(" . ", stdout);
            print_value(cursor);
        }
        putchar(')');
        break;
    }
    }
}

/* The driver's `user-print`. */
static void user_print(Val *value) {
    print_value(value);
    putchar('\n');
}

/* ------------------------------------------------------------------ */
/* The primitive table                                                  */
/* ------------------------------------------------------------------ */

/* `equal?`: identity on pairs, content everywhere else. */
static int equal_values(Val *left, Val *right) {
    if (left == right) {
        return 1;
    }
    if (left == NULL || right == NULL || left->tag != right->tag) {
        return 0;
    }
    switch (left->tag) {
    case T_INT: return left->u.integer == right->u.integer;
    case T_REAL: return left->u.real == right->u.real;
    case T_BOOL: return left->u.boolean == right->u.boolean;
    case T_SYM: case T_STR: case T_PRIM: return strcmp(left->u.text, right->u.text) == 0;
    case T_NIL: return 1;
    case T_PAIR:
        return equal_values(left->u.pair.car, right->u.pair.car)
            && equal_values(left->u.pair.cdr, right->u.pair.cdr);
    case T_COMP: case T_ENV: return 0;
    case T_VEC: {
        if (left->u.vec.length != right->u.vec.length) {
            return 0;
        }
        for (size_t index = 0; index < left->u.vec.length; index++) {
            if (!equal_values(left->u.vec.items[index], right->u.vec.items[index])) {
                return 0;
            }
        }
        return 1;
    }
    }
    return 0;
}

static Val *arg_at(Val *argl, size_t index) { return form_item(argl, index); }

static void require_arity(const char *name, size_t count, size_t wanted) {
    if (count != wanted) {
        fprintf(stderr, "eceval: %s: needs %zu argument(s), got %zu\n", name, wanted, count);
        exit(EXIT_FAILURE);
    }
}

/* The arithmetic folds need a first argument to seed the accumulator. */
static void require_at_least(const char *name, size_t count, size_t wanted) {
    if (count < wanted) {
        fprintf(stderr, "eceval: %s: needs at least %zu argument(s), got %zu\n", name, wanted,
                count);
        exit(EXIT_FAILURE);
    }
}

static long long integer_of(Val *value, const char *name) {
    if (value->tag != T_INT) {
        fprintf(stderr, "eceval: %s: needs a number\n", name);
        exit(EXIT_FAILURE);
    }
    return value->u.integer;
}

/* The fold the book's `+`, `*`, and `-` run: the first argument seeds the
   accumulator, so an empty argument list is the arithmetic error. */
static Val *fold_arith(const char *name, Val *argl, size_t count) {
    long long accumulator = integer_of(arg_at(argl, 0), name);
    for (size_t index = 1; index < count; index++) {
        long long next = integer_of(arg_at(argl, index), name);
        long long result;
        int overflow = strcmp(name, "+") == 0 ? __builtin_add_overflow(accumulator, next, &result)
            : strcmp(name, "*") == 0 ? __builtin_mul_overflow(accumulator, next, &result)
                                     : __builtin_sub_overflow(accumulator, next, &result);
        if (overflow) {
            die("arithmetic overflow");
        }
        accumulator = result;
    }
    return make_integer(accumulator);
}

static Val *compare(const char *name, Val *left_word, Val *right_word) {
    long long left = integer_of(left_word, name);
    long long right = integer_of(right_word, name);
    int truth = strcmp(name, "=") == 0 ? left == right
        : strcmp(name, "<") == 0 ? left < right : left > right;
    return make_boolean(truth);
}

/* The primitives that take exactly one argument. */
static int is_unary_test(const char *name) {
    return strcmp(name, "null?") == 0 || strcmp(name, "pair?") == 0
        || strcmp(name, "symbol?") == 0 || strcmp(name, "number?") == 0
        || strcmp(name, "string?") == 0 || strcmp(name, "not") == 0;
}

/* The primitives that take exactly two arguments. */
static int is_binary_test(const char *name) {
    return strcmp(name, "eq?") == 0 || strcmp(name, "equal?") == 0
        || strcmp(name, "=") == 0 || strcmp(name, "<") == 0 || strcmp(name, ">") == 0
        || strcmp(name, "remainder") == 0;
}

/* The n-ary arithmetic folds, each of which needs at least one argument. */
static int is_arith_fold(const char *name) {
    return strcmp(name, "+") == 0 || strcmp(name, "-") == 0 || strcmp(name, "*") == 0;
}

static Val *unary_test(const char *name, Val *argument) {
    if (strcmp(name, "null?") == 0) return make_boolean(argument == V_NIL);
    if (strcmp(name, "pair?") == 0) return make_boolean(argument->tag == T_PAIR);
    if (strcmp(name, "symbol?") == 0) return make_boolean(argument->tag == T_SYM);
    if (strcmp(name, "number?") == 0)
        return make_boolean(argument->tag == T_INT || argument->tag == T_REAL);
    if (strcmp(name, "string?") == 0) return make_boolean(argument->tag == T_STR);
    return make_boolean(!is_true(argument));
}

static Val *binary_test(const char *name, Val *first, Val *second) {
    if (strcmp(name, "eq?") == 0) {
        /* `eq?` is identity on pairs and content on the atomic types. */
        if (first->tag == T_PAIR && second->tag == T_PAIR) {
            return make_boolean(first == second);
        }
        return make_boolean(equal_values(first, second));
    }
    if (strcmp(name, "equal?") == 0) return make_boolean(equal_values(first, second));
    if (strcmp(name, "remainder") == 0) {
        long long dividend = integer_of(first, name);
        long long divisor = integer_of(second, name);
        if (divisor == 0) {
            die("division by zero");
        }
        return make_integer(dividend % divisor);
    }
    return compare(name, first, second);
}

/* `apply-primitive-procedure` over the object-language primitives. */
static Val *apply_primitive(const char *name, Val *argl) {
    size_t count = list_length(argl);

    if (strcmp(name, "list") == 0) {
        return argl;
    }
    if (strcmp(name, "cons") == 0) {
        require_arity(name, count, 2);
        return cons(arg_at(argl, 0), arg_at(argl, 1));
    }
    if (strcmp(name, "car") == 0 || strcmp(name, "cdr") == 0) {
        Val *pair;
        require_arity(name, count, 1);
        pair = arg_at(argl, 0);
        if (pair->tag != T_PAIR) {
            fprintf(stderr, "eceval: %s: not a pair\n", name);
            exit(EXIT_FAILURE);
        }
        return strcmp(name, "car") == 0 ? pair->u.pair.car : pair->u.pair.cdr;
    }
    if (is_unary_test(name)) {
        require_arity(name, count, 1);
        return unary_test(name, arg_at(argl, 0));
    }
    if (is_binary_test(name)) {
        require_arity(name, count, 2);
        return binary_test(name, arg_at(argl, 0), arg_at(argl, 1));
    }
    if (strcmp(name, "/") == 0) {
        require_at_least(name, count, 1);
        long long accumulator = integer_of(arg_at(argl, 0), name);
        for (size_t index = 1; index < count; index++) {
            long long divisor = integer_of(arg_at(argl, index), name);
            if (divisor == 0) {
                die("division by zero");
            }
            /* An exact quotient stays integral; the first inexact one ends
               the fold as a real, as the section's divide answers. */
            if (accumulator % divisor != 0) {
                return make_real((double)accumulator / (double)divisor);
            }
            accumulator /= divisor;
        }
        return make_integer(accumulator);
    }
    if (is_arith_fold(name)) {
        require_at_least(name, count, 1);
        if (count > 1) {
            return fold_arith(name, argl, count);
        }
        long long value = integer_of(arg_at(argl, 0), name);
        if (strcmp(name, "-") == 0) {
            if (value == LLONG_MIN) {
                die("arithmetic overflow");
            }
            return make_integer(-value);
        }
        return make_integer(value);
    }
    fprintf(stderr, "eceval: unknown primitive: %s\n", name);
    exit(EXIT_FAILURE);
}

static void initialize_world(void) {
    static const char *const primitive[] = {
        "cons", "car", "cdr", "null?", "pair?", "symbol?", "number?", "string?",
        "not", "eq?", "equal?", "list", "+", "-", "*", "/", "=", "<", ">", "remainder",
    };
    V_NIL = new_value(T_NIL);
    V_TRUE = new_value(T_BOOL);
    V_TRUE->u.boolean = 1;
    V_FALSE = new_value(T_BOOL);
    V_FALSE->u.boolean = 0;
    global_environment = checked_malloc(sizeof *global_environment);
    global_environment->variables = V_NIL;
    global_environment->values = V_NIL;
    global_environment->parent = NULL;
    env_define(make_symbol("true"), V_TRUE, global_environment);
    env_define(make_symbol("false"), V_FALSE, global_environment);
    for (size_t index = 0; index < sizeof primitive / sizeof primitive[0]; index++) {
        env_define(make_symbol(primitive[index]), make_primitive(primitive[index]),
                   global_environment);
    }
}

/* ------------------------------------------------------------------ */
/* The reader                                                           */
/* ------------------------------------------------------------------ */

static int at_end(void) { return reader.position >= reader.length; }

/* ------------------------------------------------------------------ */
/* The guest-slice front end                                            */
/* ------------------------------------------------------------------ */

/* The 5.51 object language is the admitted Rust slice the edition     */
/* teaches: `fn` items, integer arithmetic, comparisons, `if`/`else`, */
/* calls, and `println!` interactions. The parser desugars each item  */
/* to the core Val form the controller already evaluates: `fn`       */
/* becomes `define`, `if` becomes the core `if` form, `!=`/`<=`/`>=` */
/* become `not` over the primitive comparison, and each `println!`   */
/* of `main` becomes one interaction the driver evaluates and       */
/* prints. Only checker-admitted programs are fed here; the harness  */
/* admits every input before running the binary, so anything outside */
/* the slice below dies loudly instead of guessing.                  */

static Val *pending_head;
static Val *pending_tail;
static int program_parsed;

static void emit_form(Val *form) {
    Val *cell = cons(form, V_NIL);
    if (pending_head == NULL) {
        pending_head = cell;
    } else {
        pending_tail->u.pair.cdr = cell;
    }
    pending_tail = cell;
}

static int peek_ch(void) {
    if (at_end()) {
        return EOF;
    }
    return (unsigned char)reader.text[reader.position];
}

/* Blanks, `//` line comments, and slash-star block comments. */
static void skip_ws(void) {
    for (;;) {
        while (!at_end() && isspace((unsigned char)reader.text[reader.position])) {
            reader.position++;
        }
        if (at_end() || reader.text[reader.position] != '/') {
            return;
        }
        if (reader.position + 1 >= reader.length) {
            return;
        }
        char next = reader.text[reader.position + 1];
        if (next == '/') {
            reader.position += 2;
            while (!at_end() && reader.text[reader.position] != '\n') {
                reader.position++;
            }
            continue;
        }
        if (next != '*') {
            return;
        }
        reader.position += 2;
        for (;;) {
            if (at_end()) {
                die("unterminated block comment");
            }
            if (reader.text[reader.position] == '*'
                && reader.position + 1 < reader.length
                && reader.text[reader.position + 1] == '/') {
                reader.position += 2;
                break;
            }
            reader.position++;
        }
    }
}

static int is_ident_start(int c) { return isalpha(c) || c == '_'; }
static int is_ident_char(int c) { return isalnum(c) || c == '_'; }

/* Copies the identifier at the cursor; dies when none is there. */
static void take_ident(char *out, size_t cap) {
    size_t len = 0;
    skip_ws();
    if (!is_ident_start(peek_ch())) {
        die("expected a name");
    }
    while (is_ident_char(peek_ch())) {
        if (len + 1 >= cap) {
            die("name too long");
        }
        out[len++] = (char)peek_ch();
        reader.position++;
    }
    out[len] = '\0';
}

/* Eats `word` when it starts at the cursor with a word boundary. */
static int take_keyword(const char *word) {
    size_t save = reader.position;
    size_t i = 0;
    skip_ws();
    save = reader.position;
    while (word[i] != '\0') {
        if (at_end() || reader.text[reader.position] != word[i]) {
            reader.position = save;
            return 0;
        }
        reader.position++;
        i++;
    }
    if (is_ident_char(peek_ch())) {
        reader.position = save;
        return 0;
    }
    return 1;
}

static void expect_keyword(const char *word) {
    if (!take_keyword(word)) {
        fprintf(stderr, "eceval: expected `%s`\n", word);
        exit(EXIT_FAILURE);
    }
}

static int take_punct(char c) {
    skip_ws();
    if (peek_ch() != c) {
        return 0;
    }
    reader.position++;
    return 1;
}

static void expect_punct(char c) {
    if (!take_punct(c)) {
        fprintf(stderr, "eceval: expected `%c`\n", c);
        exit(EXIT_FAILURE);
    }
}

static Val *parse_expr(void);

/* One integer literal with `_` separators and an optional type suffix. */
static Val *parse_integer(void) {
    char buf[64];
    size_t len = 0;
    long long value;
    skip_ws();
    if (!isdigit(peek_ch())) {
        die("expected an integer");
    }
    while (isdigit(peek_ch()) || peek_ch() == '_') {
        int c = peek_ch();
        reader.position++;
        if (c == '_') {
            continue;
        }
        if (len + 1 >= sizeof buf) {
            die("integer too long");
        }
        buf[len++] = (char)c;
    }
    while (is_ident_start(peek_ch())) {
        reader.position++;
    }
    buf[len] = '\0';
    errno = 0;
    value = strtoll(buf, NULL, 10);
    if (errno == ERANGE) {
        die("integer out of range");
    }
    return make_integer(value);
}

/* Skips one string literal with ordinary Rust escapes. */
static void skip_string(void) {
    skip_ws();
    if (peek_ch() != '"') {
        die("expected a string");
    }
    reader.position++;
    for (;;) {
        int c;
        if (at_end()) {
            die("unterminated string");
        }
        c = (unsigned char)reader.text[reader.position++];
        if (c == '"') {
            return;
        }
        if (c == '\\' && !at_end()) {
            reader.position++;
        }
    }
}

/* Skips one type spelling; balanced brackets decide where it ends,   */
/* so `Box<dyn FnMut() -> i64 + 'static>` stops at its comma.        */
static void skip_type(void) {
    int paren = 0;
    int angle = 0;
    int brack = 0;
    int seen = 0;
    for (;;) {
        int c = peek_ch();
        if (c == EOF) {
            die("unterminated type");
        }
        if ((c == ',' || c == ')' || c == '{') && paren == 0 && angle == 0 && brack == 0) {
            if (!seen) {
                die("expected a type");
            }
            return;
        }
        if (c == '"') {
            skip_string();
            seen = 1;
            continue;
        }
        if (c == '(') {
            paren++;
        } else if (c == ')') {
            paren--;
        } else if (c == '<') {
            angle++;
        } else if (c == '>') {
            angle--;
        } else if (c == '[') {
            brack++;
        } else if (c == ']') {
            brack--;
        }
        if (paren < 0 || angle < 0 || brack < 0) {
            die("unbalanced type");
        }
        if (!isspace(c)) {
            seen = 1;
        }
        reader.position++;
    }
}

static Val *apply2(const char *op, Val *left, Val *right) {
    return cons(make_symbol(op), cons(left, cons(right, V_NIL)));
}

static Val *parse_primary(void) {
    skip_ws();
    if (peek_ch() == '(') {
        Val *inner;
        reader.position++;
        inner = parse_expr();
        expect_punct(')');
        return inner;
    }
    if (isdigit(peek_ch())) {
        return parse_integer();
    }
    if (is_ident_start(peek_ch())) {
        char name[64];
        take_ident(name, sizeof name);
        return make_symbol(name);
    }
    die("expected an expression");
    return NULL;
}

static Val *parse_postfix(void) {
    Val *head = parse_primary();
    for (;;) {
        Val *args;
        Val *last;
        skip_ws();
        if (peek_ch() != '(') {
            return head;
        }
        reader.position++;
        args = V_NIL;
        last = NULL;
        skip_ws();
        if (peek_ch() != ')') {
            for (;;) {
                Val *cell = cons(parse_expr(), V_NIL);
                if (last == NULL) {
                    args = cell;
                } else {
                    last->u.pair.cdr = cell;
                }
                last = cell;
                skip_ws();
                if (!take_punct(',')) {
                    break;
                }
            }
        }
        expect_punct(')');
        head = cons(head, args);
    }
}

static Val *parse_unary(void) {
    skip_ws();
    if (take_punct('-')) {
        Val *operand = parse_unary();
        return cons(make_symbol("-"), cons(operand, V_NIL));
    }
    if (take_punct('!')) {
        Val *operand = parse_unary();
        return cons(make_symbol("not"), cons(operand, V_NIL));
    }
    return parse_postfix();
}

static Val *parse_mul(void) {
    Val *left = parse_unary();
    for (;;) {
        skip_ws();
        if (take_punct('*')) {
            left = apply2("*", left, parse_unary());
        } else if (take_punct('/')) {
            left = apply2("/", left, parse_unary());
        } else if (take_punct('%')) {
            left = apply2("remainder", left, parse_unary());
        } else {
            return left;
        }
    }
}

static Val *parse_add(void) {
    Val *left = parse_mul();
    for (;;) {
        skip_ws();
        if (take_punct('+')) {
            left = apply2("+", left, parse_mul());
        } else if (take_punct('-')) {
            left = apply2("-", left, parse_mul());
        } else {
            return left;
        }
    }
}

static Val *boolean_not(Val *value) {
    return cons(make_symbol("not"), cons(value, V_NIL));
}

static Val *parse_cmp(void) {
    Val *left = parse_add();
    skip_ws();
    if (take_punct('=')) {
        expect_punct('=');
        return apply2("=", left, parse_add());
    }
    if (take_punct('!')) {
        expect_punct('=');
        return boolean_not(apply2("=", left, parse_add()));
    }
    if (take_punct('<')) {
        if (take_punct('=')) {
            return boolean_not(apply2(">", left, parse_add()));
        }
        return apply2("<", left, parse_add());
    }
    if (take_punct('>')) {
        if (take_punct('=')) {
            return boolean_not(apply2("<", left, parse_add()));
        }
        return apply2(">", left, parse_add());
    }
    return left;
}

static Val *parse_and(void) {
    Val *left = parse_cmp();
    for (;;) {
        Val *right;
        skip_ws();
        if (peek_ch() != '&') {
            return left;
        }
        reader.position++;
        expect_punct('&');
        right = parse_cmp();
        left = cons(make_symbol("if"),
            cons(left, cons(right, cons(make_symbol("false"), V_NIL))));
    }
}

static Val *parse_or(void) {
    Val *left = parse_and();
    for (;;) {
        Val *right;
        skip_ws();
        if (peek_ch() != '|') {
            return left;
        }
        reader.position++;
        expect_punct('|');
        right = parse_and();
        left = cons(make_symbol("if"),
            cons(left, cons(make_symbol("true"), cons(right, V_NIL))));
    }
}

/* `if test { yes } else { no }` becomes the core four-element form.  */
/* Bare blocks hold one tail expression; anything else dies loudly.   */
static Val *parse_block_expr(void) {
    Val *value;
    expect_punct('{');
    value = parse_expr();
    skip_ws();
    if (peek_ch() == ';') {
        die("statement sequences are outside the 5.51 object slice");
    }
    expect_punct('}');
    return value;
}

static Val *parse_expr(void) {
    skip_ws();
    if (take_keyword("if")) {
        Val *test = parse_or();
        Val *yes = parse_block_expr();
        Val *no;
        expect_keyword("else");
        no = parse_block_expr();
        return cons(make_symbol("if"), cons(test, cons(yes, cons(no, V_NIL))));
    }
    if (take_keyword("let") || take_keyword("return") || take_keyword("while")
        || take_keyword("for") || take_keyword("loop") || take_keyword("match")) {
        die("that statement form is outside the 5.51 object slice");
    }
    return parse_or();
}

/* One `println!("{}", value);` interaction of `main`. */
static void parse_println(void) {
    Val *interaction;
    expect_keyword("println");
    expect_punct('!');
    expect_punct('(');
    skip_string();
    expect_punct(',');
    interaction = parse_expr();
    expect_punct(')');
    expect_punct(';');
    emit_form(interaction);
}

/* One `fn` item: a definition, or `main`'s run of interactions. */
static void parse_fn(void) {
    char name[64];
    Val *params = V_NIL;
    Val *last = NULL;
    take_ident(name, sizeof name);
    expect_punct('(');
    skip_ws();
    if (!take_punct(')')) {
        for (;;) {
            char pname[64];
            Val *cell;
            take_keyword("mut");
            take_ident(pname, sizeof pname);
            expect_punct(':');
            skip_type();
            cell = cons(make_symbol(pname), V_NIL);
            if (last == NULL) {
                params = cell;
            } else {
                last->u.pair.cdr = cell;
            }
            last = cell;
            skip_ws();
            if (take_punct(',')) {
                continue;
            }
            expect_punct(')');
            break;
        }
    }
    skip_ws();
    if (take_punct('-')) {
        expect_punct('>');
        skip_type();
    }
    if (strcmp(name, "main") == 0) {
        expect_punct('{');
        for (;;) {
            skip_ws();
            if (take_punct('}')) {
                return;
            }
            parse_println();
        }
    }
    {
        Val *body = parse_block_expr();
        Val *head = cons(make_symbol(name), params);
        emit_form(cons(make_symbol("define"), cons(head, cons(body, V_NIL))));
    }
}

static void parse_program(void) {
    for (;;) {
        skip_ws();
        if (at_end()) {
            return;
        }
        expect_keyword("fn");
        parse_fn();
    }
}

/* The next interaction, or NULL at the end of the input. */
static Val *read_expression(void) {
    Val *form;
    if (!program_parsed) {
        program_parsed = 1;
        parse_program();
    }
    if (pending_head == NULL) {
        return NULL;
    }
    form = pending_head->u.pair.car;
    pending_head = pending_head->u.pair.cdr;
    return form;
}

static char *read_all(FILE *stream, size_t *length) {
    size_t capacity = 4096, used = 0;
    char *buffer = checked_malloc(capacity);
    for (;;) {
        if (used == capacity) {
            if (capacity > SIZE_MAX / 2) {
                die("input too large");
            }
            capacity *= 2;
            buffer = checked_grow(buffer, capacity);
        }
        size_t amount = fread(buffer + used, 1, capacity - used, stream);
        used += amount;
        if (amount == 0) {
            if (ferror(stream)) {
                die("failed to read input");
            }
            break;
        }
    }
    buffer = checked_grow(buffer, used + 1);
    buffer[used] = '\0';
    *length = used;
    return buffer;
}

/* ------------------------------------------------------------------ */
/* The syntax predicates the controller tests                           */
/* ------------------------------------------------------------------ */

/* The book's `self-evaluating?`: numbers, strings, and booleans. */
static int self_evaluating(Val *expression) {
    return expression->tag == T_INT || expression->tag == T_REAL || expression->tag == T_STR
        || expression->tag == T_BOOL;
}

/* A `(tag ...)` form of exactly `wanted` items. */
static int form_of_length(Val *expression, const char *tag, size_t wanted) {
    if (tagged_form(expression, tag) == NULL) {
        return 0;
    }
    return list_length(expression) == wanted;
}

/* A `(tag ...)` form of at least `wanted` items. */
static int form_of_at_least(Val *expression, const char *tag, size_t wanted) {
    if (tagged_form(expression, tag) == NULL) {
        return 0;
    }
    return list_length(expression) >= wanted;
}

/* `definition?` additionally requires a name or a procedure form. */
static int is_definition(Val *expression) {
    if (!form_of_at_least(expression, "define", 3)) {
        return 0;
    }
    Val *target = form_item(expression, 1);
    return target->tag == T_SYM || target->tag == T_PAIR;
}

/* `lambda-body`, `rest-operands`, and `rest-exps`: the form without its head. */
static Val *without_head(Val *list) {
    if (list == NULL || list->tag != T_PAIR) {
        die("empty sequence");
    }
    return list->u.pair.cdr;
}

/* `first-exp`, `first-operand`, and `operator`: the head of a sequence. */
static Val *first_of(Val *list) {
    if (list == NULL || list->tag != T_PAIR) {
        die("empty sequence");
    }
    return list->u.pair.car;
}

/* `definition-value`: a value definition names its value directly; a
   procedure definition becomes the `lambda` it abbreviates. */
static Val *definition_value(Val *expression) {
    Val *target = form_item(expression, 1);
    if (target->tag != T_PAIR) {
        return form_item(expression, 2);
    }
    Val *parameters = without_head(target);
    Val *body = without_head(without_head(expression));
    return cons(make_symbol("lambda"), cons(parameters, body));
}

/* `if-alternative`: an `if` without one has a false branch. */
static Val *if_alternative(Val *expression) {
    return list_length(expression) == 4 ? form_item(expression, 3) : make_symbol("false");
}

/* ------------------------------------------------------------------ */
/* The controller                                                       */
/* ------------------------------------------------------------------ */

static void eceval(void) {
    /* The driver loop of 5.4.5. */
read_eval_print_loop:
    stack_pointer = 0;
    fputs(";;; EC-Eval input:\n", stdout);
    R_exp = read_expression();
    if (R_exp == NULL) {
        /* The stack is monitored as 5.4.4 does.  The base driver of 5.4.5 has
           no statistics instruction, so the session's totals are reported on
           their own comment line at the end rather than between values. */
        printf(";;; EC-Eval stack: (total-pushes = %zu maximum-depth = %zu)\n", total_pushes,
               maximum_depth);
        return;
    }
    R_env = make_environment_word(global_environment);
    R_continue = &&print_result;
    goto eval_dispatch;

print_result:
    fputs(";;; EC-Eval value:\n", stdout);
    user_print(R_val);
    goto read_eval_print_loop;

    /* eval-dispatch: the book's order of tests, first match wins. */
eval_dispatch:
    if (self_evaluating(R_exp)) goto ev_self_eval;
    if (R_exp->tag == T_SYM) goto ev_variable;
    if (form_of_length(R_exp, "quote", 2)) goto ev_quoted;
    if (form_of_length(R_exp, "set!", 3)) goto ev_assignment;
    if (is_definition(R_exp)) goto ev_definition;
    if (form_of_length(R_exp, "if", 3) || form_of_length(R_exp, "if", 4)) goto ev_if;
    if (form_of_at_least(R_exp, "lambda", 3)) goto ev_lambda;
    if (form_of_at_least(R_exp, "begin", 2)) goto ev_begin;
    if (R_exp->tag == T_PAIR) goto ev_application;
    goto unknown_expression_type;

ev_self_eval:
    R_val = R_exp;
    goto *R_continue;

ev_variable:
    R_val = env_lookup(R_exp, environment_of(R_env));
    goto *R_continue;

ev_quoted:
    R_val = form_item(R_exp, 1);
    goto *R_continue;

ev_lambda:
    R_unev = form_item(R_exp, 1);
    R_exp = without_head(without_head(R_exp));
    R_val = make_procedure(R_unev, R_exp, environment_of(R_env));
    goto *R_continue;

ev_application:
    push_address(R_continue);
    push_value(R_env);
    R_unev = without_head(R_exp);
    push_value(R_unev);
    R_exp = first_of(R_exp);
    R_continue = &&ev_appl_did_operator;
    goto eval_dispatch;

ev_appl_did_operator:
    R_unev = pop_value();
    R_env = pop_value();
    R_argl = V_NIL;
    R_proc = R_val;
    if (R_unev == V_NIL) goto apply_dispatch;
    push_value(R_proc);

ev_appl_operand_loop:
    push_value(R_argl);
    R_exp = first_of(R_unev);
    if (without_head(R_unev) == V_NIL) goto ev_appl_last_arg;
    push_value(R_env);
    push_value(R_unev);
    R_continue = &&ev_appl_accumulate_arg;
    goto eval_dispatch;

ev_appl_accumulate_arg:
    R_unev = pop_value();
    R_env = pop_value();
    R_argl = pop_value();
    R_argl = adjoin_arg(R_argl, R_val);
    R_unev = without_head(R_unev);
    goto ev_appl_operand_loop;

ev_appl_last_arg:
    R_continue = &&ev_appl_accum_last_arg;
    goto eval_dispatch;

ev_appl_accum_last_arg:
    R_argl = pop_value();
    R_argl = adjoin_arg(R_argl, R_val);
    R_proc = pop_value();
    goto apply_dispatch;

apply_dispatch:
    if (R_proc->tag == T_PRIM) goto primitive_apply;
    if (R_proc->tag == T_COMP) goto compound_apply;
    goto unknown_procedure_type;

primitive_apply:
    R_val = apply_primitive(R_proc->u.text, R_argl);
    R_continue = pop_address();
    goto *R_continue;

compound_apply:
    R_unev = R_proc->u.procedure.parameters;
    R_env = extend_environment(R_unev, R_argl, R_proc->u.procedure.environment);
    R_unev = R_proc->u.procedure.body;
    goto ev_sequence;

ev_begin:
    R_unev = without_head(R_exp);
    push_address(R_continue);
    goto ev_sequence;

ev_sequence:
    R_exp = first_of(R_unev);
    if (without_head(R_unev) == V_NIL) goto ev_sequence_last_exp;
    push_value(R_unev);
    push_value(R_env);
    R_continue = &&ev_sequence_continue;
    goto eval_dispatch;

ev_sequence_continue:
    R_env = pop_value();
    R_unev = pop_value();
    R_unev = without_head(R_unev);
    goto ev_sequence;

ev_sequence_last_exp:
    R_continue = pop_address();
    goto eval_dispatch;

ev_if:
    push_value(R_exp);
    push_value(R_env);
    push_address(R_continue);
    R_continue = &&ev_if_decide;
    R_exp = form_item(R_exp, 1);
    goto eval_dispatch;

ev_if_decide:
    R_continue = pop_address();
    R_env = pop_value();
    R_exp = pop_value();
    if (is_true(R_val)) {
        goto ev_if_consequent;
    }
    /* The controller branches to the consequent and falls through here; the
       explicit goto names the label the book's `ev-if-alternative` entry is. */
    goto ev_if_alternative;

ev_if_alternative:
    R_exp = if_alternative(R_exp);
    goto eval_dispatch;

ev_if_consequent:
    R_exp = form_item(R_exp, 2);
    goto eval_dispatch;

ev_assignment:
    R_unev = form_item(R_exp, 1);
    push_value(R_unev);
    R_exp = form_item(R_exp, 2);
    push_value(R_env);
    push_address(R_continue);
    R_continue = &&ev_assignment_1;
    goto eval_dispatch;

ev_assignment_1:
    R_continue = pop_address();
    R_env = pop_value();
    R_unev = pop_value();
    env_set(R_unev, R_val, environment_of(R_env));
    R_val = make_symbol("ok");
    goto *R_continue;

ev_definition:
    R_unev = form_item(R_exp, 1);
    if (R_unev->tag == T_PAIR) {
        R_unev = first_of(R_unev);
    }
    push_value(R_unev);
    R_exp = definition_value(R_exp);
    push_value(R_env);
    push_address(R_continue);
    R_continue = &&ev_definition_1;
    goto eval_dispatch;

ev_definition_1:
    R_continue = pop_address();
    R_env = pop_value();
    R_unev = pop_value();
    env_define(R_unev, R_val, environment_of(R_env));
    R_val = make_symbol("ok");
    goto *R_continue;

unknown_expression_type:
    R_val = make_symbol("unknown-expression-type-error");
    goto signal_error;

unknown_procedure_type:
    R_continue = pop_address();
    R_val = make_symbol("unknown-procedure-type-error");

signal_error:
    user_print(R_val);
    goto read_eval_print_loop;
}

int main(int argc, char **argv) {
    FILE *input = NULL;
    char *text = NULL;
    size_t length = 0;
    int status = EXIT_FAILURE;

    if (argc > 2) {
        fprintf(stderr, "usage: %s [program-file]\n", argv[0]);
        return EXIT_FAILURE;
    }
    if (argc == 2) {
        input = fopen(argv[1], "rb");
        if (input == NULL) {
            perror(argv[1]);
            goto cleanup;
        }
    } else {
        input = stdin;
    }
    text = read_all(input, &length);
    reader.text = text;
    reader.length = length;
    reader.position = 0;
    fclose(input);
    input = NULL;
    initialize_world();
    eceval();
    status = EXIT_SUCCESS;

cleanup:
    if (input != NULL && input != stdin) {
        fclose(input);
    }
    free(text);
    return status;
}
