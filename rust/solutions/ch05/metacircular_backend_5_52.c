/* SPDX-License-Identifier: GPL-3.0-only */
/* Exercise 5.52 run-time support: the object world the compiled
   metacircular evaluator runs in. Registers are globals, `continue`
   holds a label address (the GNU computed-goto extension), and the
   machine stack carries values and saved addresses. Environments are
   association chains; the empty parent arrives as the `the-empty`
   symbol, the machine's own marker. The generated code (constant
   globals, the initializer, and the compiled forms) is appended after
   this file. Numbers are long longs; the session stays far below the
   width, and division mirrors the host: an exact quotient stays
   integral, otherwise it is a real. */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum { T_INT, T_REAL, T_BOOL, T_SYM, T_STR, T_NIL, T_PAIR, T_PRIM, T_COMP, T_ENV } Tag;

typedef struct Val Val;
typedef struct Env Env;

struct Val {
    Tag tag;
    union {
        long long num;
        double real;
        int boolean;
        const char *sym;
        const char *str;
        struct { Val *car, *cdr; } pair;
        const char *prim;
        struct { void *entry; Val *env; } comp;
        Env *env;
    } u;
};

struct Env {
    Val *vars;
    Val *vals;
    Env *parent;
};

static Val *R_exp, *R_env, *R_val, *R_proc, *R_argl, *R_unev;
static void *R_continue, *entry_addr;
static int flag;

static Val *V_NIL, *V_TRUE, *V_FALSE;
static Env *global_env;

typedef struct { int is_addr; Val *v; void *a; } Slot;
static Slot *stack;
static size_t stack_cap, sp;

static void die(const char *msg) {
    fprintf(stderr, "eceval: %s\n", msg);
    exit(1);
}

static void *xmalloc(size_t n) {
    void *p = malloc(n);
    if (p == NULL) {
        die("out of memory");
    }
    return p;
}

static Val *new_val(Tag tag) {
    Val *v = xmalloc(sizeof *v);
    v->tag = tag;
    return v;
}

static Val *mk_int(long long n) {
    Val *v = new_val(T_INT);
    v->u.num = n;
    return v;
}

static Val *mk_real(double x) {
    Val *v = new_val(T_REAL);
    v->u.real = x;
    return v;
}

static Val *mk_bool(int b) {
    return b ? V_TRUE : V_FALSE;
}
static Val *mk_str(const char *text) {
    size_t n = strlen(text) + 1;
    char *copy = xmalloc(n);
    memcpy(copy, text, n);
    Val *v = new_val(T_STR);
    v->u.str = copy;
    return v;
}
typedef struct SymNode { const char *name; struct SymNode *next; } SymNode;
static SymNode *syms;
static const char *intern(const char *name) {
    for (SymNode *s = syms; s != NULL; s = s->next) {
        if (strcmp(s->name, name) == 0) {
            return s->name;
        }
    }
    size_t n = strlen(name) + 1;
    char *copy = xmalloc(n);
    memcpy(copy, name, n);
    SymNode *node = xmalloc(sizeof *node);
    node->name = copy;
    node->next = syms;
    syms = node;
    return copy;
}

static Val *mk_sym(const char *name) {
    Val *v = new_val(T_SYM);
    v->u.sym = intern(name);
    return v;
}


static Val *cons(Val *a, Val *b) {
    Val *v = new_val(T_PAIR);
    v->u.pair.car = a;
    v->u.pair.cdr = b;
    return v;
}

static Val *mk_prim(const char *name) {
    Val *v = new_val(T_PRIM);
    v->u.prim = intern(name);
    return v;
}

static Val *mk_compiled(void *entry, Val *env) {
    Val *v = new_val(T_COMP);
    v->u.comp.entry = entry;
    v->u.comp.env = env;
    return v;
}

static Val *mk_env(Env *e) {
    Val *v = new_val(T_ENV);
    v->u.env = e;
    return v;
}

