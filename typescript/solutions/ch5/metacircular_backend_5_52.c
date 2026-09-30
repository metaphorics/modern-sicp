/* SPDX-License-Identifier: GPL-3.0-only */
/* Exercise 5.52 run-time support: the object world the compiled
   metacircular evaluator runs in.

   The compiler's C backend emits the typed instruction stream of
   packages/ch5/src/05-compilation.ts as C statements over these
   operations; the emitted forms are appended after this file and the
   whole translation unit builds to the guest interpreter. Registers are
   globals, `continue` is a saved C label address (the computed-goto
   extension), and save/restore are push and pop on the tagged machine
   stack.

   Object world (consumer contract section 6): numbers are IEEE-754
   doubles (never long long), strings are const char *, booleans are int,
   records/arrays/maps/sets/lists are runtime structs. Machine words may
   additionally carry symbols and label addresses because the exchange
   union has those variants; the guest object model itself has no legacy
   pairs and prints no symbol/nil notation. */

#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
    T_NUM,
    T_STR,
    T_BOOL,
    T_NULL,
    T_UNDEF,
    T_SYM,
    T_LIST,
    T_RECORD,
    T_ARRAY,
    T_MAP,
    T_SET,
    T_CLOSURE,
    T_COMPILED,
    T_PRIM,
    T_ENV,
    T_ADDR,
    T_ERROR,
    T_MACHINE_ERROR,
    T_TRANSFER
} Tag;

typedef struct Val Val;
typedef struct Env Env;

struct Val {
    Tag tag;
    union {
        double num;
        bool boolean;
        const char *text;
        void *address;
        struct {
            Val **items;
            size_t len;
        } seq;
        struct {
            const char **names;
            Val **vals;
            size_t len;
        } rec;
        struct {
            Val **keys;
            Val **vals;
            size_t len;
        } mapv;
        struct {
            Val *params;
            Val *body;
            Env *env;
        } proc;
        struct {
            void *entry;
            Val *params;
            Env *env;
        } compiled;
        struct {
            bool is_error;
            Val *value;
        } transfer;
        Env *environment;
    } u;
};

struct Env {
    Val *names;
    Val *vals;
    Env *parent;
};

static void die(const char *message) {
    fprintf(stderr, "metacircular-backend: %s\n", message);
    exit(EXIT_FAILURE);
}

static void *checked_malloc(size_t size) {
    void *result = malloc(size == 0 ? 1 : size);
    if (result == NULL) {
        die("out of memory");
    }
    return result;
}

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

static Val *V_NULL, *V_UNDEF, *V_TRUE, *V_FALSE;
static Env *global_environment;

static const char *intern(const char *text) {
    size_t length = strlen(text) + 1;
    char *copy = checked_malloc(length);
    memcpy(copy, text, length);
    return copy;
}

static Val *make_num(double n) {
    Val *value = new_value(T_NUM);
    value->u.num = n;
    return value;
}

static Val *make_bool(bool b) { return b ? V_TRUE : V_FALSE; }

static Val *make_str(const char *text) {
    Val *value = new_value(T_STR);
    value->u.text = intern(text);
    return value;
}

static Val *make_sym(const char *name) {
    Val *value = new_value(T_SYM);
    value->u.text = intern(name);
    return value;
}

static Val *make_seq(Tag tag, Val **items, size_t len) {
    Val *value = new_value(tag);
    value->u.seq.items = items;
    value->u.seq.len = len;
    return value;
}

static Val *cons(Val *item, Val *tail) {
    size_t len = tail->u.seq.len;
    Val **items = checked_malloc((len + 1) * sizeof *items);
    items[0] = item;
    for (size_t i = 0; i < len; i += 1) {
        items[i + 1] = tail->u.seq.items[i];
    }
    return make_seq(T_LIST, items, len + 1);
}

static Val *make_record(const char **names, Val **vals, size_t len) {
    Val *value = new_value(T_RECORD);
    value->u.rec.names = names;
    value->u.rec.vals = vals;
    value->u.rec.len = len;
    return value;
}

