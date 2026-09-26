// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.51: the explicit-control evaluator translated
// into C. The registers are the machine struct's fields, every `ev-`
// entry point of 5.4.1 to 5.4.4 is a C label in one `eceval` function,
// `(assign continue (label l))` stores a label address, `(goto (reg
// continue))` is a computed goto, and the stack is an array with the
// same push/pop discipline. The object world, pairs, symbols,
// environment frames, and the primitive table, is the run-time support
// the exercise says must be provided. `translatedEvaluatorRuns`
// writes the translation, builds it with the system C compiler, runs
// the book's factorial session, and answers the output lines.

package sicp.ch5.solutions

import java.io.File
import java.nio.file.Files
import java.util.concurrent.TimeUnit

/** The C translation of the 5.4 explicit-control evaluator. */
private val ecevalC: String =
    """
/* The explicit-control evaluator of SICP 5.4, translated to C.
   The registers of 5.4.4; the ev- entry points are labels of eceval. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Value Value;
typedef struct Env Env;
struct Value {
    int tag;              /* 0 int, 1 symbol, 2 pair, 3 primitive, 4 compound, 5 bool, 6 nil */
    long i;               /* int payload; primitive index; bool */
    char *sym;
    Value *car, *cdr;
    Value *params, *body; /* compound: parameter and body lists */
    Env *env;             /* compound: definition environment */
};
struct Env { Value *names, *values; Env *outer; };

static Value TRUE_V, FALSE_V, NIL_V;
static Value *boolean(int b) { return b ? &TRUE_V : &FALSE_V; }
static Value *sym(const char *s) { Value *v = calloc(1, sizeof *v); v->tag = 1; v->sym = strdup(s); return v; }
static Value *num(long n) { Value *v = calloc(1, sizeof *v); v->tag = 0; v->i = n; return v; }
static Value *pair(Value *a, Value *d) { Value *v = calloc(1, sizeof *v); v->tag = 2; v->car = a; v->cdr = d; return v; }
static Value *NIL(void) { return &NIL_V; }
/* adjoin-arg of 5.4.3: append the new argument at the end of argl. */
static Value *adjoin_arg(Value *arg, Value *argl) {
    Value *cell = pair(arg, NIL());
    if (argl->tag != 2) return cell;
    Value *t = argl;
    while (t->cdr->tag == 2) t = t->cdr;
    t->cdr = cell;
    return argl;
}
static int is_true(Value *v) { return !(v->tag == 5 && v->i == 0); }
static int eqv(Value *a, Value *b) {
    if (a == b) return 1;
    if (a->tag != b->tag) return 0;
    if (a->tag == 0 || a->tag == 3) return a->i == b->i;
    if (a->tag == 1) return strcmp(a->sym, b->sym) == 0;
    return 0;
}
static int val_equal(Value *a, Value *b) {
    if (eqv(a, b)) return 1;
    if (a->tag == 2 && b->tag == 2)
        return val_equal(a->car, b->car) && val_equal(a->cdr, b->cdr);
    return 0;
}

static const char *prim_names[] = {
    "cons","car","cdr","null?","pair?","symbol?","number?","eq?","equal?",
    "+","-","*","=","<",">","remainder","not","list","display","newline",
    "cadr","caddr","cddr","caadr","cdadr","cdddr","error"
};
#define NPRIMS (sizeof(prim_names)/sizeof(*prim_names))

static void print_value(Value *v);

static Value *prim_apply(long idx, Value *args) {
    const char *n = prim_names[idx];
    if (!strcmp(n, "cons")) return pair(args->car, args->cdr->car);
    if (!strcmp(n, "car")) return args->car->car;
    if (!strcmp(n, "cdr")) return args->car->cdr;
    if (!strcmp(n, "null?")) return boolean(args->car->tag == 6);
    if (!strcmp(n, "pair?")) return boolean(args->car->tag == 2);
    if (!strcmp(n, "symbol?")) return boolean(args->car->tag == 1);
    if (!strcmp(n, "number?")) return boolean(args->car->tag == 0);
    if (!strcmp(n, "eq?")) return boolean(eqv(args->car, args->cdr->car));
    if (!strcmp(n, "equal?")) return boolean(val_equal(args->car, args->cdr->car));
    if (!strcmp(n, "not")) return boolean(!is_true(args->car));
    if (!strcmp(n, "+") || !strcmp(n, "-") || !strcmp(n, "*")) {
        long a = args->car->i, b = args->cdr->car->i;
        return num(n[0] == '+' ? a + b : n[0] == '-' ? a - b : a * b);
    }
    if (!strcmp(n, "=")) return boolean(args->car->i == args->cdr->car->i);
    if (!strcmp(n, "<")) return boolean(args->car->i < args->cdr->car->i);
    if (!strcmp(n, ">")) return boolean(args->car->i > args->cdr->car->i);
    if (!strcmp(n, "remainder")) return num(args->car->i % args->cdr->car->i);
    if (!strcmp(n, "list")) return args;
    if (!strcmp(n, "display")) { print_value(args->car); return args->car; }
    if (!strcmp(n, "newline")) { printf("\n"); return NIL(); }
    if (!strcmp(n, "cadr")) return args->car->cdr->car;
    if (!strcmp(n, "caddr")) return args->car->cdr->cdr->car;
    if (!strcmp(n, "cddr")) return args->car->cdr->cdr;
    if (!strcmp(n, "caadr")) return args->car->cdr->car->car;
    if (!strcmp(n, "cdadr")) return args->car->cdr->car->cdr;
    if (!strcmp(n, "cdddr")) return args->car->cdr->cdr->cdr;
    if (!strcmp(n, "error")) { fprintf(stderr, "error\n"); exit(2); }
    return sym("unimplemented");
}

/* The registers of 5.4.4. */
static Env *R_env;
static Value *R_exp, *R_val, *R_proc, *R_argl, *R_unev;
static void *R_continue;
static int R_flag;
static int esp = 0, emax = 0, epushes = 0;

/* The monitored stack: register words.  Continue holds a label
   address; every save is a push of one void *, every restore a pop. */
static void *estack[200000];
static void spush(void *v) { estack[esp++] = v; epushes++; if (esp > emax) emax = esp; }
static void spush_label(void *l) { spush(l); }
static void *spop(void) { return estack[--esp]; }

static Value *lookup(Value *name, Env *env) {
    for (Env *e = env; e; e = e->outer) {
        Value *n = e->names, *v = e->values;
        while (n && n->tag == 2) {
            if (eqv(n->car, name)) return v->car;
            n = n->cdr; v = v->cdr;
        }
    }
    fprintf(stderr, "unbound variable\n"); exit(3);
}
static void define_var(Value *name, Value *val, Env *env) {
    Value *n = env->names, *v = env->values;
    while (n && n->tag == 2) {
        if (eqv(n->car, name)) { v->car = val; return; }
        n = n->cdr; v = v->cdr;
    }
    env->names = pair(name, env->names);
    env->values = pair(val, env->values);
}
static void set_var(Value *name, Value *val, Env *env) {
    for (Env *e = env; e; e = e->outer) {
        Value *n = e->names, *v = e->values;
        while (n && n->tag == 2) {
            if (eqv(n->car, name)) { v->car = val; return; }
            n = n->cdr; v = v->cdr;
        }
    }
}
static Env *extend(Value *params, Value *args, Env *base) {
    Env *e = calloc(1, sizeof *e);
    e->names = params; e->values = args; e->outer = base;
    return e;
}

/* The driver's input queue: the program file named by argv[1]. */
static char *program_text;
static const char *cursor;

static Value *read_form(void);
static Value *read_list(void) {
    while (*cursor == ' ' || *cursor == '\n' || *cursor == '\t') cursor++;
    if (!*cursor) return NULL;
    if (*cursor == ')') { cursor++; return NIL(); }
    Value *first = read_form();
    return pair(first, read_list());
}
static Value *read_form(void) {
    while (*cursor == ' ' || *cursor == '\n' || *cursor == '\t' || *cursor == ';') {
        if (*cursor == ';') while (*cursor && *cursor != '\n') cursor++; else cursor++;
    }
    if (!*cursor) return NULL;
    if (*cursor == '(') { cursor++; return read_list(); }
    if (*cursor == '\'') { cursor++; return pair(sym("quote"), pair(read_form(), NIL())); }
    char buf[256]; int i = 0;
    while (*cursor && !strchr(" \n\t();", *cursor)) buf[i++] = *cursor++;
    buf[i] = 0;
    if (buf[0] >= '0' && buf[0] <= '9') return num(atol(buf));
    return sym(buf);
}

static void print_value(Value *v) {
    if (!v) { printf("()"); return; }
    switch (v->tag) {
    case 0: printf("%ld", v->i); break;
    case 1: printf("%s", v->sym); break;
    case 6: printf("()"); break;
    case 5: printf(v->i ? "#t" : "#f"); break;
    case 3: printf("#[primitive]"); break;
    case 4: printf("#[compound-procedure]"); break;
    case 2: {
        printf("(");
        print_value(v->car);
        for (Value *d = v->cdr; d && d->tag == 2; d = d->cdr) { printf(" "); print_value(d->car); }
        printf(")");
        break;
    }
    }
}

/* eceval: the controller.  Every label of the book's text is here. */
static void eceval(void) {
    void *dispose_stack[200000]; int dsp = 0; /* per-entry saved continue */

    /* initialize the machine: the global environment and primitives */
    static Env global_env;
    global_env.names = NIL(); global_env.values = NIL(); global_env.outer = NULL;
    R_env = &global_env;
    for (long i = 0; i < (long)NPRIMS; i++) {
        Value *p = calloc(1, sizeof *p); p->tag = 3; p->i = i;
        define_var(sym(prim_names[i]), p, R_env);
    }
    define_var(sym("true"), boolean(1), R_env);
    define_var(sym("false"), boolean(0), R_env);

read_eval_print_loop:
    R_exp = read_form();
    if (!R_exp) goto halt;
    R_env = &global_env;
    R_continue = &&print_result;
    goto eval_dispatch;

print_result:
    print_value(R_val);
    printf("\n");
    goto read_eval_print_loop;

halt:
    return;

eval_dispatch:
    if (R_exp->tag == 0 || R_exp->tag == 5 || R_exp->tag == 6) goto ev_self_eval;
    if (R_exp->tag == 1) goto ev_variable;
    if (R_exp->tag == 2 && R_exp->car->tag == 1) {
        const char *h = R_exp->car->sym;
        if (!strcmp(h, "quote")) goto ev_quoted;
        if (!strcmp(h, "set!")) goto ev_assignment;
        if (!strcmp(h, "define")) goto ev_definition;
        if (!strcmp(h, "if")) goto ev_if;
        if (!strcmp(h, "begin")) goto ev_begin;
        if (!strcmp(h, "lambda")) goto ev_lambda;
    }
    if (R_exp->tag == 2) goto ev_application;
    goto unknown_expression_type;

ev_self_eval:
    R_val = R_exp;
    goto *R_continue;

ev_variable:
    R_val = lookup(R_exp, R_env);
    goto *R_continue;

ev_quoted:
    R_val = R_exp->cdr->car;
    goto *R_continue;

ev_lambda:
    R_val = calloc(1, sizeof *R_val);
    R_val->tag = 4;                 /* compound procedure */
    R_val->params = R_exp->cdr->car;
    R_val->body = R_exp->cdr->cdr;
    R_val->env = R_env;
    goto *R_continue;

ev_application:
    spush_label(R_continue);
    spush(R_env);
    R_unev = R_exp->cdr;
    spush(R_unev);
    R_exp = R_exp->car;
    R_continue = &&ev_appl_did_operator;
    goto eval_dispatch;

ev_appl_did_operator:
    R_unev = spop();
    R_env = spop();
    R_argl = NIL();
    R_proc = R_val;
    if (R_unev->tag != 2) { goto apply_dispatch; }
    spush(R_proc);
    goto ev_appl_operand_loop;

ev_appl_operand_loop:
    spush(R_argl);
    R_exp = R_unev->car;
    if (R_unev->cdr->tag != 2) goto ev_appl_last_arg;
    spush(R_env);
    spush(R_unev);
    R_continue = &&ev_appl_accumulate_arg;
    goto eval_dispatch;

ev_appl_accumulate_arg:
    R_unev = spop();
    R_env = spop();
    R_argl = spop();
    R_argl = adjoin_arg(R_val, R_argl);
    R_unev = R_unev->cdr;
    goto ev_appl_operand_loop;

ev_appl_last_arg:
    R_continue = &&ev_appl_accum_last_arg;
    goto eval_dispatch;

ev_appl_accum_last_arg:
    R_argl = spop();
    R_argl = adjoin_arg(R_val, R_argl);
    R_proc = spop();
    goto apply_dispatch;

apply_dispatch:
    if (R_proc->tag == 3) goto primitive_apply;
    if (R_proc->tag == 4) goto compound_apply;
    goto unknown_procedure_type;

primitive_apply:
    {
        /* apply the primitive to argl's items */
        Value *items = NIL(); Value *t = NULL;
        for (Value *a = R_argl; a->tag == 2; a = a->cdr) {
            if (!t) { items = pair(a->car, NIL()); t = items; }
            else { t->cdr = pair(a->car, NIL()); t = t->cdr; }
        }
        R_val = prim_apply(R_proc->i, items);
    }
    (void)dsp; (void)dispose_stack;
    R_continue = spop();            /* the entry continue, as the book's primitive-branch restores */
    goto *R_continue;

compound_apply:
    R_env = extend(R_proc->params, R_argl, R_proc->env);
    R_unev = R_proc->body;
    goto ev_sequence;

ev_begin:
    R_unev = R_exp->cdr;
    spush_label(R_continue);
    goto ev_sequence;

ev_sequence:
    R_exp = R_unev->car;
    if (R_unev->cdr->tag != 2) goto ev_sequence_last_exp;
    spush(R_unev);
    spush(R_env);
    R_continue = &&ev_sequence_continue;
    goto eval_dispatch;

ev_sequence_continue:
    R_env = spop();
    R_unev = spop();
    R_unev = R_unev->cdr;
    goto ev_sequence;

ev_sequence_last_exp:
    R_continue = spop();
    goto eval_dispatch;

ev_if:
    spush(R_exp);
    spush(R_env);
    spush_label(R_continue);
    R_continue = &&ev_if_decide;
    R_exp = R_exp->cdr->car;
    goto eval_dispatch;

ev_if_decide:
    R_continue = spop();
    R_env = spop();
    R_exp = spop();
    R_flag = is_true(R_val);
    if (R_flag) goto ev_if_consequent;
    goto ev_if_alternative;

ev_if_alternative:
    R_exp = R_exp->cdr->cdr->cdr->tag == 2 ? R_exp->cdr->cdr->cdr->car : boolean(0);
    goto eval_dispatch;

ev_if_consequent:
    R_exp = R_exp->cdr->cdr->car;
    goto eval_dispatch;

ev_assignment:
    R_unev = R_exp->cdr->car;
    spush(R_unev);
    R_exp = R_exp->cdr->cdr->car;
    spush(R_env);
    spush_label(R_continue);
    R_continue = &&ev_assignment_1;
    goto eval_dispatch;

ev_assignment_1:
    R_continue = spop();
    R_env = spop();
    R_unev = spop();
    set_var(R_unev, R_val, R_env);
    R_val = sym("ok");
    goto *R_continue;

ev_definition:
    { Value *target = R_exp->cdr->car;
      if (target->tag == 2) {
          R_unev = target->car;        /* the defined name */
          R_exp = pair(sym("lambda"), pair(target->cdr, R_exp->cdr->cdr));
      } else {
          R_unev = target;
          R_exp = R_exp->cdr->cdr->car;
      } }
    spush(R_unev);
    spush(R_env);
    spush_label(R_continue);
    R_continue = &&ev_definition_1;
    goto eval_dispatch;

ev_definition_1:
    R_continue = spop();
    R_env = spop();
    R_unev = spop();
    define_var(R_unev, R_val, R_env);
    R_val = sym("ok");
    goto *R_continue;

unknown_expression_type:
unknown_procedure_type:
    fprintf(stderr, "unknown type\n");
    exit(2);
}

int main(int argc, char **argv) {
    TRUE_V.tag = 5; TRUE_V.i = 1;
    FALSE_V.tag = 5; FALSE_V.i = 0;
    NIL_V.tag = 6;
    if (argc < 2) { fprintf(stderr, "usage: eceval program.scm\n"); return 1; }
    {
        FILE *f = fopen(argv[1], "rb");
        long len;
        if (!f) { fprintf(stderr, "cannot open %s\n", argv[1]); return 1; }
        fseek(f, 0, SEEK_END);
        len = ftell(f);
        fseek(f, 0, SEEK_SET);
        program_text = calloc(1, (size_t)len + 1);
        if (len > 0 && fread(program_text, 1, (size_t)len, f) != (size_t)len) {
            fprintf(stderr, "read error\n");
            return 1;
        }
        fclose(f);
    }
    cursor = program_text;
    eceval();
    fprintf(stderr, "pushes=%d depth=%d\n", epushes, emax);
    return 0;
}
    """.trimIndent()

/** The book's factorial session. */
private val factorialSession: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    (factorial 5)
    """.trimIndent()

/** Builds the translation with `cc -O1`, runs the session, and answers stdout lines. */
public fun translatedEvaluatorRuns(): List<String> {
    val dir = Files.createTempDirectory("sicp_5_51").toFile()
    try {
        File(dir, "eceval.c").writeText(ecevalC)
        File(dir, "program.scm").writeText(factorialSession)
        val build =
            ProcessBuilder("cc", "-O1", "-o", "eceval", "eceval.c")
                .directory(dir)
                .redirectErrorStream(true)
                .start()
        val buildOut = build.inputStream.bufferedReader().readText()
        check(build.waitFor(300, TimeUnit.SECONDS) && build.exitValue() == 0) {
            "the C translation failed to build: $buildOut"
        }
        val run =
            ProcessBuilder("./eceval", "program.scm")
                .directory(dir)
                .start()
        val out = run.inputStream.bufferedReader().readText()
        check(run.waitFor(300, TimeUnit.SECONDS)) { "the C evaluator timed out" }
        check(run.exitValue() == 0) { "the C evaluator failed" }
        return out.split("\n").filter { it.isNotEmpty() }
    } finally {
        dir.deleteRecursively()
    }
}
