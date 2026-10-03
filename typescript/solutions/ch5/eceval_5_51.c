/* SPDX-License-Identifier: GPL-3.0-only */
/* Exercise 5.51: the explicit-control evaluator of section 5.4
   translated into C, in the guest runtime's object world.

   The translation is the controller itself. The machine's registers are
   file-scope variables, every ev- entry point of 5.4.1 to 5.4.4 is a C
   label, (assign continue (label l)) stores a label address, (goto (reg
   continue)) is `goto *R_continue` (the computed-goto extension the
   system compiler accepts in the GNU dialect), and save/restore are push
   and pop on a growable, tag-checked stack.

   Object world (consumer contract section 6): numbers are IEEE-754
   doubles (never long long), strings are const char *, booleans are int,
   and records/arrays/maps/sets/lists are runtime structs. There are no
   pairs and no symbol/nil guest object model.

   Input is serialized typed data, never source text and never a second
   parser: the boundary driver serializes the shared Program/Expr data as
   JSON (the "program" input) or a typed MachineStatement stream (the
   "machine" input). The read-eval-print loop runs over the translated
   controller; console.log output matches the direct evaluator transcript,
   and runtime faults use the `Error: <message>` prefix. */

#include <stdbool.h>
#include <stddef.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ------------------------------------------------------------------ */
/* The value world                                                      */
/* ------------------------------------------------------------------ */

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
    T_PRIM,
    T_ENV
} Tag;

typedef struct Val Val;
typedef struct Env Env;

struct Val {
    Tag tag;
    union {
        double num;
        bool boolean;
        const char *text; /* string or symbol name */
        struct {
            Val **items;
            size_t len;
        } seq; /* list, array, set */
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
        } closure;
        Env *environment;
    } u;
};

struct Env {
    Val *names;
    Val *vals;
    Env *parent;
};

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

static Val *make_num(double n) {
    Val *value = new_value(T_NUM);
    value->u.num = n;
    return value;
}

static Val *make_bool(bool b) { return b ? V_TRUE : V_FALSE; }

static const char *intern(const char *text) {
    /* Strings and symbol names live for the whole process; the teaching
       runtime never frees them. */
    size_t length = strlen(text) + 1;
    char *copy = checked_malloc(length);
    memcpy(copy, text, length);
    return copy;
}

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

static Val *make_closure(Val *params, Val *body, Env *env) {
    Val *value = new_value(T_CLOSURE);
    value->u.closure.params = params;
    value->u.closure.body = body;
    value->u.closure.env = env;
    return value;
}

/* The guest value a record read answers for a missing field. */
static Val *record_get(Val *record, const char *name) {
    if (record->tag != T_RECORD) {
        die("record_get: not a record");
    }
    for (size_t i = 0; i < record->u.rec.len; i += 1) {
        if (strcmp(record->u.rec.names[i], name) == 0) {
            return record->u.rec.vals[i];
        }
    }
    return V_UNDEF;
}

static const char *record_name(Val *record, const char *field) {
    Val *value = record_get(record, field);
    if (value->tag != T_SYM && value->tag != T_STR) {
        die("expected a name in the syntax data");
    }
    return value->u.text;
}

static bool record_tag_is(Val *node, const char *tag) {
    if (node->tag != T_RECORD) {
        return false;
    }
    Val *value = record_get(node, "tag");
    return (value->tag == T_SYM || value->tag == T_STR) && strcmp(value->u.text, tag) == 0;
}

/* ------------------------------------------------------------------ */
/* The JSON reader for serialized typed data                            */
/* ------------------------------------------------------------------ */

typedef struct {
    const char *text;
    size_t position;
} Reader;

static void reader_space(Reader *reader) {
    while (reader->text[reader->position] == ' ' || reader->text[reader->position] == '\t' ||
           reader->text[reader->position] == '\n' || reader->text[reader->position] == '\r') {
        reader->position += 1;
    }
}

static Val *read_json(Reader *reader);

static void read_string_into(Reader *reader, char *out, size_t capacity) {
    size_t at = 0;
    if (reader->text[reader->position] != '"') {
        die("expected a JSON string");
    }
    reader->position += 1;
    while (reader->text[reader->position] != '"') {
        char ch = reader->text[reader->position];
        if (ch == '\0') {
            die("unterminated JSON string");
        }
        if (ch == '\\') {
            reader->position += 1;
            char esc = reader->text[reader->position];
            ch = esc == 'n' ? '\n' : esc == 't' ? '\t' : esc == 'r' ? '\r' : esc;
        }
        if (at + 1 >= capacity) {
            die("JSON string too long for the reader buffer");
        }
        out[at++] = ch;
        reader->position += 1;
    }
    reader->position += 1;
    out[at] = '\0';
}

static Val *read_json_value(Reader *reader) {
    reader_space(reader);
    char ch = reader->text[reader->position];
    if (ch == '{' ) {
        reader->position += 1;
        size_t capacity = 8, len = 0;
        const char **names = checked_malloc(capacity * sizeof *names);
        Val **vals = checked_malloc(capacity * sizeof *vals);
        reader_space(reader);
        if (reader->text[reader->position] == '}') {
            reader->position += 1;
            return make_record(names, vals, 0);
        }
        for (;;) {
            char name[256];
            reader_space(reader);
            read_string_into(reader, name, sizeof name);
            names[len] = intern(name);
            reader_space(reader);
            if (reader->text[reader->position] != ':') {
                die("expected : in a JSON object");
            }
            reader->position += 1;
            vals[len] = read_json(reader);
            len += 1;
            reader_space(reader);
            char sep = reader->text[reader->position];
            reader->position += 1;
            if (sep == '}') {
                break;
            }
            if (sep != ',') {
                die("expected , or } in a JSON object");
            }
            if (len == capacity) {
                capacity *= 2;
                names = checked_grow(names, capacity * sizeof *names);
                vals = checked_grow(vals, capacity * sizeof *vals);
            }
        }
        return make_record(names, vals, len);
    }
    if (ch == '[') {
        reader->position += 1;
        size_t capacity = 8, len = 0;
        Val **items = checked_malloc(capacity * sizeof *items);
        reader_space(reader);
        if (reader->text[reader->position] == ']') {
            reader->position += 1;
            return make_seq(T_ARRAY, items, 0);
        }
        for (;;) {
            if (len == capacity) {
                capacity *= 2;
                items = checked_grow(items, capacity * sizeof *items);
            }
            items[len++] = read_json(reader);
            reader_space(reader);
            char sep = reader->text[reader->position];
            reader->position += 1;
            if (sep == ']') {
                break;
            }
            if (sep != ',') {
                die("expected , or ] in a JSON array");
            }
        }
        return make_seq(T_ARRAY, items, len);
    }
    if (ch == '"') {
        char text[1024];
        read_string_into(reader, text, sizeof text);
        return make_str(text);
    }
    if (strncmp(reader->text + reader->position, "true", 4) == 0) {
        reader->position += 4;
        return V_TRUE;
    }
    if (strncmp(reader->text + reader->position, "false", 5) == 0) {
        reader->position += 5;
        return V_FALSE;
    }
    if (strncmp(reader->text + reader->position, "null", 4) == 0) {
        reader->position += 4;
        return V_NULL;
    }
    char *end = NULL;
    double number = strtod(reader->text + reader->position, &end);
    if (end == reader->text + reader->position) {
        die("unexpected JSON input");
    }
    reader->position = (size_t)(end - reader->text);
    return make_num(number);
}