static Val *make_map(Val **keys, Val **vals, size_t len) {
    Val *value = new_value(T_MAP);
    value->u.mapv.keys = keys;
    value->u.mapv.vals = vals;
    value->u.mapv.len = len;
    return value;
}

static Val *env_word(Env *env) {
    Val *value = new_value(T_ENV);
    value->u.environment = env;
    return value;
}

static Env *env_of(Val *value) {
    if (value->tag != T_ENV) {
        die("expected an environment word");
    }
    return value->u.environment;
}

static Val *addr_word(void *address) {
    Val *value = new_value(T_ADDR);
    value->u.address = address;
    return value;
}

static void *addr_of(Val *value) {
    if (value->tag != T_ADDR) {
        die("expected a label address word");
    }
    return value->u.address;
}

static Val *record_get(Val *record, const char *name) {
    if (record->tag == T_ERROR && strcmp(name, "message") == 0) {
        return make_str(record->u.text);
    }
    if (record->tag != T_RECORD) {
        return V_UNDEF;
    }
    for (size_t i = 0; i < record->u.rec.len; i += 1) {
        if (strcmp(record->u.rec.names[i], name) == 0) {
            return record->u.rec.vals[i];
        }
    }
    return V_UNDEF;
}

static bool tag_is(Val *node, const char *tag) {
    Val *value = node->tag == T_RECORD ? record_get(node, "tag") : V_UNDEF;
    return (value->tag == T_SYM || value->tag == T_STR) && strcmp(value->u.text, tag) == 0;
}

static bool is_false_value(Val *value) {
    if (value->tag == T_BOOL) return !value->u.boolean;
    if (value->tag == T_NUM) return value->u.num == 0 || isnan(value->u.num);
    if (value->tag == T_STR) return value->u.text[0] == '\0';
    return value->tag == T_NULL || value->tag == T_UNDEF;
}

/* ------------------------------------------------------------------ */
/* The machine: registers, tagged stack, and the emitted forms          */
/* ------------------------------------------------------------------ */

static Val *R_exp, *R_env, *R_val, *R_proc, *R_argl, *R_unev, *R_arg1, *R_arg2;
static Val *R_continue;
static bool flag;

typedef struct {
    Val *value;
} StackSlot;

static StackSlot *machine_stack;
static size_t stack_capacity, stack_pointer;

static void initializeStack(void) {
    stack_pointer = 0;
}

static void push_val(Val *value) {
    if (stack_pointer == stack_capacity) {
        size_t next = stack_capacity == 0 ? 256 : stack_capacity * 2;
        machine_stack = checked_grow(machine_stack, next * sizeof *machine_stack);
        stack_capacity = next;
    }
    machine_stack[stack_pointer++] = (StackSlot){ .value = value };
}

static Val *pop_val(void) {
    if (stack_pointer == 0) {
        die("stack underflow");
    }
    return machine_stack[--stack_pointer].value;
}

/* ------------------------------------------------------------------ */
/* Environments                                                         */
/* ------------------------------------------------------------------ */

static Val *copy_sequence(Val *source) {
    if (source->tag != T_ARRAY && source->tag != T_LIST) die("environment frames need name and value sequences");
    size_t len = source->u.seq.len;
    Val **items = checked_malloc((len == 0 ? 1 : len) * sizeof *items);
    for (size_t i = 0; i < len; i += 1) items[i] = source->u.seq.items[i];
    return make_seq(source->tag, items, len);
}

static Env *make_env(Val *names, Val *vals, Env *parent) {
    Env *env = checked_malloc(sizeof *env);
    env->names = copy_sequence(names);
    env->vals = copy_sequence(vals);
    env->parent = parent;
    return env;
}

static Val *lookup_binding(Env *env, const char *name) {
    for (Env *frame = env; frame != NULL; frame = frame->parent) {
        for (size_t i = 0; i < frame->names->u.seq.len; i += 1) {
            if (strcmp(frame->names->u.seq.items[i]->u.text, name) == 0) {
                return frame->vals->u.seq.items[i];
            }
        }
    }
    return NULL;
}

