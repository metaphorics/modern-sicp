// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.51: develop a rudimentary implementation of the
// guest language in C by translating the explicit-control evaluator of
// 5.4 into C. The evaluator's registers ride as one machine struct's
// fields, every `ev-` entry point is a C label of the single evaluator
// function, `continue` holds a return label, and the frame stack is one
// array with the same push/pop discipline the machines of 5.2 use -- the
// book's saves and restores made literal. The run-time support the
// statement asks for -- storage allocation, the value model,
// environments, and the section 3.7 output convention -- rides along in
// the same file.
//
// The program validates its whole input before any effect: the reader
// runs to completion, the load pass resolves every call's name and
// arity, and constructs outside the documented guest core are rejected
// with an explicit diagnostic. Only then does the evaluator run.

package sicp.ch5.solutions

import sicp.ch5.ExplicitControl
import java.io.File
import java.nio.file.Files
import java.util.concurrent.TimeUnit

private val ecevalC: String =
    """
    /* The explicit-control evaluator of SICP 5.4, translated to C.
       The evaluator's registers are the fields of `struct Machine`.
       Every ev- entry point is a label of eceval(); the continue
       register holds a label address; the frame stack is one array
       carrying the book's saved registers beside each return address. */
    #include <ctype.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>

    /* ---- the value model and storage allocation ---- */

    typedef struct Value Value;
    typedef struct Env Env;
    typedef struct Expr Expr;
    typedef struct StmtNode StmtNode;
    typedef struct Fun Fun;

    struct Value {
        int tag;              /* 0 long, 1 bool, 2 unit, 3 closure */
        long n;
        int b;
        Fun *fun;
    };

    static void *allocate(size_t bytes) {
        void *p = calloc(1, bytes);
        if (!p) { fprintf(stderr, "out of memory\n"); exit(4); }
        return p;
    }

    static Value *vlong(long n) { Value *v = allocate(sizeof *v); v->tag = 0; v->n = n; return v; }
    static Value *vbool(int b) { Value *v = allocate(sizeof *v); v->tag = 1; v->b = b; return v; }
    static Value *vunit(void) { Value *v = allocate(sizeof *v); v->tag = 2; return v; }

    /* ---- the syntax: the documented guest core ---- */

    typedef enum { E_NUM, E_NAME, E_CALL, E_BIN, E_UN } ExprKind;
    struct Expr {
        ExprKind kind;
        long num;
        char *name;
        const char *op;
        Expr *a, *b;
        Expr **args;
        int nargs;
    };

    typedef enum { S_BLOCK, S_VAL, S_ASSIGN, S_IF, S_WHILE, S_RETURN, S_EXPR } StmtKind;
    struct StmtNode {
        StmtKind kind;
        char *name;
        Expr *value;
        StmtNode *thenPart, *elsePart, *body;
        StmtNode **stmts;
        int nstmts;
    };

    struct Fun {
        char *name;
        char **params;
        int arity;
        StmtNode *body;
        Fun *next;
    };

    /* ---- the reader ---- */

    static const char *cursor;

    static void reject(const char *what) {
        fprintf(stderr, "unsupported at load time: %s\n", what);
        exit(3);
    }

    static void skip(void) {
        for (;;) {
            while (isspace((unsigned char)*cursor)) cursor++;
            if (cursor[0] == '/' && cursor[1] == '/') { while (*cursor && *cursor != '\n') cursor++; continue; }
            return;
        }
    }

    static int at(const char *word) {
        size_t k = strlen(word);
        skip();
        if (strncmp(cursor, word, k) == 0 && !isalnum((unsigned char)cursor[k]) && cursor[k] != '_') {
            cursor += k;
            return 1;
        }
        return 0;
    }

    static void expect(const char *word) {
        skip();
        if (strncmp(cursor, word, strlen(word)) != 0) reject(word);
        cursor += strlen(word);
    }

    static char *ident(void) {
        skip();
        if (!isalpha((unsigned char)*cursor) && *cursor != '_') reject("identifier");
        const char *start = cursor;
        while (isalnum((unsigned char)*cursor) || *cursor == '_') cursor++;
        char *name = allocate((size_t)(cursor - start) + 1);
        memcpy(name, start, (size_t)(cursor - start));
        return name;
    }

    static Expr *expr(void);

    static Expr *new_expr(ExprKind kind) {
        Expr *e = allocate(sizeof *e);
        e->kind = kind;
        return e;
    }

    static Expr *primary(void) {
        skip();
        if (*cursor == '(') {
            cursor++;
            Expr *e = expr();
            expect(")");
            return e;
        }
        if (isdigit((unsigned char)*cursor)) {
            char *end;
            long n = strtol(cursor, &end, 10);
            cursor = end;
            if (*cursor == 'L' || *cursor == 'l') cursor++;
            Expr *e = new_expr(E_NUM);
            e->num = n;
            return e;
        }
        char *name = ident();
        if (strcmp(name, "true") == 0 || strcmp(name, "false") == 0) {
            Expr *e = new_expr(E_NUM);
            e->num = strcmp(name, "true") == 0 ? 1 : 0;
            e->name = name;
            e->op = "bool";
            return e;
        }
        skip();
        if (*cursor == '(') {
            cursor++;
            Expr *e = new_expr(E_CALL);
            e->name = name;
            skip();
            if (*cursor != ')') {
                for (;;) {
                    e->args = realloc(e->args, sizeof(Expr *) * (size_t)(e->nargs + 1));
                    e->args[e->nargs++] = expr();
                    skip();
                    if (*cursor == ',') { cursor++; continue; }
                    break;
                }
            }
            expect(")");
            return e;
        }
        Expr *e = new_expr(E_NAME);
        e->name = name;
        return e;
    }

    static Expr *unary(void) {
        skip();
        if (*cursor == '-') {
            cursor++;
            Expr *e = new_expr(E_UN);
            e->op = "-";
            e->a = unary();
            return e;
        }
        return primary();
    }

    static int bin_rank(const char *op) {
        if (strcmp(op, "*") == 0 || strcmp(op, "/") == 0 || strcmp(op, "%") == 0) return 3;
        if (strcmp(op, "+") == 0 || strcmp(op, "-") == 0) return 2;
        return 1;
    }

    static const char *bin_op(void) {
        static const char *two[] = {"<=", ">=", "==", "!=", NULL};
        static const char *one[] = {"<", ">", "+", "-", "*", "/", "%", NULL};
        skip();
        for (int i = 0; two[i]; i++)
            if (strncmp(cursor, two[i], 2) == 0) { cursor += 2; return two[i]; }
        for (int i = 0; one[i]; i++)
            if (cursor[0] == one[i][0]) { cursor += 1; return one[i]; }
        return NULL;
    }

    static Expr *binary_at(int min_rank) {
        Expr *left = unary();
        for (;;) {
            const char *save = cursor;
            const char *op = bin_op();
            if (!op || bin_rank(op) < min_rank) { cursor = save; return left; }
            Expr *right = binary_at(bin_rank(op) + 1);
            Expr *e = new_expr(E_BIN);
            e->op = op;
            e->a = left;
            e->b = right;
            left = e;
        }
    }

    static Expr *expr(void) { return binary_at(1); }

    static StmtNode *block(void);

    static StmtNode *statement(void) {
        if (at("val") || at("var")) {
            StmtNode *s = allocate(sizeof *s);
            s->kind = S_VAL;
            s->name = ident();
            expect(":");
            (void)ident();
            expect("=");
            s->value = expr();
            return s;
        }
        if (at("return")) {
            StmtNode *s = allocate(sizeof *s);
            s->kind = S_RETURN;
            s->value = expr();
            return s;
        }
        if (at("if")) {
            StmtNode *s = allocate(sizeof *s);
            s->kind = S_IF;
            expect("(");
            s->value = expr();
            expect(")");
            s->thenPart = block();
            if (at("else")) s->elsePart = block();
            return s;
        }
        if (at("while")) {
            StmtNode *s = allocate(sizeof *s);
            s->kind = S_WHILE;
            expect("(");
            s->value = expr();
            expect(")");
            s->body = block();
            return s;
        }
        {
            const char *save = cursor;
            char *name = ident();
            skip();
            if (*cursor == '=' && cursor[1] != '=') {
                cursor++;
                StmtNode *s = allocate(sizeof *s);
                s->kind = S_ASSIGN;
                s->name = name;
                s->value = expr();
                return s;
            }
            cursor = save;
        }
        {
            StmtNode *s = allocate(sizeof *s);
            s->kind = S_EXPR;
            s->value = expr();
            return s;
        }
    }

    static StmtNode *block(void) {
        expect("{");
        StmtNode *s = allocate(sizeof *s);
        s->kind = S_BLOCK;
        skip();
        while (*cursor != '}') {
            s->stmts = realloc(s->stmts, sizeof(StmtNode *) * (size_t)(s->nstmts + 1));
            s->stmts[s->nstmts++] = statement();
            skip();
        }
        expect("}");
        return s;
    }

    static Fun *functions = NULL;

    static void top_level(void) {
        skip();
        if (!*cursor) return;
        if (!at("fun")) reject("anything but top-level fun declarations");
        Fun *f = allocate(sizeof *f);
        f->name = ident();
        expect("(");
        skip();
        if (*cursor != ')') {
            for (;;) {
                f->params = realloc(f->params, sizeof(char *) * (size_t)(f->arity + 1));
                f->params[f->arity++] = ident();
                expect(":");
                (void)ident();
                if (at(",")) continue;
                break;
            }
        }
        expect(")");
        if (at(":")) (void)ident();
        f->body = block();
        f->next = functions;
        functions = f;
        top_level();
    }

    static Fun *find_fun(const char *name) {
        for (Fun *f = functions; f; f = f->next)
            if (strcmp(f->name, name) == 0) return f;
        return NULL;
    }

    /* The load pass: every call names a declared function with the
       declared arity, before any effect runs. */
    static void check_expr(Expr *e) {
        if (!e) return;
        switch (e->kind) {
        case E_NUM:
        case E_NAME:
            return;
        case E_UN:
            check_expr(e->a);
            return;
        case E_BIN:
            check_expr(e->a);
            check_expr(e->b);
            return;
        case E_CALL: {
            if (strcmp(e->name, "println") == 0) {
                if (e->nargs != 1) reject("println needs one argument");
            } else {
                Fun *f = find_fun(e->name);
                if (!f) { fprintf(stderr, "unknown function at load time: %s\n", e->name); exit(3); }
                if (f->arity != e->nargs) {
                    fprintf(stderr, "arity mismatch at load time: %s expected %d, given %d\n", e->name, f->arity, e->nargs);
                    exit(3);
                }
            }
            for (int i = 0; i < e->nargs; i++) check_expr(e->args[i]);
            return;
        }
        }
    }

    static void check_stmt(StmtNode *s) {
        if (!s) return;
        switch (s->kind) {
        case S_BLOCK:
            for (int i = 0; i < s->nstmts; i++) check_stmt(s->stmts[i]);
            return;
        case S_VAL:
        case S_ASSIGN:
        case S_EXPR:
            check_expr(s->value);
            return;
        case S_IF:
            check_expr(s->value);
            check_stmt(s->thenPart);
            check_stmt(s->elsePart);
            return;
        case S_WHILE:
            check_expr(s->value);
            check_stmt(s->body);
            return;
        case S_RETURN:
            check_expr(s->value);
            return;
        }
    }

    /* ---- environments ---- */

    struct Env {
        char **names;
        Value **slots;
        int count;
        Env *outer;
    };

    static Env *extend(Env *outer) { Env *e = allocate(sizeof *e); e->outer = outer; return e; }

    static Value **find_slot(const char *name, Env *env) {
        for (Env *e = env; e; e = e->outer)
            for (int i = 0; i < e->count; i++)
                if (strcmp(e->names[i], name) == 0) return &e->slots[i];
        return NULL;
    }

    static void define_name(Env *env, const char *name, Value *v) {
        Value **slot = find_slot(name, env);
        if (slot) { *slot = v; return; }
        env->names = realloc(env->names, sizeof(char *) * (size_t)(env->count + 1));
        env->slots = realloc(env->slots, sizeof(Value *) * (size_t)(env->count + 1));
        env->names[env->count] = (char *)name;
        env->slots[env->count] = v;
        env->count++;
    }

    /* ---- the machine: the evaluator's registers and stack ---- */

    struct Frame {
        void *kont;               /* local label identifying the frame */
        void *parent_kont;        /* continuation to restore when popped */
        Expr *exp;                /* the saved expression register */
        StmtNode *stmt;           /* the saved statement register */
        Env *env;                 /* the saved lexical environment */
        Value *val;               /* the saved value register */
        Value **argl;             /* the saved arguments */
        int argc, next;
    };

    struct Machine {
        Value *val;
        Value *retval;
        Expr *exp;
        StmtNode *stmt;
        Env *env;
        void *kont;
        Value **argl;
        int argc, next;
        struct Frame stack[16384];
        int sp;
        long steps;
    };

    static struct Machine machine;

    static void push_frame(void *kont) {
        if (machine.sp >= 16384) { fprintf(stderr, "stack overflow\n"); exit(4); }
        struct Frame *f = &machine.stack[machine.sp++];
        f->kont = kont;
        f->parent_kont = machine.kont;
        f->exp = machine.exp;
        f->stmt = machine.stmt;
        f->env = machine.env;
        f->val = machine.val;
        f->argl = machine.argl;
        f->argc = machine.argc;
        f->next = machine.next;
        machine.kont = kont;
    }

    static void *pop_frame(void) {
        if (machine.sp <= 0) { fprintf(stderr, "restore past the stack bottom\n"); exit(4); }
        struct Frame *f = &machine.stack[--machine.sp];
        machine.exp = f->exp;
        machine.stmt = f->stmt;
        machine.env = f->env;
        machine.val = f->val;
        machine.argl = f->argl;
        machine.argc = f->argc;
        machine.next = f->next;
        machine.kont = f->parent_kont;
        return machine.kont;
    }

    /* The explicit-control evaluator: every computed-goto target is
       local to this single function. */
    static void eceval(void) {
        machine.kont = &&finished;
        goto stmt_dispatch;

    ev_num:
        machine.val = machine.exp->op && strcmp(machine.exp->op, "bool") == 0
            ? vbool((int)machine.exp->num)
            : vlong(machine.exp->num);
        goto *machine.kont;

    ev_name: {
            Value **slot = find_slot(machine.exp->name, machine.env);
            if (!slot) { fprintf(stderr, "unbound name: %s\n", machine.exp->name); exit(3); }
            machine.val = *slot;
            goto *machine.kont;
        }

    ev_unary: {
            Expr *expression = machine.exp;
            push_frame(&&unary_done);
            machine.exp = expression->a;
            goto ev_dispatch;
        }

    unary_done: {
            Value *answer = vlong(-machine.val->n);
            pop_frame();
            machine.val = answer;
            goto *machine.kont;
        }

    ev_binary: {
            Expr *expression = machine.exp;
            push_frame(&&binary_left);
            machine.exp = expression->a;
            goto ev_dispatch;
        }

    binary_left:
        machine.stack[machine.sp - 1].val = machine.val;
        machine.exp = machine.stack[machine.sp - 1].exp->b;
        machine.kont = &&binary_right;
        goto ev_dispatch;

    binary_right: {
            struct Frame *f = &machine.stack[machine.sp - 1];
            Value *l = f->val;
            Value *r = machine.val;
            const char *op = f->exp->op;
            Value *answer;
            if (strcmp(op, "+") == 0) answer = vlong(l->n + r->n);
            else if (strcmp(op, "-") == 0) answer = vlong(l->n - r->n);
            else if (strcmp(op, "*") == 0) answer = vlong(l->n * r->n);
            else if (strcmp(op, "/") == 0 || strcmp(op, "%") == 0) {
                if (r->n == 0) { fprintf(stderr, "division by zero\n"); exit(3); }
                answer = vlong(strcmp(op, "/") == 0 ? l->n / r->n : l->n % r->n);
            } else if (strcmp(op, "<") == 0) answer = vbool(l->n < r->n);
            else if (strcmp(op, ">") == 0) answer = vbool(l->n > r->n);
            else if (strcmp(op, "<=") == 0) answer = vbool(l->n <= r->n);
            else if (strcmp(op, ">=") == 0) answer = vbool(l->n >= r->n);
            else if (strcmp(op, "==") == 0) answer = vbool(l->n == r->n);
            else answer = vbool(l->n != r->n);
            pop_frame();
            machine.val = answer;
            goto *machine.kont;
        }

    ev_call: {
            Expr *call = machine.exp;
            if (strcmp(call->name, "println") == 0) {
                push_frame(&&print_done);
                machine.exp = call->args[0];
                goto ev_dispatch;
            }
            if (!find_fun(call->name)) {
                fprintf(stderr, "unknown function: %s\n", call->name);
                exit(3);
            }
            push_frame(&&apply_done);
            machine.argl = allocate(sizeof(Value *) * (size_t)(call->nargs ? call->nargs : 1));
            machine.argc = call->nargs;
            machine.next = 0;
            machine.exp = call;
            machine.kont = &&call_next_arg;
            goto call_next_arg;
        }

    call_next_arg: {
            Expr *call = machine.stack[machine.sp - 1].exp;
            if (machine.next >= machine.argc) {
                machine.val = vunit();
                goto apply_args_done;
            }
            machine.exp = call->args[machine.next];
            machine.kont = &&arg_done;
            goto ev_dispatch;
        }

    arg_done: {
            machine.argl[machine.next] = machine.val;
            machine.next++;
            machine.exp = machine.stack[machine.sp - 1].exp;
            machine.kont = &&call_next_arg;
            goto call_next_arg;
        }

    apply_args_done: {
            Expr *call = machine.stack[machine.sp - 1].exp;
            Fun *f = find_fun(call->name);
            Value **args = machine.argl;
            Env *frame = extend(machine.env);
            for (int i = 0; i < f->arity; i++) define_name(frame, f->params[i], args[i]);
            machine.env = frame;
            machine.stmt = f->body;
            machine.kont = &&body_done;
            goto stmt_dispatch;
        }

    body_done:
        machine.retval = vunit();
        goto apply_done;

    apply_done: {
            Value *answer = machine.retval;
            pop_frame();
            machine.val = answer;
            goto *machine.kont;
        }

    print_done: {
            if (machine.val->tag == 0) printf("%ld\n", machine.val->n);
            else if (machine.val->tag == 1) printf(machine.val->b ? "true\n" : "false\n");
            else printf("unit\n");
            pop_frame();
            machine.val = vunit();
            goto *machine.kont;
        }

    ev_dispatch:
        machine.steps++;
        switch (machine.exp->kind) {
        case E_NUM: goto ev_num;
        case E_NAME: goto ev_name;
        case E_UN: goto ev_unary;
        case E_BIN: goto ev_binary;
        case E_CALL: goto ev_call;
        }
        goto *machine.kont;

    stmt_dispatch:
        machine.steps++;
        switch (machine.stmt->kind) {
        case S_BLOCK:
            if (machine.stmt->nstmts == 0) {
                machine.val = vunit();
                goto *machine.kont;
            }
            push_frame(&&block_advance);
            machine.stack[machine.sp - 1].next = 1;
            machine.stmt = machine.stmt->stmts[0];
            goto stmt_dispatch;
        case S_VAL:
            machine.exp = machine.stmt->value;
            push_frame(&&val_done);
            goto ev_dispatch;
        case S_ASSIGN:
            machine.exp = machine.stmt->value;
            push_frame(&&assign_done);
            goto ev_dispatch;
        case S_EXPR:
            machine.exp = machine.stmt->value;
            push_frame(&&expr_done);
            goto ev_dispatch;
        case S_IF:
            machine.exp = machine.stmt->value;
            push_frame(&&if_tested);
            goto ev_dispatch;
        case S_WHILE:
            push_frame(&&while_done);
            machine.exp = machine.stmt->value;
            machine.kont = &&while_tested;
            goto ev_dispatch;
        case S_RETURN:
            push_frame(&&return_done);
            if (!machine.stmt->value) {
                machine.val = vunit();
                goto return_done;
            }
            machine.exp = machine.stmt->value;
            goto ev_dispatch;
        }

    val_done: {
            Value *value = machine.val;
            pop_frame();
            define_name(machine.env, machine.stmt->name, value);
            machine.val = vunit();
            goto *machine.kont;
        }

    assign_done: {
            Value *value = machine.val;
            pop_frame();
            Value **slot = find_slot(machine.stmt->name, machine.env);
            if (!slot) {
                fprintf(stderr, "assignment to unbound name: %s\n", machine.stmt->name);
                exit(3);
            }
            *slot = value;
            machine.val = vunit();
            goto *machine.kont;
        }

    expr_done:
        pop_frame();
        machine.val = vunit();
        goto *machine.kont;

    if_tested: {
            Value *condition = machine.val;
            pop_frame();
            if (condition->tag != 1) {
                fprintf(stderr, "a condition is not Boolean\n");
                exit(3);
            }
            machine.val = condition;
            machine.stmt = condition->b ? machine.stmt->thenPart : machine.stmt->elsePart;
            if (!machine.stmt) {
                machine.val = vunit();
                goto *machine.kont;
            }
            goto stmt_dispatch;
        }

    while_tested:
        if (machine.val->tag != 1) {
            fprintf(stderr, "a condition is not Boolean\n");
            exit(3);
        }
        if (!machine.val->b) goto while_done;
        machine.stmt = machine.stmt->body;
        machine.kont = &&while_retest;
        goto stmt_dispatch;

    while_retest:
        machine.stmt = machine.stack[machine.sp - 1].stmt;
        machine.exp = machine.stmt->value;
        machine.kont = &&while_tested;
        goto ev_dispatch;

    while_done:
        pop_frame();
        machine.val = vunit();
        goto *machine.kont;

    return_done:
        machine.retval = machine.val;
        while (machine.sp > 0 && machine.stack[machine.sp - 1].kont != &&apply_done) {
            pop_frame();
        }
        if (machine.sp == 0) {
            machine.val = machine.retval;
            goto finished;
        }
        goto apply_done;

    block_advance: {
            struct Frame *top = &machine.stack[machine.sp - 1];
            StmtNode *block = top->stmt;
            int index = top->next;
            if (index >= block->nstmts) {
                pop_frame();
                machine.val = vunit();
                goto *machine.kont;
            }
            top->next = index + 1;
            machine.stmt = block->stmts[index];
            machine.kont = &&block_advance;
            goto stmt_dispatch;
        }

    finished:
        return;
    }

    int main(int argc, char **argv) {
        if (argc != 2) { fprintf(stderr, "usage: eceval <program>\n"); return 2; }
        FILE *file = fopen(argv[1], "rb");
        if (!file) { fprintf(stderr, "cannot open %s\n", argv[1]); return 2; }
        fseek(file, 0, SEEK_END);
        long size = ftell(file);
        fseek(file, 0, SEEK_SET);
        char *program_text = allocate((size_t)size + 1);
        if (fread(program_text, 1, (size_t)size, file) != (size_t)size) { fprintf(stderr, "read error\n"); return 2; }
        fclose(file);

        /* The load pass: read the whole program and resolve every call
           before any effect runs. */
        cursor = program_text;
        top_level();
        for (Fun *f = functions; f; f = f->next) check_stmt(f->body);
        Fun *entry = find_fun("main");
        if (!entry || entry->arity != 0) { fprintf(stderr, "no parameterless main\n"); return 2; }

        machine.val = vunit();
        machine.retval = vunit();
        machine.env = extend(NULL);
        machine.sp = 0;
        machine.steps = 0;
        machine.stmt = entry->body;
        eceval();
        return 0;
    }

    """.trimIndent()

