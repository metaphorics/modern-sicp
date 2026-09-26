// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.52: the compiler's C backend. The instruction
// sequences the 5.5 compiler emits are translated one statement at a
// time into the statements of a single C function: registers are
// globals, labels are C labels, `continue` and procedure entries hold
// label addresses, and the machine's operations are one C dispatch
// mirroring the compiled operations table. Constants are emitted
// inline as constructors, since they are immutable values the machine
// only reads. Compiling the adapted metacircular source with this
// backend produces a Scheme interpreter in C; the build uses the
// system C compiler and the run answers 120.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.EvaluatorFault
import sicp.ch5.MachineError
import sicp.ch5.compileBlock
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.Value
import java.io.File
import java.nio.file.Files
import java.util.concurrent.TimeUnit

/** A label name as a C identifier. */
private fun cIdent(name: String): String = name.replace("-", "_")

/** A Kotlin string as a C string literal body. */
private fun cEscape(s: String): String = s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n")

/** A C expression building the immutable constant [v]. */
context(r: Raise<MachineError>)
private fun cValue(v: Value): String =
    when (v) {
        is VInt -> "num(${v.n})"
        is VBool -> if (v.b) "&TRUE_V" else "&FALSE_V"
        is VSym -> "sym(\"${cEscape(v.name)}\")"
        is VStr -> "string_const(\"${cEscape(v.s)}\")"
        is VNil -> "&NIL_V"
        is VPair -> "pair(${cValue(v.car)}, ${cValue(v.cdr)})"
        else -> r.raise(EvaluatorFault("the C backend needs a data constant, found $v"))
    }

/** A C expression for the operand source [src]. */
context(r: Raise<MachineError>)
private fun srcC(src: Source): String =
    when (src) {
        is Source.RegSrc -> "R_${src.reg}"
        is Source.ConstSrc -> cValue(src.v)
        is Source.LabelSrc -> "&&${cIdent(src.name)}"
        is Source.OpSrc -> r.raise(EvaluatorFault("the C backend needs a flat operand, found $src"))
    }

/** The operand sources padded to the dispatch's three slots. */
context(r: Raise<MachineError>)
private fun argsC(args: List<Source>): String {
    val rendered = args.map { srcC(it) }
    return when (rendered.size) {
        0 -> "NULL, NULL, NULL"
        1 -> "${rendered[0]}, NULL, NULL"
        2 -> "${rendered[0]}, ${rendered[1]}, NULL"
        3 -> "${rendered[0]}, ${rendered[1]}, ${rendered[2]}"
        else -> r.raise(EvaluatorFault("the C backend needs at most three operands"))
    }
}

/** The parameter names the one list constant of `extend-environment` carries. */
context(r: Raise<MachineError>)
private fun extendNames(
    op: String,
    src: Source,
): String {
    val list = (src as? Source.ConstSrc)?.v ?: r.raise(EvaluatorFault("$op needs a parameter list"))
    val names = ArrayList<String>()
    var cursor: Value = list
    while (cursor is VPair) {
        names.add((cursor.car as? VSym)?.name ?: r.raise(EvaluatorFault("$op needs parameter names")))
        cursor = cursor.cdr
    }
    if (cursor !is VNil) r.raise(EvaluatorFault("$op needs a proper parameter list"))
    return names.joinToString(" ")
}

/** One controller statement as C statements. */
context(r: Raise<MachineError>)
private fun stmtC(stmt: Stmt): String =
    when (stmt) {
        is Label -> {
            "${cIdent(stmt.name)}: ;\n"
        }

        is Assign -> {
            assignC(stmt.reg, stmt.src)
        }

        is Test -> {
            val cond = stmt.cond as? OpCond ?: r.raise(EvaluatorFault("the C backend needs an operation condition"))
            if (cond.name == "false?") {
                "R_flag = !is_true(${srcC(cond.args[0])});\n"
            } else {
                "R_flag = is_true(machine_op(\"${cond.name}\", ${argsC(cond.args)}));\n"
            }
        }

        is Branch -> {
            "if (R_flag) goto ${cIdent(stmt.label)};\n"
        }

        is Goto -> {
            when (val to = stmt.to) {
                is GotoTarget.Lbl -> {
                    "goto ${cIdent(to.name)};\n"
                }

                is GotoTarget.ByReg -> {
                    when (to.reg) {
                        "continue" -> "goto *R_continue;\n"
                        "val" -> "goto *R_entry;\n"
                        else -> r.raise(EvaluatorFault("the C backend needs a code address in ${to.reg}"))
                    }
                }
            }
        }

        is Save -> {
            "spush(R_${stmt.reg});\n"
        }

        is Restore -> {
            "R_${stmt.reg} = spop_v();\n"
        }

        is Perform -> {
            val act = stmt.act as? OpAct ?: r.raise(EvaluatorFault("the C backend needs an operation action"))
            "(void)machine_op(\"${act.name}\", ${argsC(act.args)});\n"
        }
    }