static void print_value(Val *value);
static const char *name_of(Val *value) {
    if (value->tag != T_SYM && value->tag != T_STR) {
        die("expected a name word");
    }
    return value->u.text;
}

static Val *make_error_value(const char *message) {
    Val *value = new_value(T_ERROR);
    value->u.text = intern(message);
    return value;
}

static bool same_value(Val *left, Val *right) {
    if (left == right) return true;
    if (left->tag != right->tag) return false;
    if (left->tag == T_NUM) return left->u.num == right->u.num;
    if (left->tag == T_STR || left->tag == T_SYM) return strcmp(left->u.text, right->u.text) == 0;
    if (left->tag == T_BOOL) return left->u.boolean == right->u.boolean;
    return false;
}

static Val *map_get(Val *map, Val *key) {
    if (map->tag != T_MAP) die("Map.get called on a non-map");
    for (size_t i = 0; i < map->u.mapv.len; i += 1) {
        if (same_value(map->u.mapv.keys[i], key)) return map->u.mapv.vals[i];
    }
    return V_UNDEF;
}

static Val *map_set(Val *map, Val *key, Val *value) {
    if (map->tag != T_MAP) die("Map.set called on a non-map");
    for (size_t i = 0; i < map->u.mapv.len; i += 1) {
        if (same_value(map->u.mapv.keys[i], key)) {
            map->u.mapv.vals[i] = value;
            return map;
        }
    }
    size_t len = map->u.mapv.len;
    map->u.mapv.keys = checked_grow(map->u.mapv.keys, (len + 1) * sizeof *map->u.mapv.keys);
    map->u.mapv.vals = checked_grow(map->u.mapv.vals, (len + 1) * sizeof *map->u.mapv.vals);
    map->u.mapv.keys[len] = key;
    map->u.mapv.vals[len] = value;
    map->u.mapv.len = len + 1;
    return map;
}

/* ------------------------------------------------------------------ */
/* Primitive procedures: the guest builtins behind applyProcedure       */
/* ------------------------------------------------------------------ */

static Val *apply_primitive(const char *name, Val *argl) {
    size_t argc = argl->u.seq.len;
    Val **a = argl->u.seq.items;
    double x = argc > 0 && a[0]->tag == T_NUM ? a[0]->u.num : 0;
    double y = argc > 1 && a[1]->tag == T_NUM ? a[1]->u.num : 0;
    if (strcmp(name, "+") == 0 || strcmp(name, "number-add") == 0) {
        double sum = 0;
        for (size_t i = 0; i < argc; i += 1) sum += a[i]->u.num;
        return make_num(sum);
    }
    if (strcmp(name, "-") == 0 || strcmp(name, "number-subtract") == 0) {
        double rest = argc > 1 ? x : 0;
        for (size_t i = 1; i < argc; i += 1) rest -= a[i]->u.num;
        return make_num(argc > 1 ? rest : -x);
    }
    if (strcmp(name, "*") == 0 || strcmp(name, "number-multiply") == 0) {
        double product = 1;
        for (size_t i = 0; i < argc; i += 1) product *= a[i]->u.num;
        return make_num(product);
    }
    if (strcmp(name, "/") == 0 || strcmp(name, "number-divide") == 0) return make_num(x / y);
    if (strcmp(name, "%") == 0 || strcmp(name, "number-remainder") == 0) return make_num(fmod(x, y));
    if (strcmp(name, "<") == 0 || strcmp(name, "number-less") == 0) return make_bool(x < y);
    if (strcmp(name, "<=") == 0 || strcmp(name, "number-less-equal") == 0) return make_bool(x <= y);
    if (strcmp(name, ">") == 0 || strcmp(name, "number-greater") == 0) return make_bool(x > y);
    if (strcmp(name, ">=") == 0 || strcmp(name, "number-greater-equal") == 0) return make_bool(x >= y);
    if (strcmp(name, "=") == 0 || strcmp(name, "===") == 0 || strcmp(name, "value-equal") == 0) {
        return make_bool(argc > 1 && same_value(a[0], a[1]));
    }
    if (strcmp(name, "!==") == 0 || strcmp(name, "value-not-equal") == 0) {
        return make_bool(argc > 1 && !same_value(a[0], a[1]));
    }
    if (strcmp(name, "!") == 0 || strcmp(name, "boolean-not") == 0) {
        return make_bool(argc > 0 && is_false_value(a[0]));
    }
    if (strcmp(name, "string-concat") == 0) {
        size_t length = strlen(a[0]->u.text) + strlen(a[1]->u.text) + 1;
        char *joined = checked_malloc(length);
        snprintf(joined, length, "%s%s", a[0]->u.text, a[1]->u.text);
        return make_str(joined);
    }
    if (strcmp(name, "map.get") == 0) return argc > 1 ? map_get(a[0], a[1]) : V_UNDEF;
    if (strcmp(name, "map.set") == 0) return argc > 2 ? map_set(a[0], a[1], a[2]) : V_UNDEF;
    if (strcmp(name, "math.abs") == 0) return make_num(fabs(x));
    if (strcmp(name, "math.floor") == 0) return make_num(floor(x));
    if (strcmp(name, "math.max") == 0) return make_num(x > y ? x : y);
    if (strcmp(name, "math.min") == 0) return make_num(x < y ? x : y);
    if (strcmp(name, "math.sqrt") == 0) return make_num(sqrt(x));
    if (strcmp(name, "math.trunc") == 0) return make_num(trunc(x));
    if (strcmp(name, "error-new") == 0) {
        return make_error_value(argc == 0 ? "" : name_of(a[0]));
    }
    if (strcmp(name, "print-value") == 0) {
        print_value(a[0]);
        return V_UNDEF;
    }
    if (strcmp(name, "is-true") == 0) return make_bool(!is_false_value(a[0]));
    {
        char message[256];
        snprintf(message, sizeof message, "unknown primitive procedure %s", name);
        die(message);
    }
    return V_UNDEF;
}