static int is_true(Val *v) {
    return v != V_FALSE;
}

static void push_val(Val *v) {
    if (sp == stack_cap) {
        die("stack overflow");
    }
    stack[sp].is_addr = 0;
    stack[sp].v = v;
    sp++;
}

static void push_addr(void *a) {
    if (sp == stack_cap) {
        die("stack overflow");
    }
    stack[sp].is_addr = 1;
    stack[sp].a = a;
    sp++;
}

static Val *pop_val(void) {
    if (sp == 0) {
        die("stack underflow");
    }
    sp--;
    if (stack[sp].is_addr) {
        die("stack type mismatch");
    }
    return stack[sp].v;
}

static void *pop_addr(void) {
    if (sp == 0) {
        die("stack underflow");
    }
    sp--;
    if (!stack[sp].is_addr) {
        die("stack type mismatch");
    }
    return stack[sp].a;
}

static Env *env_of(Val *v, const char *who) {
    if (v->tag == T_ENV) {
        return v->u.env;
    }
    if (v->tag == T_SYM && strcmp(v->u.sym, "the-empty") == 0) {
        return global_env;
    }
    {
        char buf[160];
        int n = snprintf(buf, sizeof buf, "environment operation %s on tag %d", who, (int)v->tag);
        if (n < 0 || (size_t)n >= sizeof buf) {
            die("environment operation on a non-environment");
        }
        die(buf);
    }
    return NULL;
}

static Val *env_lookup(Val *var, Env *e) {
    const char *name = var->u.sym;
    for (Env *f = e; f != NULL; f = f->parent) {
        Val *vars = f->vars, *vals = f->vals;
        while (vars->tag == T_PAIR) {
            Val *one = vars->u.pair.car;
            if (one->tag == T_SYM && strcmp(one->u.sym, name) == 0) {
                return vals->u.pair.car;
            }
            vars = vars->u.pair.cdr;
            vals = vals->u.pair.cdr;
        }
    }
    die("unbound variable");
    return NULL;
}

static void env_define(Val *var, Val *val, Env *e) {
    e->vars = cons(var, e->vars);
    e->vals = cons(val, e->vals);
}

static void env_set(Val *var, Val *val, Env *e) {
    const char *name = var->u.sym;
    for (Env *f = e; f != NULL; f = f->parent) {
        Val *vars = f->vars, *vals = f->vals;
        while (vars->tag == T_PAIR) {
            Val *one = vars->u.pair.car;
            if (one->tag == T_SYM && strcmp(one->u.sym, name) == 0) {
                vals->u.pair.car = val;
                return;
            }
            vars = vars->u.pair.cdr;
            vals = vals->u.pair.cdr;
        }
    }
    die("unbound variable in set");
}

static long long as_int(Val *v, const char *who) {
    if (v->tag != T_INT) {
        (void)who;
        die("arithmetic on a non-integer");
    }
    return v->u.num;
}

/* The primitive table: the object set plus the names the compiled
   metacircular calls. Arguments arrive as a Scheme list. */
static Val *apply_prim(const char *name, Val *argl);

static Val *list_args(Val *argl) {
    return argl;
}

static size_t arg_count(Val *argl) {
    size_t n = 0;
    while (argl->tag == T_PAIR) {
        n++;
        argl = argl->u.pair.cdr;
    }
    return n;
}

static Val *arg_at(Val *argl, size_t k) {
    while (k-- > 0) {
        argl = argl->u.pair.cdr;
    }
    return argl->u.pair.car;
}