/** One assignment as C statements, with the entry, procedure, and environment builders special. */
context(r: Raise<MachineError>)
private fun assignC(
    reg: String,
    src: Source,
): String =
    when (src) {
        is Source.RegSrc -> {
            "R_$reg = R_${src.reg};\n"
        }

        is Source.ConstSrc -> {
            "R_$reg = ${cValue(src.v)};\n"
        }

        is Source.LabelSrc -> {
            "R_$reg = &&${cIdent(src.name)};\n"
        }

        is Source.OpSrc -> {
            when (src.name) {
                "compiled-procedure-entry" -> {
                    "R_$reg = R_proc;\nR_entry = R_proc->entry;\n"
                }

                "make-compiled-procedure" -> {
                    val entry =
                        src.args.getOrNull(0) as? Source.LabelSrc
                            ?: r.raise(EvaluatorFault("make-compiled-procedure needs a label entry"))
                    "R_$reg = make_compiled_procedure(&&${cIdent(entry.name)}, R_env);\n"
                }

                "extend-environment" -> {
                    if (src.args.size != 3) r.raise(EvaluatorFault("extend-environment needs three inputs"))
                    val names = extendNames(src.name, src.args[0])
                    "R_$reg = extend_compile(\"$names\", ${srcC(src.args[1])}, ${srcC(src.args[2])});\n"
                }

                else -> {
                    "R_$reg = machine_op(\"${src.name}\", ${argsC(src.args)});\n"
                }
            }
        }
    }