static Val *make_primitive(const char *name) {
    const char **names = checked_malloc(2 * sizeof *names);
    Val **vals = checked_malloc(2 * sizeof *vals);
    names[0] = intern("kind");
    vals[0] = make_sym("primitive");
    names[1] = intern("name");
    vals[1] = make_str(name);
    return make_record(names, vals, 2);
}
static Val *make_bound_primitive(const char *name, Val *receiver) {
    const char **names = checked_malloc(3 * sizeof *names);
    Val **values = checked_malloc(3 * sizeof *values);
    names[0] = intern("kind");
    values[0] = make_sym("primitive");
    names[1] = intern("name");
    values[1] = make_str(name);
    names[2] = intern("receiver");
    values[2] = receiver;
    return make_record(names, values, 3);
}

static bool is_primitive(Val *value) {
    Val *kind = value->tag == T_RECORD ? record_get(value, "kind") : V_UNDEF;
    return kind->tag == T_SYM && strcmp(kind->u.text, "primitive") == 0;
}

/* ------------------------------------------------------------------ */
/* The canonical machine-operation table                               */
/* ------------------------------------------------------------------ */

static Val *lookupVariableValue(Val *var, Val *env) {
    Val *cell = lookup_binding(env_of(env), name_of(var));
    if (cell == NULL) {
        char message[256];
        snprintf(message, sizeof message, "unbound variable %s", name_of(var));
        die(message);
    }
    return cell;
}

static Val *setVariableValue(Val *var, Val *value, Val *env) {
    const char *name = name_of(var);
    for (Env *frame = env_of(env); frame != NULL; frame = frame->parent) {
        for (size_t i = 0; i < frame->names->u.seq.len; i += 1) {
            if (strcmp(frame->names->u.seq.items[i]->u.text, name) == 0) {
                frame->vals->u.seq.items[i] = value;
                return value;
            }
        }
    }
    die("set-variable-value on an unbound variable");
    return V_UNDEF;
}