static int structurally_equal(Val *a, Val *b) {
    if (a->tag != b->tag) {
        return 0;
    }
    switch (a->tag) {
    case T_INT:
        return a->u.num == b->u.num;
    case T_REAL:
        return a->u.real == b->u.real;
    case T_BOOL:
        return a->u.boolean == b->u.boolean;
    case T_SYM:
        return strcmp(a->u.sym, b->u.sym) == 0;
    case T_STR:
        return strcmp(a->u.str, b->u.str) == 0;
    case T_NIL:
        return 1;
    case T_PAIR:
        return structurally_equal(a->u.pair.car, b->u.pair.car)
            && structurally_equal(a->u.pair.cdr, b->u.pair.cdr);
    case T_PRIM:
        return strcmp(a->u.prim, b->u.prim) == 0;
    default:
        return a == b;
    }
}

static void print_val(Val *v) {
    switch (v->tag) {
    case T_INT:
        printf("%lld", v->u.num);
        break;
    case T_REAL:
        printf("%g", v->u.real);
        break;
    case T_BOOL:
        fputs(v == V_FALSE ? "#f" : "#t", stdout);
        break;
    case T_SYM:
        fputs(v->u.sym, stdout);
        break;
    case T_STR:
        putchar('"');
        fputs(v->u.str, stdout);
        putchar('"');
        break;
    case T_NIL:
        fputs("()", stdout);
        break;
    case T_PAIR: {
        putchar('(');
        Val *cursor = v;
        int first = 1;
        while (cursor->tag == T_PAIR) {
            if (!first) {
                putchar(' ');
            }
            first = 0;
            print_val(cursor->u.pair.car);
            cursor = cursor->u.pair.cdr;
        }
        if (cursor->tag != T_NIL) {
            fputs(" . ", stdout);
            print_val(cursor);
        }
        putchar(')');
        break;
    }
    case T_PRIM:
        printf("#[primitive-procedure %s]", v->u.prim);
        break;
    case T_COMP:
        fputs("#[compiled-procedure]", stdout);
        break;
    case T_ENV:
        fputs("#[environment]", stdout);
        break;
    }
}

static void emit_newline(void) {
    putchar('\n');
}

static void user_print(Val *v) {
    print_val(v);
}