/** The C run-time support: values, environments, primitives, operations, driver. */
private val runtimeC: String =
    """
/* The C runtime of the compiled evaluator. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Value Value;
struct Value {
    int tag;              /* 0 int, 1 symbol, 2 pair, 3 primitive, 4 compiled, 5 bool, 6 nil, 7 string */
    long i;
    char *sym;
    char *str;
    Value *car, *cdr;
    void *entry;
    Value *env;
};
static Value TRUE_V, FALSE_V, NIL_V;
static Value *boolean(int b) { return b ? &TRUE_V : &FALSE_V; }
static Value *num(long n) { Value *v = calloc(1, sizeof *v); v->tag = 0; v->i = n; return v; }
static Value *sym(const char *s) { Value *v = calloc(1, sizeof *v); v->tag = 1; v->sym = strdup(s); return v; }
static Value *string_const(const char *s) { Value *v = calloc(1, sizeof *v); v->tag = 7; v->str = strdup(s); return v; }
static Value *pair(Value *a, Value *d) { Value *v = calloc(1, sizeof *v); v->tag = 2; v->car = a; v->cdr = d; return v; }
static int is_true(Value *v) { return !(v->tag == 5 && v->i == 0); }
static int val_eq(Value *a, Value *b) {
    if (a == b) return 1;
    if (a->tag != b->tag) return 0;
    if (a->tag == 0 || a->tag == 3) return a->i == b->i;
    if (a->tag == 1) return strcmp(a->sym, b->sym) == 0;
    return 0;
}
static Value *car(Value *v) { return v->tag == 2 ? v->car : sym("<car-of-atom>"); }
static Value *cdr(Value *v) { return v->tag == 2 ? v->cdr : sym("<cdr-of-atom>"); }

/* The environment table: id symbols to frame lists. */
static Value *env_keys[8192]; static Value *env_vals[8192]; static int env_n = 0;
static Value *env_of_id(const char *id) {
    for (int i = 0; i < env_n; i++)
        if (!strcmp(env_keys[i]->sym, id)) return env_vals[i];
    return &NIL_V;
}
static Value *env_register(Value *frames) {
    char buf[32]; snprintf(buf, sizeof buf, "env%d", env_n);
    Value *id = sym(buf);
    env_keys[env_n] = id; env_vals[env_n] = frames; env_n++;
    return id;
}
static Value *lookup_var(Value *name, Value *env_id) {
    for (Value *frames = env_of_id(env_id->sym); frames && frames->tag == 2; frames = frames->cdr) {
        Value *names = frames->car->car, *values = frames->car->cdr;
        while (names && names->tag == 2) {
            if (val_eq(names->car, name)) return values->car;
            names = names->cdr; values = values->cdr;
        }
    }
    fprintf(stderr, "unbound variable\n"); exit(3);
}
static void set_var(Value *name, Value *val, Value *env_id) {
    for (Value *frames = env_of_id(env_id->sym); frames && frames->tag == 2; frames = frames->cdr) {
        Value *names = frames->car->car, *values = frames->car->cdr;
        Value *n = names, *v = values;
        while (n && n->tag == 2) {
            if (val_eq(n->car, name)) { v->car = val; return; }
            n = n->cdr; v = v->cdr;
        }
    }
}
static void define_var(Value *name, Value *val, Value *env_id) {
    for (Value *frames = env_of_id(env_id->sym); frames && frames->tag == 2; frames = frames->cdr) {
        Value *names = frames->car->car, *values = frames->car->cdr;
        Value *n = names, *v = values;
        while (n && n->tag == 2) {
            if (val_eq(n->car, name)) { v->car = val; return; }
            n = n->cdr; v = v->cdr;
        }
        frames->car = pair(pair(name, names), pair(val, values));
        return;
    }
}

static void print_value_pub(Value *v);

static const char *prim_names[] = {
    "cons","car","cdr","null?","pair?","symbol?","number?","string?","eq?","equal?",
    "+","-","*","/","=","<",">","<=",">=","remainder","quotient","abs","not",
    "list","error","display","newline","cadr","caddr","cadddr","caadr","cdadr","cddr","cdddr",
    "extend-environment","lookup-variable-value","set-variable-value!","define-variable!",
    "apply-in-underlying-scheme"
};
#define NPRIMS (long)(sizeof(prim_names)/sizeof(*prim_names))
static Value *prim_apply(long idx, Value *args) {
    const char *n = prim_names[idx];
    Value *a = args->tag == 2 ? args->car : &NIL_V;
    Value *b = args->tag == 2 && args->cdr->tag == 2 ? args->cdr->car : &NIL_V;
    Value *c = args->tag == 2 && args->cdr->tag == 2 && args->cdr->cdr->tag == 2 ? args->cdr->cdr->car : &NIL_V;
    if (!strcmp(n, "cons")) return pair(a, b);
    if (!strcmp(n, "car")) return car(a);
    if (!strcmp(n, "cdr")) return cdr(a);
    if (!strcmp(n, "null?")) return boolean(a->tag == 6);
    if (!strcmp(n, "pair?")) return boolean(a->tag == 2);
    if (!strcmp(n, "symbol?")) return boolean(a->tag == 1);
    if (!strcmp(n, "number?")) return boolean(a->tag == 0);
    if (!strcmp(n, "string?")) return boolean(a->tag == 7);
    if (!strcmp(n, "eq?")) return boolean(val_eq(a, b));
    if (!strcmp(n, "equal?")) return boolean(val_eq(a, b));
    if (!strcmp(n, "not")) return boolean(!is_true(a));
    if (!strcmp(n, "+") || !strcmp(n, "-") || !strcmp(n, "*")) {
        long x = a->i, y = b->i;
        return num(n[0] == '+' ? x + y : n[0] == '-' ? x - y : x * y);
    }
    if (!strcmp(n, "/")) return num(b->i ? a->i / b->i : 0);
    if (!strcmp(n, "=")) return boolean(a->i == b->i);
    if (!strcmp(n, "<")) return boolean(a->i < b->i);
    if (!strcmp(n, ">")) return boolean(a->i > b->i);
    if (!strcmp(n, "<=")) return boolean(a->i <= b->i);
    if (!strcmp(n, ">=")) return boolean(a->i >= b->i);
    if (!strcmp(n, "remainder")) return num(a->i % b->i);
    if (!strcmp(n, "quotient")) return num(a->i / b->i);
    if (!strcmp(n, "abs")) return num(a->i < 0 ? -a->i : a->i);
    if (!strcmp(n, "list")) return args;
    if (!strcmp(n, "error")) { fprintf(stderr, "error\n"); exit(2); }
    if (!strcmp(n, "display")) { print_value_pub(a); return a; }
    if (!strcmp(n, "newline")) { printf("\n"); return sym("newline"); }
    if (!strcmp(n, "cadr")) return car(cdr(a));
    if (!strcmp(n, "caddr")) return car(cdr(cdr(a)));
    if (!strcmp(n, "cadddr")) return car(cdr(cdr(cdr(a))));
    if (!strcmp(n, "caadr")) return car(car(cdr(a)));
    if (!strcmp(n, "cdadr")) return cdr(car(cdr(a)));
    if (!strcmp(n, "cddr")) return cdr(cdr(a));
    if (!strcmp(n, "cdddr")) return cdr(cdr(cdr(a)));
    if (!strcmp(n, "extend-environment")) {
        Value *frame = pair(a, b);
        Value *frames = (c->tag == 1 && !strcmp(c->sym, "the-empty"))
            ? pair(frame, &NIL_V)
            : pair(frame, env_of_id(c->sym));
        return env_register(frames);
    }
    if (!strcmp(n, "lookup-variable-value")) return lookup_var(a, b);
    if (!strcmp(n, "set-variable-value!")) { set_var(a, b, c); return sym("ok"); }
    if (!strcmp(n, "define-variable!")) { define_var(a, b, c); return sym("ok"); }
    if (!strcmp(n, "apply-in-underlying-scheme")) return prim_apply(a->i, b);
    return sym("unimplemented");
}
static void print_value_pub(Value *v) {
    if (!v) { printf("()"); return; }
    switch (v->tag) {
    case 0: printf("%ld", v->i); break;
    case 1: printf("%s", v->sym); break;
    case 6: printf("()"); break;
    case 5: printf(v->i ? "#t" : "#f"); break;
    case 7: printf("\"%s\"", v->str); break;
    case 3: printf("#[primitive]"); break;
    case 4: printf("#[compiled-procedure]"); break;
    case 2: {
        printf("(");
        print_value_pub(v->car);
        for (Value *d = v->cdr; d && d->tag == 2; d = d->cdr) { printf(" "); print_value_pub(d->car); }
        printf(")");
        break;
    }
    default: printf("#[?]");
    }
}

/* The machine registers. */
static Value *R_exp, *R_env, *R_val, *R_proc, *R_argl, *R_unev, *R_arg1, *R_arg2;
static void *R_continue, *R_entry;
static int R_flag;
static Value *estack[400000];
static int esp = 0;
static void spush(Value *v) { estack[esp] = v; esp++; }
static Value *spop_v(void) { esp--; return estack[esp]; }

static Value *make_compiled_procedure(void *entry, Value *env_id) {
    Value *v = calloc(1, sizeof *v);
    v->tag = 4; v->entry = entry; v->env = env_id;
    return v;
}
static Value *machine_op(const char *name, Value *w1, Value *w2, Value *w3) {
    if (!strcmp(name, "lookup-variable-value")) return lookup_var(w1, w2);
    if (!strcmp(name, "text-of-quotation")) return w1;
    if (!strcmp(name, "false?")) return boolean(!is_true(w1));
    if (!strcmp(name, "empty-arglist")) return &NIL_V;
    if (!strcmp(name, "list")) return w1 ? pair(w1, &NIL_V) : &NIL_V;
    if (!strcmp(name, "cons")) return pair(w1, w2);
    if (!strcmp(name, "compiled-procedure-env")) return w1->env;
    if (!strcmp(name, "primitive-procedure?")) return boolean(w1->tag == 3);
    if (!strcmp(name, "compound-procedure?")) return boolean(w1->tag == 4);
    if (!strcmp(name, "set-variable-value!")) { set_var(w1, w2, w3); return sym("ok"); }
    if (!strcmp(name, "define-variable!")) { define_var(w1, w2, w3); return sym("ok"); }
    if (!strcmp(name, "apply-primitive-procedure")) {
        if (w1->tag != 3) { fprintf(stderr, "apply of a non-primitive\n"); exit(4); }
        return prim_apply(w1->i, w2);
    }
    if (!strcmp(name, "+") || !strcmp(name, "-") || !strcmp(name, "*")) {
        long x = w1->i, y = w2->i;
        return num(name[0] == '+' ? x + y : name[0] == '-' ? x - y : x * y);
    }
    if (!strcmp(name, "=")) return boolean(w1->i == w2->i);
    if (!strcmp(name, "<")) return boolean(w1->i < w2->i);
    return sym("no-such-machine-op");
}
static Value *extend_compile(const char *names_csv, Value *args, Value *base_id) {
    Value *names = &NIL_V;
    char buf[256]; snprintf(buf, sizeof buf, "%s", names_csv);
    for (char *t = strtok(buf, " "); t; t = strtok(NULL, " ")) names = pair(sym(t), names);
    { Value *r = &NIL_V; for (Value *p = names; p && p->tag == 2; p = p->cdr) r = pair(p->car, r); names = r; }
    Value *frame = pair(names, args);
    Value *frames = (base_id->tag == 1 && !strcmp(base_id->sym, "the-empty"))
        ? pair(frame, &NIL_V)
        : pair(frame, env_of_id(base_id->sym));
    return env_register(frames);
}

/* The driver: run the compiled program, print the value. */
static void compiled_program(void);
static void init_registers(void);
int main(void) {
    TRUE_V.tag = 5; TRUE_V.i = 1;
    FALSE_V.tag = 5; FALSE_V.i = 0;
    NIL_V.tag = 6;
    init_registers();
    R_env = env_register(pair(pair(&NIL_V, &NIL_V), &NIL_V));
    for (long i = 0; i < NPRIMS; i++) {
        Value *p = calloc(1, sizeof *p);
        p->tag = 3;
        p->i = i;
        define_var(sym(prim_names[i]), p, R_env);
    }
    define_var(sym("true"), &TRUE_V, R_env);
    define_var(sym("false"), &FALSE_V, R_env);
    compiled_program();
    return 0;
}
    """.trimIndent()