static Val *defineVariableValue(Val *var, Val *value, Val *env) {
    Env *frame = env_of(env);
    const char *name = name_of(var);
    for (size_t i = 0; i < frame->names->u.seq.len; i += 1) {
        if (strcmp(frame->names->u.seq.items[i]->u.text, name) == 0) {
            frame->vals->u.seq.items[i] = value;
            return value;
        }
    }
    size_t len = frame->names->u.seq.len;
    Val **names = checked_grow(frame->names->u.seq.items, (len + 1) * sizeof *names);
    Val **vals = checked_grow(frame->vals->u.seq.items, (len + 1) * sizeof *vals);
    names[len] = make_sym(name);
    vals[len] = value;
    frame->names->u.seq.items = names;
    frame->vals->u.seq.items = vals;
    frame->names->u.seq.len = len + 1;
    frame->vals->u.seq.len = len + 1;
    return value;
}

static Val *extendEnvironment(Val *params, Val *values, Val *base) {
    if (params->tag != T_ARRAY && params->tag != T_LIST) {
        Val **pitems = checked_malloc(sizeof *pitems);
        pitems[0] = params;
        params = make_seq(T_ARRAY, pitems, 1);
        Val **vitems = checked_malloc(sizeof *vitems);
        vitems[0] = values;
        values = make_seq(T_ARRAY, vitems, 1);
    } else if (values->tag != T_ARRAY && values->tag != T_LIST) {
        Val **items = checked_malloc(sizeof *items);
        items[0] = values;
        values = make_seq(T_ARRAY, items, 1);
    }
    if (params->u.seq.len != values->u.seq.len) {
        char message[128];
        snprintf(message, sizeof message, "wrong number of arguments: %zu parameters, %zu values",
                 params->u.seq.len, values->u.seq.len);
        die(message);
    }
    return env_word(make_env(params, values, env_of(base)));
}

static Val *makeProcedure(Val *params, Val *body, Val *env) {
    Val *value = new_value(T_CLOSURE);
    value->u.proc.params = params;
    value->u.proc.body = body;
    value->u.proc.env = env_of(env);
    return value;
}

static Val *makeCompiledProcedure(void *entry, Val *params, Val *env) {
    Val *value = new_value(T_COMPILED);
    value->u.compiled.entry = entry;
    value->u.compiled.params = params;
    value->u.compiled.env = env_of(env);
    return value;
}

static void *compiledProcedureEntry(Val *proc) {
    if (proc->tag != T_COMPILED) {
        die("compiled-procedure-entry on a non-compiled word");
    }
    return proc->u.compiled.entry;
}

static Val *procedureEntryWord(Val *proc) {
    return addr_word(compiledProcedureEntry(proc));
}

static Val *compiledProcedureParameters(Val *proc) {
    if (proc->tag == T_COMPILED) return proc->u.compiled.params;
    if (proc->tag == T_CLOSURE) return proc->u.proc.params;
    die("procedure-parameters on a non-procedure word");
    return V_UNDEF;
}

static Val *compiledProcedureEnvironment(Val *proc) {
    if (proc->tag == T_COMPILED) return env_word(proc->u.compiled.env);
    if (proc->tag == T_CLOSURE) return env_word(proc->u.proc.env);
    die("procedure-environment on a non-procedure word");
    return V_UNDEF;
}


static Val *isPrimitiveProcedure(Val *proc) { return make_bool(is_primitive(proc)); }

static Val *isCompiledProcedure(Val *proc) { return make_bool(proc->tag == T_COMPILED); }

static Val *applyProcedure(Val *proc, Val *argl) {
    if (!is_primitive(proc)) {
        die("apply-procedure reached a non-primitive: compound apply is controller flow");
    }
    const char *name = name_of(record_get(proc, "name"));
    Val *receiver = record_get(proc, "receiver");
    if (receiver == V_UNDEF) return apply_primitive(name, argl);
    size_t len = argl->u.seq.len;
    Val **args = checked_malloc((len + 1) * sizeof *args);
    args[0] = receiver;
    for (size_t i = 0; i < len; i += 1) args[i + 1] = argl->u.seq.items[i];
    return apply_primitive(name, make_seq(T_LIST, args, len + 1));
}

static Val *emptyArgList(void) { return make_seq(T_LIST, checked_malloc(sizeof(Val *)), 0); }