/* A tag field names a variant; JSON strings arrive as strings and are
   read as variant names here. */
static Val *read_json(Reader *reader) { return read_json_value(reader); }

/* ------------------------------------------------------------------ */
/* Environments, faults, and the machine stack                          */
/* ------------------------------------------------------------------ */

static void runtime_error(const char *message) {
    printf("Error: %s\n", message);
    exit(EXIT_SUCCESS);
}

static Env *make_env(Val *names, Val *vals, Env *parent) {
    Env *env = checked_malloc(sizeof *env);
    env->names = names;
    env->vals = vals;
    env->parent = parent;
    return env;
}

static Val *env_names_array(Env *env) { return env->names; }

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

static Val *lookup_variable(const char *name, Env *env) {
    Val *cell = lookup_binding(env, name);
    if (cell == NULL) {
        char message[256];
        snprintf(message, sizeof message, "unbound variable %s", name);
        runtime_error(message);
    }
    return cell;
}

static void define_variable(const char *name, Val *value, Env *env) {
    for (size_t i = 0; i < env->names->u.seq.len; i += 1) {
        if (strcmp(env->names->u.seq.items[i]->u.text, name) == 0) {
            env->vals->u.seq.items[i] = value;
            return;
        }
    }
    size_t len = env->names->u.seq.len;
    Val **names = checked_grow(env->names->u.seq.items, (len + 1) * sizeof *names);
    Val **vals = checked_grow(env->vals->u.seq.items, (len + 1) * sizeof *vals);
    names[len] = make_sym(name);
    vals[len] = value;
    env->names->u.seq.items = names;
    env->vals->u.seq.items = vals;
    env->names->u.seq.len = len + 1;
    env->vals->u.seq.len = len + 1;
}

static void set_variable(const char *name, Val *value, Env *env) {
    Val *cell = lookup_binding(env, name);
    if (cell == NULL) {
        char message[256];
        snprintf(message, sizeof message, "unbound variable %s", name);
        runtime_error(message);
    }
    for (Env *frame = env; frame != NULL; frame = frame->parent) {
        for (size_t i = 0; i < frame->names->u.seq.len; i += 1) {
            if (strcmp(frame->names->u.seq.items[i]->u.text, name) == 0) {
                frame->vals->u.seq.items[i] = value;
                return;
            }
        }
    }
}

/* One machine stack slot: a saved register value, or a saved continue
   address. The two are tagged so a mismatched restore is caught, the
   lesson of 5.2's stack discipline. */
typedef struct {
    Val *value;
    void *address;
    bool is_address;
} StackSlot;

static StackSlot *machine_stack;
static size_t stack_capacity, stack_pointer;

static void stack_push(StackSlot slot) {
    if (stack_pointer == stack_capacity) {
        size_t next = stack_capacity == 0 ? 256 : stack_capacity * 2;
        machine_stack = checked_grow(machine_stack, next * sizeof *machine_stack);
        stack_capacity = next;
    }
    machine_stack[stack_pointer++] = slot;
}

static StackSlot stack_pop(void) {
    if (stack_pointer == 0) {
        runtime_error("stack underflow");
    }
    return machine_stack[--stack_pointer];
}

static void push_val(Val *value) {
    StackSlot slot;
    slot.value = value;
    slot.address = NULL;
    slot.is_address = false;
    stack_push(slot);
}

static void push_addr(void *address) {
    StackSlot slot;
    slot.value = NULL;
    slot.address = address;
    slot.is_address = true;
    stack_push(slot);
}

static Val *pop_val(void) {
    StackSlot slot = stack_pop();
    if (slot.is_address) {
        runtime_error("restore mismatch: expected a value slot");
    }
    return slot.value;
}

static void *pop_addr(void) {
    StackSlot slot = stack_pop();
    if (!slot.is_address) {
        runtime_error("restore mismatch: expected an address slot");
    }
    return slot.address;
}

/* ------------------------------------------------------------------ */
/* Primitives: the guest primitive procedures of the object world       */
/* ------------------------------------------------------------------ */

static void print_guest_value(Val *value);
static Val *apply_primitive(const char *name, Val *args) {
    double a = args->u.seq.len > 0 && args->u.seq.items[0]->tag == T_NUM ? args->u.seq.items[0]->u.num : 0;
    double b = args->u.seq.len > 1 && args->u.seq.items[1]->tag == T_NUM ? args->u.seq.items[1]->u.num : 0;
    if (strcmp(name, "number-add") == 0) return make_num(a + b);
    if (strcmp(name, "number-subtract") == 0) return make_num(a - b);
    if (strcmp(name, "number-multiply") == 0) return make_num(a * b);
    if (strcmp(name, "number-divide") == 0) return make_num(a / b);
    if (strcmp(name, "number-remainder") == 0) return make_num(fmod(a, b));
    if (strcmp(name, "number-less") == 0) return make_bool(a < b);
    if (strcmp(name, "number-less-equal") == 0) return make_bool(a <= b);
    if (strcmp(name, "number-greater") == 0) return make_bool(a > b);
    if (strcmp(name, "number-greater-equal") == 0) return make_bool(a >= b);
    if (strcmp(name, "value-equal") == 0) {
        return make_bool(args->u.seq.items[0]->tag == T_NUM && args->u.seq.items[1]->tag == T_NUM
                             ? a == b
                             : args->u.seq.items[0] == args->u.seq.items[1]);
    }
    if (strcmp(name, "string-concat") == 0) {
        const char *left = args->u.seq.items[0]->u.text;
        const char *right = args->u.seq.items[1]->u.text;
        size_t length = strlen(left) + strlen(right) + 1;
        char *joined = checked_malloc(length);
        snprintf(joined, length, "%s%s", left, right);
        return make_str(joined);
    }
    if (strcmp(name, "print-value") == 0) {
        print_guest_value(args->u.seq.items[0]);
        return V_UNDEF;
    }
    if (strcmp(name, "is-true") == 0) return make_bool(!(args->u.seq.items[0]->tag == T_BOOL && !args->u.seq.items[0]->u.boolean));
    char message[256];
    snprintf(message, sizeof message, "unknown primitive procedure %s", name);
    runtime_error(message);
    return V_UNDEF;
}

/* The primitive-procedure word: a name in a record with the primitive
   tag, so apply-dispatch can distinguish it from a closure. */
static Val *make_primitive(const char *name) {
    const char **names = checked_malloc(2 * sizeof *names);
    Val **vals = checked_malloc(2 * sizeof *vals);
    names[0] = intern("kind");
    vals[0] = make_sym("primitive");
    names[1] = intern("name");
    vals[1] = make_str(name);
    return make_record(names, vals, 2);
}

static bool is_primitive(Val *value) {
    Val *kind = value->tag == T_RECORD ? record_get(value, "kind") : V_UNDEF;
    return kind->tag == T_SYM && strcmp(kind->u.text, "primitive") == 0;
}