/** Compiles [source] to a whole C file: the runtime, the registers the
 *  program assigns before use, and the compiled program as one function. */
context(r: Raise<MachineError>)
private fun compileToC(source: String): String {
    val state = CompilerState()
    val (entry, block) = compileBlock(CompilerConfig(), state, readForms(source))
    val body = block.joinToString("") { stmtC(it) }
    return runtimeC +
        "\nstatic void compiled_program(void);\n" +
        "static void init_registers(void) {\n" +
        "  R_exp = &NIL_V; R_env = &NIL_V; R_val = &NIL_V; R_proc = &NIL_V;\n" +
        "  R_argl = &NIL_V; R_unev = &NIL_V; R_arg1 = &NIL_V; R_arg2 = &NIL_V;\n" +
        "  R_continue = 0; R_entry = 0; R_flag = 0;\n" +
        "}\n" +
        "void compiled_program(void) {\n" +
        "/* the top-level continuation must be a label of this function:\n" +
        "   a computed goto cannot cross function boundaries. */\n" +
        "if (!R_continue) R_continue = &&finish;\n" +
        "goto ${cIdent(entry)};\n" +
        body +
        "\nfinish: ;\n" +
        "print_value_pub(R_val);\n" +
        "printf(\"\\n\");\n" +
        "}\n"
}