/** The book's factorial session in guest source: the documented core
 *  this translation implements. */
public val factorialSessionSource: String =
    """
    fun factorial(n: Long): Long {
        if (n == 1L) {
            return 1L
        }
        return n * factorial(n - 1L)
    }

    fun main() {
        println(factorial(5L))
    }
    """.trimIndent()

/** Builds the translation with `cc -O1`, runs the session, and answers
 *  the C program's output beside the agreement verdict with the
 *  teaching engine's run of the same checked session. */
public fun translatedEvaluatorRuns(): List<String> {
    val dir = Files.createTempDirectory("sicp_5_51").toFile()
    try {
        File(dir, "eceval.c").writeText(ecevalC)
        File(dir, "session.kt").writeText(factorialSessionSource)
        val buildLog = File(dir, "build.log")
        val build =
            ProcessBuilder("cc", "-O1", "-o", "eceval", "eceval.c")
                .directory(dir)
                .redirectErrorStream(true)
                .redirectOutput(buildLog)
                .start()
        if (!build.waitFor(300, TimeUnit.SECONDS)) {
            build.destroyForcibly()
            error("the C translation build timed out")
        }
        check(build.exitValue() == 0) { "the C translation failed to build: ${buildLog.readText()}" }
        val outputFile = File(dir, "run.stdout")
        val errorsFile = File(dir, "run.stderr")
        val run =
            ProcessBuilder("./eceval", "session.kt")
                .directory(dir)
                .redirectOutput(outputFile)
                .redirectError(errorsFile)
                .start()
        if (!run.waitFor(300, TimeUnit.SECONDS)) {
            run.destroyForcibly()
            error("the C evaluator timed out: ${errorsFile.readText()}")
        }
        val out = outputFile.readText()
        check(run.exitValue() == 0) { "the C evaluator failed: ${errorsFile.readText()}\n$out" }
        val cLines = out.split("\n").filter { it.isNotEmpty() }
        val engineLines =
            outputLines(ExplicitControl.run(factorialSessionSource).fold({ fault -> error("the session did not run: $fault") }, { it }))
        return cLines + "the C evaluator agrees with the explicit-control run: ${cLines == engineLines}"
    } finally {
        dir.deleteRecursively()
    }
}