/* ------------------------------------------------------------------ */
/* Syntax data predicates: the shared AST as records                    */
/* ------------------------------------------------------------------ */

static bool self_evaluating(Val *node) {
    return record_tag_is(node, "number") || record_tag_is(node, "string") ||
           record_tag_is(node, "boolean") || record_tag_is(node, "null") ||
           record_tag_is(node, "undefined");
}

static const char *node_text(Val *node, const char *field) {
    Val *value = record_get(node, field);
    if (value->tag != T_STR && value->tag != T_SYM) {
        runtime_error("expected a string in the syntax data");
    }
    return value->u.text;
}

static Val *node_field(Val *node, const char *field) {
    Val *value = record_get(node, field);
    if (value == V_UNDEF) {
        runtime_error("missing field in the syntax data");
    }
    return value;
}

static bool is_false_value(Val *value) {
    return value->tag == T_BOOL && !value->u.boolean;
}

/* Render console.log values in the direct evaluator's transcript format. */
static void print_guest_value(Val *value) {
    switch (value->tag) {
        case T_NUM: printf("%.15g\n", value->u.num); return;
        case T_STR: printf("%s\n", value->u.text); return;
        case T_BOOL: printf("%s\n", value->u.boolean ? "true" : "false"); return;
        case T_NULL: printf("null\n"); return;
        case T_UNDEF: printf("undefined\n"); return;
        case T_SYM: printf("%s\n", value->u.text); return;
        case T_CLOSURE: printf("[function]\n"); return;
        case T_PRIM: printf("[primitive]\n"); return;
        case T_LIST:
        case T_ARRAY: {
            printf("[");
            for (size_t i = 0; i < value->u.seq.len; i += 1) {
                if (i > 0) printf(", ");
                Val *item = value->u.seq.items[i];
                if (item->tag == T_NUM) printf("%g", item->u.num);
                else if (item->tag == T_STR) printf("%s", item->u.text);
                else printf("[value]");
            }
            printf("]\n");
            return;
        }
        case T_RECORD: printf("[record]\n"); return;
        case T_MAP: printf("[map]\n"); return;
        case T_SET: printf("[set]\n"); return;
        case T_ENV: printf("[environment]\n"); return;
    }
}

/* ------------------------------------------------------------------ */
/* The explicit-control evaluator, translated                          */
/* ------------------------------------------------------------------ */

/* The translation keeps the controller's own vocabulary. The machine's
   registers are file-scope variables; every ev- entry point of 5.4.1 to
   5.4.4 is a C label; (assign continue (label l)) stores a label
   address; (goto (reg continue)) is `goto *R_continue`; save/restore
   are the tagged stack above. The apply frame is the book's discipline:
   ev-appl saves continue once, primitive-apply restores it, and a
   compound body returns through the frame marker (ev-return-exit) that
   also unwinds a non-tail `return` statement to its function. */

static Val *R_exp, *R_val, *R_proc, *R_argl, *R_unev;
static Env *R_env;
static void *R_continue;
static void *R_seq_end;
static Val *R_forms;
static size_t R_form_index;

static Val *env_word(Env *env) {
    Val *value = new_value(T_ENV);
    value->u.environment = env;
    return value;
}

static Env *env_of(Val *value) {
    if (value->tag != T_ENV) {
        runtime_error("expected an environment word");
    }
    return value->u.environment;
}

static bool is_closure(Val *value) {
    Val *kind = value->tag == T_RECORD ? record_get(value, "kind") : V_UNDEF;
    return kind->tag == T_SYM && strcmp(kind->u.text, "closure") == 0;
}

static Val *list_empty(void) { return make_seq(T_LIST, checked_malloc(sizeof(Val *)), 0); }

/* Arguments accumulate left to right: the argument joins the end of the
   operand list, the book's adjoin-arg. */
static Val *adjoin_arg(Val *argl, Val *argument) {
    size_t len = argl->u.seq.len;
    Val **items = checked_malloc((len + 1) * sizeof *items);
    for (size_t i = 0; i < len; i += 1) {
        items[i] = argl->u.seq.items[i];
    }
    items[len] = argument;
    return make_seq(T_LIST, items, len + 1);
}

/* A parameter list is a list of Param records; an environment frame
   stores the name words. */
static Val *param_names(Val *params) {
    Val *names = list_empty();
    for (size_t i = 0; i < params->u.seq.len; i += 1) {
        Val *param = params->u.seq.items[i];
        names = adjoin_arg(names, make_sym(node_text(param, "name")));
    }
    return names;
}

/* A call argument is an Arg record; the spread kind is not admitted by
   the core grammar's call sites here. */
static Val *arg_expr(Val *arg) {
    Val *kind = record_get(arg, "kind");
    if ((kind->tag == T_SYM || kind->tag == T_STR) &&
        (strcmp(kind->u.text, "item") == 0 || strcmp(kind->u.text, "spread") == 0)) {
        return node_field(arg, "expr");
    }
    return arg;
}

static Env *extend_environment(Val *params, Val *args, Env *base) {
    if (params->u.seq.len != args->u.seq.len) {
        runtime_error("wrong number of arguments");
    }
    return make_env(params, args, base);
}

static Val *stack_peek_val(void) {
    if (stack_pointer == 0) {
        runtime_error("stack underflow");
    }
    StackSlot slot = machine_stack[stack_pointer - 1];
    if (slot.is_address) {
        runtime_error("expected a value slot");
    }
    return slot.value;
}

static void stack_unwind_to_marker(void) {
    for (;;) {
        StackSlot slot = stack_pop();
        if (slot.is_address && slot.address == NULL) {
            return;
        }
    }
}