/* The book's adjoin-arg: the argument joins the end of the operand
   list, so the parameter order matches the call's argument order. */
static Val *adjoinArg(Val *value, Val *argl) {
    size_t len = argl->u.seq.len;
    Val **items = checked_malloc((len + 1) * sizeof *items);
    for (size_t i = 0; i < len; i += 1) {
        items[i] = argl->u.seq.items[i];
    }
    items[len] = value;
    return make_seq(T_LIST, items, len + 1);
}

static Val *noArgs(Val *argl) { return make_bool(argl->u.seq.len == 0); }

static Val *firstArg(Val *argl) {
    if (argl->u.seq.len == 0) {
        die("first-arg of an empty argument list");
    }
    return argl->u.seq.items[0];
}

static Val *restArgs(Val *argl) {
    return make_seq(T_LIST, argl->u.seq.items + 1, argl->u.seq.len - 1);
}

static Val *isLastArg(Val *argl) { return make_bool(argl->u.seq.len == 1); }

static Val *isTrue(Val *value) { return make_bool(!is_false_value(value)); }

static Val *isFalse(Val *value) { return make_bool(is_false_value(value)); }

static Val *render(Val *value) {
    switch (value->tag) {
        case T_NUM: {
            char text[64];
            snprintf(text, sizeof text, "%.15g", value->u.num);
            return make_str(text);
        }
        case T_STR:
        case T_SYM: return make_str(value->u.text);
        case T_BOOL: return make_str(value->u.boolean ? "true" : "false");
        case T_NULL: return make_str("null");
        case T_UNDEF: return make_str("undefined");
        default: return make_str("[value]");
    }
}

static void print_value(Val *value) {
    Val *text = render(value);
    printf("%s\n", text->u.text);
}

static Val *memberGet(Val *record, Val *name) { return record_get(record, name_of(name)); }

static Val *memberSet(Val *record, Val *name, Val *value) {
    for (size_t i = 0; i < record->u.rec.len; i += 1) {
        if (strcmp(record->u.rec.names[i], name_of(name)) == 0) {
            record->u.rec.vals[i] = value;
            return value;
        }
    }
    die("member-set on an undeclared record field");
    return V_UNDEF;
}

static Val *indexGet(Val *object, Val *index) {
    double at = index->u.num;
    if (object->tag == T_ARRAY || object->tag == T_LIST) {
        return (at < 0 || (size_t)at >= object->u.seq.len) ? V_UNDEF : object->u.seq.items[(size_t)at];
    }
    if (object->tag == T_MAP) {
        for (size_t i = 0; i < object->u.mapv.len; i += 1) {
            if (object->u.mapv.keys[i]->tag == T_NUM && object->u.mapv.keys[i]->u.num == at) {
                return object->u.mapv.vals[i];
            }
        }
        return V_UNDEF;
    }
    die("index-get on a non-sequence");
    return V_UNDEF;
}

static Val *indexSet(Val *object, Val *index, Val *value) {
    double at = index->u.num;
    if (object->tag != T_ARRAY || at < 0 || (size_t)at >= object->u.seq.len) {
        die("index-set out of range");
    }
    object->u.seq.items[(size_t)at] = value;
    return value;
}

static bool is_truthy(Val *value) { return !is_false_value(value); }

static Val *arrayNew(Val *argl) {
    size_t len = argl->u.seq.len;
    Val **items = checked_malloc((len == 0 ? 1 : len) * sizeof *items);
    for (size_t i = 0; i < len; i += 1) items[i] = argl->u.seq.items[i];
    return make_seq(T_ARRAY, items, len);
}

static Val *arrayIndex(Val *array, Val *index) { return indexGet(array, index); }

static Val *arrayLength(Val *value) {
    if (value->tag == T_ARRAY || value->tag == T_LIST) return make_num((double)value->u.seq.len);
    if (value->tag == T_MAP) return make_num((double)value->u.mapv.len);
    if (value->tag == T_SET) return make_num((double)value->u.seq.len);
    if (value->tag == T_STR) return make_num((double)strlen(value->u.text));
    return make_num(0);
}