static Val *apply_prim(const char *name, Val *argl) {
    if (strcmp(name, "cons") == 0) {
        return cons(arg_at(argl, 0), arg_at(argl, 1));
    }
    if (strcmp(name, "car") == 0) {
        return arg_at(argl, 0)->u.pair.car;
    }
    if (strcmp(name, "cdr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr;
    }
    if (strcmp(name, "list") == 0) {
        return list_args(argl);
    }
    if (strcmp(name, "null?") == 0) {
        return mk_bool(arg_at(argl, 0)->tag == T_NIL);
    }
    if (strcmp(name, "pair?") == 0) {
        return mk_bool(arg_at(argl, 0)->tag == T_PAIR);
    }
    if (strcmp(name, "symbol?") == 0) {
        return mk_bool(arg_at(argl, 0)->tag == T_SYM);
    }
    if (strcmp(name, "number?") == 0) {
        Val *a = arg_at(argl, 0);
        return mk_bool(a->tag == T_INT || a->tag == T_REAL);
    }
    if (strcmp(name, "string?") == 0) {
        return mk_bool(0);
    }
    if (strcmp(name, "not") == 0) {
        return mk_bool(!is_true(arg_at(argl, 0)));
    }
    if (strcmp(name, "eq?") == 0) {
        Val *a = arg_at(argl, 0), *b = arg_at(argl, 1);
        int same;
        if (a->tag == T_PAIR && b->tag == T_PAIR) {
            same = (a == b);
        } else {
            same = structurally_equal(a, b);
        }
        return mk_bool(same);
    }
    if (strcmp(name, "equal?") == 0) {
        return mk_bool(structurally_equal(arg_at(argl, 0), arg_at(argl, 1)));
    }
    if (strcmp(name, "+") == 0) {
        long long acc = 0;
        for (Val *r = argl; r->tag == T_PAIR; r = r->u.pair.cdr) {
            acc += as_int(r->u.pair.car, "+");
        }
        return mk_int(acc);
    }
    if (strcmp(name, "*") == 0) {
        long long acc = 1;
        for (Val *r = argl; r->tag == T_PAIR; r = r->u.pair.cdr) {
            acc *= as_int(r->u.pair.car, "*");
        }
        return mk_int(acc);
    }
    if (strcmp(name, "-") == 0) {
        if (arg_count(argl) == 1) {
            return mk_int(-as_int(arg_at(argl, 0), "-"));
        }
        long long acc = as_int(arg_at(argl, 0), "-");
        for (size_t k = 1; k < arg_count(argl); k++) {
            acc -= as_int(arg_at(argl, k), "-");
        }
        return mk_int(acc);
    }
    if (strcmp(name, "/") == 0) {
        long long acc = as_int(arg_at(argl, 0), "/");
        for (size_t k = 1; k < arg_count(argl); k++) {
            long long d = as_int(arg_at(argl, k), "/");
            if (d == 0) {
                die("division by zero");
            }
            if (acc % d == 0) {
                acc /= d;
            } else {
                return mk_real((double)acc / (double)d);
            }
        }
        return mk_int(acc);
    }
    if (strcmp(name, "=") == 0 || strcmp(name, "<") == 0 || strcmp(name, ">") == 0
        || strcmp(name, "<=") == 0 || strcmp(name, ">=") == 0) {
        long long a = as_int(arg_at(argl, 0), name);
        long long b = as_int(arg_at(argl, 1), name);
        int hit = (name[0] == '=' && a == b) || (name[0] == '<' && (name[1] == '\0' ? a < b : a <= b))
            || (name[0] == '>' && (name[1] == '\0' ? a > b : a >= b));
        return mk_bool(hit);
    }
    if (strcmp(name, "remainder") == 0) {
        long long a = as_int(arg_at(argl, 0), name);
        long long b = as_int(arg_at(argl, 1), name);
        if (b == 0) {
            die("division by zero");
        }
        return mk_int(a % b);
    }
    if (strcmp(name, "quotient") == 0) {
        long long a = as_int(arg_at(argl, 0), name);
        long long b = as_int(arg_at(argl, 1), name);
        if (b == 0) {
            die("division by zero");
        }
        return mk_int(a / b);
    }
    if (strcmp(name, "abs") == 0) {
        long long a = as_int(arg_at(argl, 0), name);
        return mk_int(a < 0 ? -a : a);
    }
    if (strcmp(name, "cadr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.car;
    }
    if (strcmp(name, "caddr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.cdr->u.pair.car;
    }
    if (strcmp(name, "cadddr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.cdr->u.pair.cdr->u.pair.car;
    }
    if (strcmp(name, "caadr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.car->u.pair.car;
    }
    if (strcmp(name, "cddr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.cdr;
    }
    if (strcmp(name, "cdddr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.cdr->u.pair.cdr;
    }
    if (strcmp(name, "cdadr") == 0) {
        return arg_at(argl, 0)->u.pair.cdr->u.pair.car->u.pair.cdr;
    }
    if (strcmp(name, "display") == 0) {
        print_val(arg_at(argl, 0));
        return mk_sym("ok");
    }
    if (strcmp(name, "newline") == 0) {
        emit_newline();
        return mk_sym("ok");
    }
    if (strcmp(name, "error") == 0) {
        fputs("error: ", stdout);
        print_val(arg_at(argl, 0));
        emit_newline();
        exit(1);
    }
    if (strcmp(name, "extend-environment") == 0) {
        Env *e = xmalloc(sizeof *e);
        e->vars = arg_at(argl, 0);
        e->vals = arg_at(argl, 1);
        e->parent = env_of(arg_at(argl, 2), name);
        return mk_env(e);
    }
    if (strcmp(name, "lookup-variable-value") == 0) {
        return env_lookup(arg_at(argl, 0), env_of(arg_at(argl, 1), name));
    }
    if (strcmp(name, "set-variable-value!") == 0) {
        env_set(arg_at(argl, 0), arg_at(argl, 1), env_of(arg_at(argl, 2), name));
        return mk_sym("ok");
    }
    if (strcmp(name, "define-variable!") == 0) {
        env_define(arg_at(argl, 0), arg_at(argl, 1), env_of(arg_at(argl, 2), name));
        return mk_sym("ok");
    }
    if (strcmp(name, "apply-in-underlying-scheme") == 0) {
        Val *proc = arg_at(argl, 0);
        if (proc->tag != T_PRIM) {
            die("apply-in-underlying-scheme needs a primitive");
        }
        return apply_prim(proc->u.prim, arg_at(argl, 1));
    }
    {
        char buf[128];
        int n = snprintf(buf, sizeof buf, "unknown primitive: %s", name);
        if (n < 0 || (size_t)n >= sizeof buf) {
            die("unknown primitive");
        }
        die(buf);
    }
    return NULL;
}
static Val *op_list(Val *v) { return cons(v, V_NIL); }
/* The machine operations the compiled code names. */
static Val *op_cons(Val *a, Val *b) { return cons(a, b); }
static Val *op_false_p(Val *v) { return mk_bool(v == V_FALSE); }
static Val *op_primitive_procedure_p(Val *v) { return mk_bool(v->tag == T_PRIM); }
static Val *op_lookup_variable_value(Val *var, Val *env) {
    return env_lookup(var, env_of(env, "lookup"));
}
static Val *op_extend_environment(Val *vars, Val *vals, Val *env) {
    Env *e = xmalloc(sizeof *e);
    e->vars = vars;
    e->vals = vals;
    e->parent = env_of(env, "extend");
    return mk_env(e);
}
static Val *op_define_variable_x(Val *var, Val *val, Val *env) {
    env_define(var, val, env_of(env, "define"));
    return mk_sym("ok");
}
static Val *op_apply_primitive_procedure(Val *proc, Val *argl) {
    if (proc->tag != T_PRIM) {
        die("apply of a non-primitive");
    }
    return apply_prim(proc->u.prim, argl);
}
static void *compiled_entry(Val *proc) {
    if (proc->tag != T_COMP) {
        die("entry of a non-compiled procedure");
    }
    return proc->u.comp.entry;
}
static Val *op_compiled_procedure_env(Val *proc) {
    if (proc->tag != T_COMP) {
        die("environment of a non-compiled procedure");
    }
    return proc->u.comp.env;
}

static const char *const prim_names[] = {
    "cons", "car", "cdr", "null?", "pair?", "symbol?", "number?", "string?",
    "not", "eq?", "equal?", "list", "+", "-", "*", "/", "=", "<", ">",
    "remainder", "cadr", "caddr", "cadddr", "caadr", "cddr", "cdddr", "cdadr",
    "quotient", "abs", "<=", ">=", "display", "newline", "error",
    "extend-environment", "lookup-variable-value", "set-variable-value!",
    "define-variable!", "apply-in-underlying-scheme",
};

static void init_symbols(void) {
    stack_cap = 1u << 20;
    stack = xmalloc(stack_cap * sizeof *stack);
    sp = 0;
}

static void init_global(void) {
    V_NIL = new_val(T_NIL);
    V_TRUE = new_val(T_BOOL);
    V_TRUE->u.boolean = 1;
    V_FALSE = new_val(T_BOOL);
    V_FALSE->u.boolean = 0;
    global_env = xmalloc(sizeof *global_env);
    global_env->vars = V_NIL;
    global_env->vals = V_NIL;
    global_env->parent = NULL;
    env_define(mk_sym("true"), V_TRUE, global_env);
    env_define(mk_sym("false"), V_FALSE, global_env);
    for (size_t k = 0; k < sizeof prim_names / sizeof prim_names[0]; k++) {
        env_define(mk_sym(prim_names[k]), mk_prim(prim_names[k]), global_env);
    }
    R_env = mk_env(global_env);
}