static void eval_controller(void) {
    void *ev_dispatch = &&ev_dispatch_label;
    void *ev_self_eval = &&ev_self_eval_label;
    void *ev_variable = &&ev_variable_label;
    void *ev_declaration = &&ev_declaration_label;
    void *ev_function = &&ev_function_label;
    void *ev_assignment = &&ev_assignment_label;
    void *ev_if = &&ev_if_label;
    void *ev_return = &&ev_return_label;
    void *ev_while = &&ev_while_label;
    void *ev_appl = &&ev_appl_label;
    void *ev_appl_did_operator = &&ev_appl_did_operator_label;
    void *ev_appl_operand_loop = &&ev_appl_operand_loop_label;
    void *ev_appl_accumulate_arg = &&ev_appl_accumulate_arg_label;
    void *ev_appl_accum_last_arg = &&ev_appl_accum_last_arg_label;
    void *apply_dispatch = &&apply_dispatch_label;
    void *ev_primitive_apply = &&ev_primitive_apply_label;
    void *ev_compound_apply = &&ev_compound_apply_label;
    void *ev_operator = &&ev_operator_label;
    void *ev_member = &&ev_member_label;
    void *ev_sequence = &&ev_sequence_label;
    void *ev_sequence_continue = &&ev_sequence_continue_label;
    void *ev_sequence_last = &&ev_sequence_last_label;
    void *ev_block_done = &&ev_block_done_label;
    void *ev_return_exit = &&ev_return_exit_label;
    void *ev_top_done = &&ev_top_done_label;
    void *ev_next_form = &&ev_next_form_label;
    void *ev_while_top = &&ev_while_top_label;
    void *ev_while_decide = &&ev_while_decide_label;
    void *ev_while_body_done = &&ev_while_body_done_label;
    (void)ev_self_eval; (void)ev_variable; (void)ev_declaration; (void)ev_function;
    (void)ev_assignment; (void)ev_if; (void)ev_return; (void)ev_while; (void)ev_appl;
    (void)ev_appl_did_operator; (void)ev_appl_operand_loop; (void)ev_appl_accumulate_arg;
    (void)ev_appl_accum_last_arg; (void)apply_dispatch; (void)ev_primitive_apply;
    (void)ev_compound_apply; (void)ev_operator; (void)ev_member; (void)ev_sequence;
    (void)ev_sequence_continue; (void)ev_sequence_last; (void)ev_block_done;
    (void)ev_return_exit; (void)ev_top_done; (void)ev_next_form; (void)ev_while_top;
    (void)ev_while_decide; (void)ev_while_body_done;

    goto *ev_next_form;

ev_next_form_label:
    if (R_form_index >= R_forms->u.seq.len) {
        return;
    }
    R_exp = R_forms->u.seq.items[R_form_index];
    R_form_index += 1;
    push_addr(ev_top_done);
    push_val(env_word(R_env));
    push_addr(R_seq_end);
    {
        StackSlot marker;
        marker.value = NULL;
        marker.address = NULL;
        marker.is_address = true;
        stack_push(marker);
    }
    R_seq_end = ev_return_exit;
    R_unev = make_seq(T_ARRAY, &R_forms->u.seq.items[R_form_index - 1], 1);
    goto *ev_sequence;

ev_top_done_label:
    goto *ev_next_form;

ev_dispatch_label:
    if (self_evaluating(R_exp)) goto *ev_self_eval;
    if (record_tag_is(R_exp, "variable")) goto *ev_variable;
    if (record_tag_is(R_exp, "var-decl")) goto *ev_declaration;
    if (record_tag_is(R_exp, "function-decl")) goto *ev_function;
    if (record_tag_is(R_exp, "assign")) goto *ev_assignment;
    if (record_tag_is(R_exp, "if")) goto *ev_if;
    if (record_tag_is(R_exp, "return")) goto *ev_return;
    if (record_tag_is(R_exp, "while")) goto *ev_while;
    if (record_tag_is(R_exp, "expr-stmt")) {
        R_exp = node_field(R_exp, "expr");
        goto *ev_dispatch;
    }
    if (record_tag_is(R_exp, "call")) goto *ev_appl;
    if (record_tag_is(R_exp, "binary") || record_tag_is(R_exp, "unary") ||
        record_tag_is(R_exp, "logical") || record_tag_is(R_exp, "conditional") ||
        record_tag_is(R_exp, "array") || record_tag_is(R_exp, "object")) {
        goto *ev_operator;
    }
    if (record_tag_is(R_exp, "member") || record_tag_is(R_exp, "index")) goto *ev_member;
    if (record_tag_is(R_exp, "block")) {
        R_unev = node_field(R_exp, "body");
        push_addr(R_continue);
        push_addr(R_seq_end);
        R_seq_end = ev_block_done;
        goto *ev_sequence;
    }
    runtime_error("unknown syntax node");

ev_self_eval_label:
    if (record_tag_is(R_exp, "number") || record_tag_is(R_exp, "string") || record_tag_is(R_exp, "boolean")) {
        R_val = node_field(R_exp, "value");
    } else {
        R_val = record_tag_is(R_exp, "null") ? V_NULL : V_UNDEF;
    }
    goto *R_continue;

ev_variable_label:
    R_val = lookup_variable(node_text(R_exp, "name"), R_env);
    goto *R_continue;

ev_declaration_label:
    push_val(R_exp);
    push_addr(R_continue);
    R_exp = node_field(R_exp, "init");
    R_continue = &&ev_declaration_done_label;
    goto *ev_dispatch;
ev_declaration_done_label: {
    R_continue = pop_addr();
    Val *declaration = pop_val();
    define_variable(node_text(declaration, "name"), R_val, R_env);
    goto *R_continue;
}

ev_function_label: {
    const char *name = node_text(R_exp, "name");
    const char **names = checked_malloc(4 * sizeof *names);
    Val **vals = checked_malloc(4 * sizeof *vals);
    names[0] = intern("kind");
    vals[0] = make_sym("closure");
    names[1] = intern("params");
    vals[1] = param_names(node_field(R_exp, "params"));
    names[2] = intern("body");
    vals[2] = node_field(R_exp, "body");
    names[3] = intern("environment");
    vals[3] = env_word(R_env);
    define_variable(name, make_record(names, vals, 4), R_env);
    R_val = V_UNDEF;
    goto *R_continue;
}

ev_assignment_label:
    push_val(R_exp);
    push_addr(R_continue);
    R_exp = node_field(R_exp, "value");
    R_continue = &&ev_assignment_done_label;
    goto *ev_dispatch;
ev_assignment_done_label: {
    R_continue = pop_addr();
    Val *assignment = pop_val();
    Val *target = node_field(assignment, "target");
    if (record_tag_is(target, "variable")) {
        set_variable(node_text(target, "name"), R_val, R_env);
        goto *R_continue;
    }
    if (record_tag_is(target, "member") || record_tag_is(target, "index")) {
        push_val(R_val);
        push_val(target);
        push_addr(R_continue);
        R_exp = node_field(target, "object");
        R_continue = &&ev_assignment_object_done_label;
        goto *ev_dispatch;
    }
    runtime_error("invalid assignment target");
ev_assignment_object_done_label: {
    Val *object = R_val;
    R_continue = pop_addr();
    Val *target = pop_val();
    Val *assigned = pop_val();
    if (record_tag_is(target, "member")) {
        const char *field = node_text(target, "name");
        if (object->tag != T_RECORD) {
            runtime_error("member assignment on a non-record");
        }
        for (size_t i = 0; i < object->u.rec.len; i += 1) {
            if (strcmp(object->u.rec.names[i], field) == 0) {
                object->u.rec.vals[i] = assigned;
                R_val = assigned;
                goto *R_continue;
            }
        }
        runtime_error("assignment to an undeclared record field");
    }
    push_val(object);
    push_val(assigned);
    push_addr(R_continue);
    R_exp = node_field(target, "index");
    R_continue = &&ev_assignment_index_done_label;
    goto *ev_dispatch;
ev_assignment_index_done_label: {
    double index = R_val->u.num;
    R_continue = pop_addr();
    Val *assigned = pop_val();
    Val *object = pop_val();
    if (object->tag != T_ARRAY || index < 0 || (size_t)index >= object->u.seq.len) {
        runtime_error("array index out of range");
    }
    object->u.seq.items[(size_t)index] = assigned;
    R_val = assigned;
    goto *R_continue;
}
}

ev_if_label:
    push_val(R_exp);
    push_addr(R_continue);
    R_exp = node_field(R_exp, "test");
    R_continue = &&ev_if_decide_label;
    goto *ev_dispatch;
ev_if_decide_label: {
    R_continue = pop_addr();
    Val *if_node = pop_val();
    Val *branch_value = is_false_value(R_val) ? record_get(if_node, "alternative")
                                              : node_field(if_node, "consequent");
    if (branch_value == V_UNDEF || branch_value->tag == T_NULL) {
        R_val = V_UNDEF;
        goto *R_continue;
    }
    R_exp = branch_value;
    goto *ev_dispatch;
}

ev_return_label:
    push_addr(R_continue);
    R_exp = node_field(R_exp, "argument");
    if (R_exp->tag == T_NULL || R_exp->tag == T_UNDEF) {
        R_continue = pop_addr();
        R_val = V_UNDEF;
        goto *ev_return_exit;
    }
    R_continue = &&ev_return_value_label;
    goto *ev_dispatch;
ev_return_value_label:
    R_continue = pop_addr();
    goto *ev_return_exit;

ev_return_exit_label:
    stack_unwind_to_marker();
    R_seq_end = pop_addr();
    R_env = env_of(pop_val());
    R_continue = pop_addr();
    goto *R_continue;

ev_while_label:
    push_addr(R_seq_end);
    push_addr(R_continue);
    push_val(R_exp);
    goto *ev_while_top;

ev_while_top_label:
    R_exp = node_field(stack_peek_val(), "test");
    R_continue = ev_while_decide;
    goto *ev_dispatch;

ev_while_decide_label: {
    if (!is_false_value(R_val)) {
        Val *body_stmt = node_field(stack_peek_val(), "body");
        if (record_tag_is(body_stmt, "block")) {
            R_unev = node_field(body_stmt, "body");
        } else {
            Val **one = checked_malloc(sizeof *one);
            one[0] = body_stmt;
            R_unev = make_seq(T_ARRAY, one, 1);
        }
        push_addr(R_seq_end);
        R_seq_end = ev_while_body_done;
        goto *ev_sequence;
    }
    (void)stack_pop();
    R_continue = pop_addr();
    R_seq_end = pop_addr();
    R_val = V_UNDEF;
    goto *R_continue;
}

ev_while_body_done_label:
    R_seq_end = pop_addr();
    goto *ev_while_top;

ev_sequence_label: {
    size_t len = R_unev->u.seq.len;
    if (len == 0) {
        R_val = V_UNDEF;
        goto *R_seq_end;
    }
    R_exp = R_unev->u.seq.items[0];
    if (len == 1) {
        goto *ev_sequence_last;
    }
    push_val(R_unev);
    push_val(env_word(R_env));
    push_addr(R_continue);
    R_continue = ev_sequence_continue;
    goto *ev_dispatch;
}

ev_sequence_continue_label:
    R_continue = pop_addr();
    R_env = env_of(pop_val());
    R_unev = pop_val();
    R_unev = make_seq(T_ARRAY, R_unev->u.seq.items + 1, R_unev->u.seq.len - 1);
    goto *ev_sequence;

ev_sequence_last_label:
    R_continue = R_seq_end;
    goto *ev_dispatch;

ev_block_done_label:
    R_seq_end = pop_addr();
    R_continue = pop_addr();
    goto *R_continue;

ev_appl_label:
    R_unev = node_field(R_exp, "args");
    R_exp = node_field(R_exp, "callee");
    push_addr(R_continue);
    push_val(env_word(R_env));
    push_val(R_unev);
    R_continue = ev_appl_did_operator;
    goto *ev_dispatch;

ev_appl_did_operator_label:
    R_unev = pop_val();
    R_env = env_of(pop_val());
    R_proc = R_val;
    R_argl = list_empty();
    if (R_unev->u.seq.len == 0) {
        goto *apply_dispatch;
    }
    push_val(R_proc);
    goto *ev_appl_operand_loop;

ev_appl_operand_loop_label:
    push_val(R_argl);
    R_exp = arg_expr(R_unev->u.seq.items[0]);
    if (R_unev->u.seq.len == 1) {
        R_continue = ev_appl_accum_last_arg;
        goto *ev_dispatch;
    }
    push_val(env_word(R_env));
    push_val(R_unev);
    R_continue = ev_appl_accumulate_arg;
    goto *ev_dispatch;

ev_appl_accumulate_arg_label:
    R_unev = pop_val();
    R_env = env_of(pop_val());
    R_argl = pop_val();
    R_argl = adjoin_arg(R_argl, R_val);
    R_unev = make_seq(T_ARRAY, R_unev->u.seq.items + 1, R_unev->u.seq.len - 1);
    goto *ev_appl_operand_loop;

ev_appl_accum_last_arg_label:
    R_argl = pop_val();
    R_argl = adjoin_arg(R_argl, R_val);
    R_proc = pop_val();
    goto *apply_dispatch;

apply_dispatch_label:
    if (is_primitive(R_proc)) {
        goto *ev_primitive_apply;
    }
    if (is_closure(R_proc)) {
        goto *ev_compound_apply;
    }
    runtime_error("applying a non-procedure");

ev_primitive_apply_label:
    R_val = apply_primitive(node_text(R_proc, "name"), R_argl);
    R_continue = pop_addr();
    goto *R_continue;

ev_compound_apply_label: {
    Val *params = node_field(R_proc, "params");
    Val *body = node_field(R_proc, "body");
    Env *closed = env_of(node_field(R_proc, "environment"));
    push_val(env_word(R_env));
    R_env = extend_environment(params, R_argl, closed);
    push_addr(R_seq_end);
    {
        StackSlot marker;
        marker.value = NULL;
        marker.address = NULL;
        marker.is_address = true;
        stack_push(marker);
    }
    R_seq_end = ev_return_exit;
    R_unev = node_field(body, "body");
    goto *ev_sequence;
}

ev_operator_label: {
    Val *node = R_exp;
    const char *kind = record_get(node, "tag")->u.text;
    if (strcmp(kind, "conditional") == 0) {
        push_val(node);
        push_addr(R_continue);
        R_exp = node_field(node, "test");
        R_continue = &&ev_operator_conditional_label;
        goto *ev_dispatch;
    ev_operator_conditional_label:
        R_continue = pop_addr();
        Val *conditional = pop_val();
        R_exp = is_false_value(R_val) ? node_field(conditional, "alternative")
                                      : node_field(conditional, "consequent");
        goto *ev_dispatch;
    }
    if (strcmp(kind, "unary") == 0) {
        push_val(node);
        push_addr(R_continue);
        R_exp = node_field(node, "operand");
        R_continue = &&ev_operator_unary_label;
        goto *ev_dispatch;
    ev_operator_unary_label: {
        R_continue = pop_addr();
        Val *unary = pop_val();
        const char *op_name = node_text(unary, "op");
        if (strcmp(op_name, "!") == 0) {
            R_val = make_bool(is_false_value(R_val));
        } else if (R_val->tag == T_NUM && strcmp(op_name, "-") == 0) {
            R_val = make_num(-R_val->u.num);
        } else if (R_val->tag == T_NUM && strcmp(op_name, "+") == 0) {
            R_val = make_num(R_val->u.num);
        } else {
            runtime_error("bad operand for a unary operator");
        }
        goto *R_continue;
    }
    }
    if (record_tag_is(node, "array") || record_tag_is(node, "object")) {
        push_val(node);
        push_addr(R_continue);
        R_unev = node_field(node, record_tag_is(node, "array") ? "elements" : "fields");
        R_argl = list_empty();
        goto ev_ctor_loop_label;
    }
    push_val(node);
    push_addr(R_continue);
    R_exp = node_field(node, "left");
    R_continue = &&ev_operator_left_done_label;
    goto *ev_dispatch;
ev_ctor_loop_label:
    if (R_unev->u.seq.len == 0) {
        goto ev_ctor_done_label;
    }
    push_val(R_argl);
    push_val(R_unev);
    push_val(env_word(R_env));
    push_addr(R_continue);
    R_exp = record_tag_is(node, "object")
                ? node_field(R_unev->u.seq.items[0], "value")
                : arg_expr(R_unev->u.seq.items[0]);
    R_continue = &&ev_ctor_accum_label;
    goto *ev_dispatch;
ev_ctor_accum_label:
    R_continue = pop_addr();
    R_env = env_of(pop_val());
    R_unev = pop_val();
    R_argl = pop_val();
    R_argl = adjoin_arg(R_argl, R_val);
    R_unev = make_seq(T_ARRAY, R_unev->u.seq.items + 1, R_unev->u.seq.len - 1);
    goto ev_ctor_loop_label;
ev_ctor_done_label: {
    R_continue = pop_addr();
    Val *ctor = pop_val();
    if (record_tag_is(ctor, "array")) {
        R_val = make_seq(T_ARRAY, R_argl->u.seq.items, R_argl->u.seq.len);
        goto *R_continue;
    }
    Val *fields = node_field(ctor, "fields");
    size_t len = fields->u.seq.len;
    const char **names = checked_malloc((len == 0 ? 1 : len) * sizeof *names);
    for (size_t i = 0; i < len; i += 1) {
        names[i] = node_text(fields->u.seq.items[i], "key");
    }
    R_val = make_record(names, R_argl->u.seq.items, len);
    goto *R_continue;
}
ev_operator_left_done_label: {
    R_continue = pop_addr();
    Val *binary = pop_val();
    const char *kind = record_get(binary, "tag")->u.text;
    const char *op_name = node_text(binary, "op");
    if (strcmp(kind, "logical") == 0 && strcmp(op_name, "&&") == 0 && is_false_value(R_val)) {
        R_val = V_FALSE;
        goto *R_continue;
    }
    if (strcmp(kind, "logical") == 0 && strcmp(op_name, "||") == 0 && !is_false_value(R_val)) {
        R_val = V_TRUE;
        goto *R_continue;
    }
    push_val(binary);
    push_val(R_val);
    push_addr(R_continue);
    R_exp = node_field(binary, "right");
    R_continue = &&ev_operator_right_done_label;
    goto *ev_dispatch;
ev_operator_right_done_label: {
    R_continue = pop_addr();
    Val *left = pop_val();
    Val *binary = pop_val();
    const char *op_name = node_text(binary, "op");
    (void)left;
    double a = left->tag == T_NUM ? left->u.num : 0;
    double b = R_val->tag == T_NUM ? R_val->u.num : 0;
    if (strcmp(op_name, "+") == 0 && left->tag == T_STR && R_val->tag == T_STR) {
        size_t length = strlen(left->u.text) + strlen(R_val->u.text) + 1;
        char *joined = checked_malloc(length);
        snprintf(joined, length, "%s%s", left->u.text, R_val->u.text);
        R_val = make_str(joined);
        goto *R_continue;
    }
    if (strcmp(op_name, "+") == 0) R_val = make_num(a + b);
    else if (strcmp(op_name, "-") == 0) R_val = make_num(a - b);
    else if (strcmp(op_name, "*") == 0) R_val = make_num(a * b);
    else if (strcmp(op_name, "/") == 0) R_val = make_num(a / b);
    else if (strcmp(op_name, "%") == 0) R_val = make_num(fmod(a, b));
    else if (strcmp(op_name, "<") == 0) R_val = make_bool(a < b);
    else if (strcmp(op_name, "<=") == 0) R_val = make_bool(a <= b);
    else if (strcmp(op_name, ">") == 0) R_val = make_bool(a > b);
    else if (strcmp(op_name, ">=") == 0) R_val = make_bool(a >= b);
    else if (strcmp(op_name, "===") == 0) {
        R_val = left->tag == T_NUM && R_val->tag == T_NUM ? make_bool(a == b)
               : left->tag == T_STR && R_val->tag == T_STR
                   ? make_bool(strcmp(left->u.text, R_val->u.text) == 0)
                   : make_bool(left == R_val);
    } else if (strcmp(op_name, "!==") == 0) {
        R_val = left->tag == T_NUM && R_val->tag == T_NUM ? make_bool(a != b)
               : left->tag == T_STR && R_val->tag == T_STR
                   ? make_bool(strcmp(left->u.text, R_val->u.text) != 0)
                   : make_bool(left != R_val);
    } else {
        runtime_error("unknown binary operator");
    }
    goto *R_continue;
}
}

ev_member_label: {
    push_val(R_exp);
    push_addr(R_continue);
    R_exp = node_field(R_exp, "object");
    R_continue = &&ev_member_object_done_label;
    goto *ev_dispatch;
ev_member_object_done_label: {
    R_continue = pop_addr();
    Val *access = pop_val();
    Val *object = R_val;
    if (record_tag_is(access, "member")) {
        if (object->tag != T_RECORD) {
            runtime_error("member access on a non-record");
        }
        R_val = record_get(object, node_text(access, "name"));
        goto *R_continue;
    }
    push_val(object);
    R_exp = node_field(access, "index");
    R_continue = &&ev_member_index_done_label;
    goto *ev_dispatch;
ev_member_index_done_label: {
    Val *object = pop_val();
    double index = R_val->u.num;
    if (object->tag == T_ARRAY || object->tag == T_LIST) {
        R_val = (index < 0 || (size_t)index >= object->u.seq.len)
                    ? V_UNDEF
                    : object->u.seq.items[(size_t)index];
        goto *R_continue;
    }
    if (object->tag == T_MAP) {
        for (size_t i = 0; i < object->u.mapv.len; i += 1) {
            Val *key = object->u.mapv.keys[i];
            if (key->tag == T_NUM && key->u.num == index) {
                R_val = object->u.mapv.vals[i];
                goto *R_continue;
            }
        }
        R_val = V_UNDEF;
        goto *R_continue;
    }
    runtime_error("index access on a non-sequence");
}
}
}
}
}
}