/** Writes [cSource], builds it with `cc -O1`, runs it, and answers stdout lines. */
private fun buildAndRunC(cSource: String): List<String> {
    val dir = Files.createTempDirectory("sicp_5_52").toFile()
    try {
        File(dir, "compiled.c").writeText(cSource)
        val build =
            ProcessBuilder("cc", "-O1", "-o", "compiled", "compiled.c")
                .directory(dir)
                .redirectErrorStream(true)
                .start()
        val buildOut = build.inputStream.bufferedReader().readText()
        check(build.waitFor(300, TimeUnit.SECONDS) && build.exitValue() == 0) {
            "the C backend failed to build: $buildOut"
        }
        val run = ProcessBuilder("./compiled").directory(dir).start()
        val out = run.inputStream.bufferedReader().readText()
        check(run.waitFor(600, TimeUnit.SECONDS)) { "the C interpreter timed out" }
        check(run.exitValue() == 0) { "the C interpreter failed" }
        return out.split("\n").filter { it.isNotEmpty() }
    } finally {
        dir.deleteRecursively()
    }
}

/** Compiles the adapted metacircular to C, builds it, and runs the object factorial. */
public fun compiledInterpreterRuns(): List<String> {
    val source =
        metacircularEvaluatorSource +
            "\n(m-eval '(factorial 5) the-global-environment)\n"
    val cSource = either { compileToC(source) }.fold({ error("the C backend failed: $it") }, { it })
    return buildAndRunC(cSource)
}