static Val *recordGetValue(Val *object, Val *field) {
    const char *name = name_of(field);
    if (object->tag == T_RECORD || object->tag == T_ERROR) return record_get(object, name);
    if (object->tag == T_ARRAY && strcmp(name, "length") == 0) return arrayLength(object);
    if (object->tag == T_MAP) {
        if (strcmp(name, "size") == 0) return arrayLength(object);
        if (strcmp(name, "get") == 0) return make_bound_primitive("map.get", object);
        if (strcmp(name, "set") == 0) return make_bound_primitive("map.set", object);
    }
    if (object->tag == T_SET && strcmp(name, "size") == 0) return arrayLength(object);
    if (object->tag == T_STR && strcmp(name, "length") == 0) return arrayLength(object);
    return V_UNDEF;
}

static Val *recordSetValue(Val *object, Val *field, Val *value) {
    if (object->tag != T_RECORD) die("record-set on a non-record");
    const char *name = name_of(field);
    for (size_t i = 0; i < object->u.rec.len; i += 1) {
        if (strcmp(object->u.rec.names[i], name) == 0) {
            object->u.rec.vals[i] = value;
            return value;
        }
    }
    size_t len = object->u.rec.len;
    object->u.rec.names = checked_grow(object->u.rec.names, (len + 1) * sizeof *object->u.rec.names);
    object->u.rec.vals = checked_grow(object->u.rec.vals, (len + 1) * sizeof *object->u.rec.vals);
    object->u.rec.names[len] = intern(name);
    object->u.rec.vals[len] = value;
    object->u.rec.len = len + 1;
    return value;
}

static Val *recordFrom(Val *keys, Val *values) {
    if (keys->u.seq.len != values->u.seq.len) die("record-from field count mismatch");
    size_t len = keys->u.seq.len;
    const char **names = checked_malloc((len == 0 ? 1 : len) * sizeof *names);
    for (size_t i = 0; i < len; i += 1) names[i] = intern(name_of(keys->u.seq.items[i]));
    return make_record(names, values->u.seq.items, len);
}

static Val *mapNew(Val *argl) {
    Val **keys = checked_malloc(sizeof *keys);
    Val **values = checked_malloc(sizeof *values);
    Val *map = make_map(keys, values, 0);
    if (argl->u.seq.len == 0) return map;
    Val *entries = argl->u.seq.items[0];
    if (entries->tag != T_ARRAY && entries->tag != T_LIST) die("Map constructor expects entry pairs");
    for (size_t i = 0; i < entries->u.seq.len; i += 1) {
        Val *pair = entries->u.seq.items[i];
        if ((pair->tag != T_ARRAY && pair->tag != T_LIST) || pair->u.seq.len != 2) {
            die("Map constructor entry must be a pair");
        }
        map_set(map, pair->u.seq.items[0], pair->u.seq.items[1]);
    }
    return map;
}

static Val *parentEnvironment(Val *env) { return env_word(env_of(env)->parent); }

static Val *apply_binary(const char *operation, Val *left, Val *right) {
    Val *args[] = { left, right };
    return apply_primitive(operation, make_seq(T_LIST, args, 2));
}

static Val *numberAdd(Val *left, Val *right) { return apply_binary("number-add", left, right); }
static Val *numberLess(Val *left, Val *right) { return apply_binary("number-less", left, right); }
static Val *numberLessEqual(Val *left, Val *right) { return apply_binary("number-less-equal", left, right); }
static Val *numberMultiply(Val *left, Val *right) { return apply_binary("number-multiply", left, right); }
static Val *numberSubtract(Val *left, Val *right) { return apply_binary("number-subtract", left, right); }
static Val *stringConcat(Val *left, Val *right) { return apply_binary("string-concat", left, right); }
static Val *booleanNot(Val *value) { return make_bool(!is_truthy(value)); }
static Val *valueEqual(Val *left, Val *right) { return make_bool(same_value(left, right)); }
static Val *valueNotEqual(Val *left, Val *right) { return make_bool(!same_value(left, right)); }
static Val *isErrorValue(Val *value) { return make_bool(value->tag == T_MACHINE_ERROR); }
static Val *printValue(Val *value) { print_value(value); return V_UNDEF; }