/* ------------------------------------------------------------------ */
/* The typed instruction stream: the exercise's C machine              */
/* ------------------------------------------------------------------ */

/* The translated controller also consumes the typed instruction stream
   the 5.5 compiler emits: MachineStatement records over the section 6
   exchange, with Source operation calls nesting recursively. Labels
   resolve to statement indices; save/restore use the same tagged stack;
   a goto-register target is the label address word a label input made.
   The operations are the canonical machine-operation table; a compound
   procedure's application is controller flow (its entry is a label), so
   applyProcedure reaches primitive procedures only. */

typedef struct {
    Val **by_name[128];
    const char *names[128];
    size_t len;
} RegisterFile;

static Val *machine_value_of(Val *wire) {
    if (wire->tag != T_RECORD) {
        return wire;
    }
    Val *tag = record_get(wire, "tag");
    const char *name = (tag->tag == T_SYM || tag->tag == T_STR) ? tag->u.text : "";
    if (strcmp(name, "symbol") == 0) {
        return make_sym(node_text(wire, "name"));
    }
    if (strcmp(name, "list") == 0 || strcmp(name, "array") == 0 || strcmp(name, "set") == 0) {
        Val *items = node_field(wire, "items");
        return make_seq(strcmp(name, "list") == 0 ? T_LIST : strcmp(name, "set") == 0 ? T_SET : T_ARRAY,
                        items->u.seq.items, items->u.seq.len);
    }
    if (strcmp(name, "record") == 0) {
        Val *fields = node_field(wire, "fields");
        size_t len = fields->u.seq.len;
        const char **names = checked_malloc((len == 0 ? 1 : len) * sizeof *names);
        Val **vals = checked_malloc((len == 0 ? 1 : len) * sizeof *vals);
        for (size_t i = 0; i < len; i += 1) {
            Val *entry = fields->u.seq.items[i];
            Val *key = entry->u.seq.items[0];
            if (key->tag != T_STR && key->tag != T_SYM) {
                die("a record field name must be a string");
            }
            names[i] = intern(key->u.text);
            vals[i] = machine_value_of(entry->u.seq.items[1]);
        }
        return make_record(names, vals, len);
    }
    if (strcmp(name, "map") == 0) {
        Val *entries = node_field(wire, "entries");
        size_t len = entries->u.seq.len;
        Val **keys = checked_malloc((len == 0 ? 1 : len) * sizeof *keys);
        Val **vals = checked_malloc((len == 0 ? 1 : len) * sizeof *vals);
        for (size_t i = 0; i < len; i += 1) {
            Val *entry = entries->u.seq.items[i];
            keys[i] = machine_value_of(entry->u.seq.items[0]);
            vals[i] = machine_value_of(entry->u.seq.items[1]);
        }
        return make_map(keys, vals, len);
    }
    return wire;
}

static Val *machine_op(const char *name, Val **args, size_t argc) {
    Val *argl = list_empty();
    for (size_t i = 0; i < argc; i += 1) {
        argl = adjoin_arg(argl, args[i]);
    }
    if (strcmp(name, "lookupVariableValue") == 0) {
        Env *env = env_of(args[1]);
        return lookup_variable(node_text(args[0], "name"), env);
    }
    if (strcmp(name, "setVariableValue") == 0) {
        set_variable(node_text(args[0], "name"), args[1], env_of(args[2]));
        return args[1];
    }
    if (strcmp(name, "defineVariableValue") == 0) {
        define_variable(node_text(args[0], "name"), args[1], env_of(args[2]));
        return args[1];
    }
    if (strcmp(name, "extendEnvironment") == 0) {
        Val *params = args[0];
        Val *values = args[1];
        return env_word(extend_environment(params, values, env_of(args[2])));
    }
    if (strcmp(name, "makeProcedure") == 0) {
        const char **names = checked_malloc(4 * sizeof *names);
        Val **vals = checked_malloc(4 * sizeof *vals);
        names[0] = intern("kind");
        vals[0] = make_sym("closure");
        names[1] = intern("params");
        vals[1] = args[0];
        names[2] = intern("body");
        vals[2] = args[1];
        names[3] = intern("environment");
        vals[3] = args[2];
        return make_record(names, vals, 4);
    }
    if (strcmp(name, "isPrimitiveProcedure") == 0) return make_bool(is_primitive(args[0]));
    if (strcmp(name, "isCompiledProcedure") == 0) {
        Val *kind = args[0]->tag == T_RECORD ? record_get(args[0], "kind") : V_UNDEF;
        return make_bool(kind->tag == T_SYM && strcmp(kind->u.text, "compiled-procedure") == 0);
    }
    if (strcmp(name, "applyProcedure") == 0) {
        if (!is_primitive(args[0])) {
            runtime_error("applyProcedure reached a non-primitive: compound apply is controller flow");
        }
        return apply_primitive(node_text(args[0], "name"), args[1]);
    }
    if (strcmp(name, "emptyArgList") == 0) return list_empty();
    if (strcmp(name, "adjoinArg") == 0) return adjoin_arg(args[1], args[0]);
    if (strcmp(name, "noArgs") == 0) return make_bool(args[0]->u.seq.len == 0);
    if (strcmp(name, "firstArg") == 0) return args[0]->u.seq.items[0];
    if (strcmp(name, "restArgs") == 0) {
        return make_seq(T_LIST, args[0]->u.seq.items + 1, args[0]->u.seq.len - 1);
    }
    if (strcmp(name, "isLastArg") == 0) return make_bool(args[0]->u.seq.len == 1);
    if (strcmp(name, "isTrue") == 0) return make_bool(!is_false_value(args[0]));
    if (strcmp(name, "isFalse") == 0) return make_bool(is_false_value(args[0]));
    if (strcmp(name, "print") == 0 || strcmp(name, "print-value") == 0) {
        print_guest_value(args[0]);
        return V_UNDEF;
    }
    if (strcmp(name, "makeLabel") == 0) return make_sym(node_text(args[0], "name"));
    {
        char message[256];
        snprintf(message, sizeof message, "unknown machine operation %s", name);
        runtime_error(message);
    }
    return V_UNDEF;
}