static Val *typeofValue(Val *value) {
    if (value->tag == T_NUM) return make_str("number");
    if (value->tag == T_STR) return make_str("string");
    if (value->tag == T_BOOL) return make_str("boolean");
    if (value->tag == T_UNDEF) return make_str("undefined");
    if (value->tag == T_CLOSURE || value->tag == T_COMPILED || value->tag == T_PRIM) return make_str("function");
    return make_str("object");
}

static Val *throwBox(Val *value) {
    Val *transfer = new_value(T_TRANSFER);
    transfer->u.transfer.is_error = value->tag == T_MACHINE_ERROR;
    transfer->u.transfer.value = value;
    return transfer;
}

static Val *throwPending(Val *value) { return make_bool(value->tag == T_TRANSFER); }
static Val *throwErrorPending(Val *value) {
    return make_bool(value->tag == T_TRANSFER && value->u.transfer.is_error);
}

static Val *errorNew(Val *argl) {
    Val *message = argl->u.seq.len == 0 ? make_str("") : render(argl->u.seq.items[0]);
    return make_error_value(message->u.text);
}

/* The syntax predicates operate on the shared AST's record shape, the
   same typed data the wire carries. */
static Val *isSelfEvaluating(Val *node) {
    return make_bool(tag_is(node, "number") || tag_is(node, "string") || tag_is(node, "boolean") ||
                     tag_is(node, "null") || tag_is(node, "undefined"));
}

static Val *isVariable(Val *node) { return make_bool(tag_is(node, "variable")); }
static Val *isAssignment(Val *node) { return make_bool(tag_is(node, "assign")); }
static Val *isDeclaration(Val *node) { return make_bool(tag_is(node, "var-decl")); }
static Val *isArrow(Val *node) { return make_bool(tag_is(node, "arrow")); }
static Val *arrowParams(Val *node) { return record_get(node, "params"); }
static Val *arrowBody(Val *node) { return record_get(node, "body"); }
static Val *isIf(Val *node) { return make_bool(tag_is(node, "if")); }
static Val *isBlock(Val *node) { return make_bool(tag_is(node, "block")); }
static Val *isCall(Val *node) { return make_bool(tag_is(node, "call")); }
static Val *callCallee(Val *node) { return record_get(node, "callee"); }
static Val *callArgs(Val *node) { return record_get(node, "args"); }
static Val *isDataLiteral(Val *node) {
    return make_bool(tag_is(node, "array") || tag_is(node, "object"));
}
static Val *dataLiteralValue(Val *node) { return node; }

static void init_global(void) {
    static const char *const names[] = {
        "+", "-", "*", "/", "%", "<", "<=", ">", ">=", "=", "===",
        "!==", "!", "string-concat", "math.abs", "math.floor", "math.max",
        "math.min", "math.sqrt", "math.trunc", "error-new",
    };
    Val *name_words = emptyArgList();
    Val *vals = emptyArgList();
    for (size_t i = 0; i < sizeof names / sizeof names[0]; i += 1) {
        name_words = cons(make_sym(names[i]), name_words);
        vals = cons(make_primitive(names[i]), vals);
    }
    global_environment = make_env(name_words, vals, NULL);
    Val *global = env_word(global_environment);
    defineVariableValue(make_sym("undefined"), V_UNDEF, global);
    defineVariableValue(make_sym("null"), V_NULL, global);
}

/* ------------------------------------------------------------------ */
/* Entry: the appended compiled forms call run_compiled(void)           */
/* ------------------------------------------------------------------ */

static void run_compiled(void);

int main(void) {
    V_UNDEF = new_value(T_UNDEF);
    V_NULL = new_value(T_NULL);
    V_TRUE = new_value(T_BOOL);
    V_TRUE->u.boolean = true;
    V_FALSE = new_value(T_BOOL);
    V_FALSE->u.boolean = false;
    initializeStack();
    init_global();
    run_compiled();
    return EXIT_SUCCESS;
}