static Val *resolve_source(Val *source, RegisterFile *registers);

static Val *register_value(RegisterFile *registers, const char *name) {
    for (size_t i = 0; i < registers->len; i += 1) {
        if (strcmp(registers->names[i], name) == 0) {
            return *registers->by_name[i];
        }
    }
    runtime_error("unknown register in the machine stream");
    return V_UNDEF;
}

/* A Source is an input or an operation call whose arguments are Sources
   themselves: the recursion of the exchange contract. */
static Val *resolve_source(Val *source, RegisterFile *registers) {
    if (record_tag_is(source, "reg")) {
        return register_value(registers, node_text(source, "name"));
    }
    if (record_tag_is(source, "const")) {
        return machine_value_of(node_field(source, "value"));
    }
    if (record_tag_is(source, "label")) {
        return make_sym(node_text(source, "name"));
    }
    if (record_tag_is(source, "op")) {
        Val *args_wire = node_field(source, "args");
        if (args_wire->u.seq.len > 64) {
            runtime_error("too many operation arguments");
        }
        Val *args[64];
        for (size_t i = 0; i < args_wire->u.seq.len; i += 1) {
            args[i] = resolve_source(args_wire->u.seq.items[i], registers);
        }
        return machine_op(node_text(source, "operation"), args, args_wire->u.seq.len);
    }
    runtime_error("unknown source in the machine stream");
    return V_UNDEF;
}

static void run_machine_stream(Val *statements, Val *registers_wire) {
    size_t len = statements->u.seq.len;
    size_t labels_len = 0;
    const char *label_names[256];
    size_t label_index[256];
    for (size_t i = 0; i < len; i += 1) {
        Val *statement = statements->u.seq.items[i];
        if (record_tag_is(statement, "label")) {
            if (labels_len == 256) {
                runtime_error("too many labels in the machine stream");
            }
            label_names[labels_len] = node_text(statement, "name");
            label_index[labels_len] = i;
            labels_len += 1;
        }
    }
    RegisterFile registers;
    registers.len = 0;
    for (size_t i = 0; i < registers_wire->u.rec.len; i += 1) {
        registers.names[registers.len] = registers_wire->u.rec.names[i];
        registers.by_name[registers.len] = &registers_wire->u.rec.vals[i];
        *registers.by_name[registers.len] = machine_value_of(*registers.by_name[registers.len]);
        registers.len += 1;
    }
    bool flag = false;
    size_t pc = 0;
    size_t steps = 0;
    while (pc < len) {
        if (steps > 10000000) {
            runtime_error("out-of-steps");
        }
        steps += 1;
        Val *statement = statements->u.seq.items[pc];
        if (record_tag_is(statement, "label")) {
            pc += 1;
            continue;
        }
        if (record_tag_is(statement, "assign")) {
            Val *value = resolve_source(node_field(statement, "source"), &registers);
            const char *target = node_text(statement, "register");
            bool stored = false;
            for (size_t i = 0; i < registers.len; i += 1) {
                if (strcmp(registers.names[i], target) == 0) {
                    *registers.by_name[i] = value;
                    stored = true;
                }
            }
            if (!stored) {
                runtime_error("unknown register in the machine stream");
            }
            pc += 1;
            continue;
        }
        if (record_tag_is(statement, "test") || record_tag_is(statement, "perform")) {
            Val *args_wire = node_field(statement, "args");
            Val *args[64];
            if (args_wire->u.seq.len > 64) {
                runtime_error("too many operation arguments");
            }
            for (size_t i = 0; i < args_wire->u.seq.len; i += 1) {
                args[i] = resolve_source(args_wire->u.seq.items[i], &registers);
            }
            Val *result = machine_op(node_text(statement, "operation"), args, args_wire->u.seq.len);
            if (record_tag_is(statement, "test")) {
                flag = !is_false_value(result);
            }
            pc += 1;
            continue;
        }
        if (record_tag_is(statement, "branch")) {
            const char *label = node_text(statement, "label");
            if (!flag) {
                pc += 1;
                continue;
            }
            for (size_t i = 0; i < labels_len; i += 1) {
                if (strcmp(label_names[i], label) == 0) {
                    pc = label_index[i];
                    goto stream_continue;
                }
            }
            runtime_error("unknown label in the machine stream");
        }
        if (record_tag_is(statement, "goto-label")) {
            const char *label = node_text(statement, "label");
            for (size_t i = 0; i < labels_len; i += 1) {
                if (strcmp(label_names[i], label) == 0) {
                    pc = label_index[i];
                    goto stream_continue;
                }
            }
            runtime_error("unknown label in the machine stream");
        }
        if (record_tag_is(statement, "goto-register")) {
            const char *reg_name = node_text(statement, "register");
            Val *target = NULL;
            for (size_t i = 0; i < registers.len; i += 1) {
                if (strcmp(registers.names[i], reg_name) == 0) {
                    target = *registers.by_name[i];
                }
            }
            if (target == NULL || target->tag != T_SYM) {
                runtime_error("bad goto-register target in the machine stream");
            }
            for (size_t i = 0; i < labels_len; i += 1) {
                if (strcmp(label_names[i], target->u.text) == 0) {
                    pc = label_index[i];
                    goto stream_continue;
                }
            }
            runtime_error("bad goto-register target in the machine stream");
        }
        if (record_tag_is(statement, "save") || record_tag_is(statement, "restore")) {
            const char *reg_name = node_text(statement, "register");
            for (size_t i = 0; i < registers.len; i += 1) {
                if (strcmp(registers.names[i], reg_name) != 0) {
                    continue;
                }
                if (record_tag_is(statement, "save")) {
                    push_val(*registers.by_name[i]);
                } else {
                    *registers.by_name[i] = pop_val();
                }
                pc += 1;
                goto stream_continue;
            }
            runtime_error("unknown register in a save or restore");
        }
        runtime_error("unknown statement in the machine stream");
    stream_continue:;
    }
    for (size_t i = 0; i < registers.len; i += 1) {
        Val *value = *registers.by_name[i];
        if (value->tag == T_NUM) printf("=> %s = %.15g\n", registers.names[i], value->u.num);
        else if (value->tag == T_STR || value->tag == T_SYM) printf("=> %s = %s\n", registers.names[i], value->u.text);
        else if (value->tag == T_BOOL) printf("=> %s = %s\n", registers.names[i], value->u.boolean ? "true" : "false");
        else printf("=> %s = [value]\n", registers.names[i]);
    }
}

/* ------------------------------------------------------------------ */
/* The read-eval-print loop                                             */
/* ------------------------------------------------------------------ */

static char *read_file(const char *path) {
    FILE *file = fopen(path, "rb");
    if (file == NULL) {
        die("cannot open the input file");
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        die("cannot size the input file");
    }
    long length = ftell(file);
    if (length < 0) {
        fclose(file);
        die("cannot size the input file");
    }
    rewind(file);
    char *text = checked_malloc((size_t)length + 1);
    if (fread(text, 1, (size_t)length, file) != (size_t)length) {
        fclose(file);
        die("cannot read the input file");
    }
    fclose(file);
    text[length] = '\0';
    return text;
}

int main(int argc, char **argv) {
    V_UNDEF = new_value(T_UNDEF);
    V_NULL = new_value(T_NULL);
    V_TRUE = new_value(T_BOOL);
    V_TRUE->u.boolean = true;
    V_FALSE = new_value(T_BOOL);
    V_FALSE->u.boolean = false;
    if (argc != 2) {
        fprintf(stderr, "usage: eceval <typed-data.json>\n");
        return EXIT_FAILURE;
    }
    Reader reader;
    reader.text = read_file(argv[1]);
    reader.position = 0;
    Val *input = read_json(&reader);
    Val *kind = record_get(input, "kind");
    if (kind->tag != T_STR && kind->tag != T_SYM) {
        die("input needs a kind field: program or machine");
    }
    global_environment = make_env(list_empty(), list_empty(), NULL);
    define_variable("undefined", V_UNDEF, global_environment);
    define_variable("null", V_NULL, global_environment);
    const char *console_names[] = { intern("log") };
    Val *console_values[] = { make_primitive("print-value") };
    define_variable("console", make_record(console_names, console_values, 1), global_environment);
    if (strcmp(kind->u.text, "machine") == 0) {
        run_machine_stream(node_field(input, "statements"), node_field(input, "registers"));
        return EXIT_SUCCESS;
    }
    if (strcmp(kind->u.text, "program") == 0) {
        R_forms = node_field(input, "forms");
        R_form_index = 0;
        R_env = global_environment;
        R_seq_end = NULL;
        eval_controller();
        return EXIT_SUCCESS;
    }
    die("input needs a kind field: program or machine");
    return EXIT_FAILURE;
}
