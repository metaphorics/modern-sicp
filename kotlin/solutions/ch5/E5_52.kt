// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.52: emit C from the compiler's typed
// instruction sequence. The emitter writes one C function with
// machine registers, labels, a stack, and dispatch over the
// `compiled-*` operations; its C support implements the compiler's whole
// vocabulary -- environments, procedure entries and closures, class and
// object construction, methods, pairs, lists, maps, collections, type
// tests, arithmetic, and output. Guest errors leave the process with their
// category, so the C program is compared with the direct, explicit-control,
// and compiled-machine runs on output and on error category, for the
// factorial probe, a structured probe, and the canonical self-interpreter
// of exercise 5.50. The reference `CBackend.emitProgram` is a separate
// complete artifact and must not be mistaken for the exercise's emitter.

package sicp.ch5.solutions

import sicp.ch4.Direct
import sicp.ch5.CBackend
import sicp.ch5.Compiler
import sicp.ch5.ExplicitControl
import sicp.guest.CheckedProgram
import sicp.guest.DataClass
import sicp.guest.DataObject
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.PlainClass
import sicp.guest.RunResult
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
import java.io.File
import java.nio.file.Files
import java.util.concurrent.TimeUnit

/** The small compiled program the exercise's execution leg runs: one
 *  recursive procedure and a `main` that prints its answer. */
public val factorialProbeSource: String =
    """
    fun factorial(n: Long): Long = if (n < 2L) 1L else n * factorial(n - 1L)

    fun main() {
        println(factorial(5L))
    }
    """.trimIndent()

/** A program that stops on a typed guest error after one effect: the C
 *  program must report the same category the three engines do. */
public val errorProbeSource: String =
    """
    fun divide(a: Long, b: Long): Long = a / b

    fun main() {
        println(1L)
        println(divide(1L, 0L))
        println(2L)
    }
    """.trimIndent()

/** A program over the structured values the canonical evaluator relies
 *  on: sealed data classes and objects, methods, mutable maps and lists,
 *  closures over a `var`, the collection operations, pairs, doubles, and
 *  string templates. */
public val structuredProbeSource: String =
    """
    sealed interface Shape

    data class Dot(val at: Long) : Shape

    data class Box(val width: Double, val height: Double) : Shape

    data object Nothingness : Shape

    class Counter(var count: Long) {
        fun bump(by: Long): Long {
            this.count = this.count + by
            return this.count
        }
    }

    fun area(shape: Shape): Double =
        when (shape) {
            is Dot -> 0.0
            is Box -> shape.width * shape.height
            is Nothingness -> -1.0
        }

    fun makeAdder(start: Long): (Long) -> Long {
        var total = start
        return { step: Long ->
            total = total + step
            total
        }
    }

    fun main() {
        val shapes: List<Shape> = listOf(Dot(3L), Box(2.5, 4.0), Nothingness)
        println(shapes.map { s: Shape -> area(s) }.fold(0.0) { acc: Double, x: Double -> acc + x })
        val moved = Dot(1L).copy(at = 9L)
        println("moved to ${'$'}{moved.at}")
        val counter = Counter(10L)
        println(counter.bump(5L) + counter.bump(1L))
        val add = makeAdder(100L)
        add(1L)
        println(add(2L))
        val table = mutableMapOf("one" to 1L)
        table["two"] = 2L
        println(table.get("two") ?: 0L)
        val (left, right) = 7L to 8L
        println(left * right)
        println(listOf(1L, 2L, 3L, 4L).filter { n: Long -> n % 2L == 0L }.size.toLong())
        println(setOf(1L to 2L, 1L to 2L).size.toLong())
    }
    """.trimIndent()

/** A label name as a C identifier. */
private fun cIdent(name: String): String = name.replace("-", "_")

/** A Kotlin string as a C string literal body. */
private fun cEscape(s: String): String =
    s
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")

/** A C expression for the double [d], including the values C has no literal for. */
private fun cDouble(d: Double): String =
    when {
        d.isNaN() -> "vdouble(0.0 / 0.0)"
        d == Double.POSITIVE_INFINITY -> "vdouble(HUGE_VAL)"
        d == Double.NEGATIVE_INFINITY -> "vdouble(-HUGE_VAL)"
        else -> "vdouble($d)"
    }

/** A C expression building the constant [v]; a compiled procedure constant
 *  (a lambda or local function) points at its entry label. */
private fun cValue(v: GValue): String =
    when (v) {
        is GValue.VInt -> "vint(${v.value})"

        is GValue.VLong -> if (v.value == Long.MIN_VALUE) "vlong(-9223372036854775807L - 1)" else "vlong(${v.value}L)"

        is GValue.VDouble -> cDouble(v.value)

        is GValue.VBool -> if (v.value) "&TRUE_V" else "&FALSE_V"

        is GValue.VString -> "vstring(\"${cEscape(v.value)}\")"

        is GValue.VNull -> "&NULL_V"

        is GValue.VUnit -> "&UNIT_V"

        is GValue.VList -> "vlist(${v.items.size}, ${if (v.items.isEmpty()) {
            "NULL"
        } else {
            "(Value*[]){${v.items.joinToString(
                ", ",
            ) { cValue(it) }}}"
        }})"

        is GValue.VObject -> procedureConstant(v)

        else -> error("the C emitter cannot encode this constant value: ${v::class.simpleName}")
    }

private fun procedureConstant(v: GValue.VObject): String {
    check(v.className == "compiled-procedure") { "the C emitter cannot encode an object constant: ${v.className}" }
    val entry = (v.fields["entry"] as GValue.VString).value
    val params = (v.fields["params"] as GValue.VList).items.map { (it as GValue.VString).value }
    return "vproc((void*)&&${cIdent(entry)}, ${params.size}, ${paramsC(params)})"
}

/** A C `const char *` array of [names], or NULL for none. */
private fun paramsC(names: List<String>): String =
    if (names.isEmpty()) "NULL" else "(const char*[]){${names.joinToString(", ") { "\"${cEscape(it)}\"" }}}"

/** A C expression for the operand source [src]. */
private fun srcC(src: Source): String =
    when (src) {
        is Source.RegSrc -> "R_${cIdent(src.reg)}"
        is Source.ConstSrc -> cValue(src.v)
        is Source.LabelSrc -> "(Value *)&&${cIdent(src.name)}"
        is Source.OpSrc -> "machine_op(\"${src.name}\", ${argsC(src.args)})"
    }

/** The operand sources padded to the dispatch's four slots. */
private fun argsC(args: List<Source>): String {
    check(args.size <= 4) { "the C backend cannot dispatch an operation with ${args.size} operands" }
    return (args.map { srcC(it) } + List(4 - args.size) { "NULL" }).joinToString(", ")
}

/** One controller statement as C statements. */
private fun stmtC(stmt: Stmt): String =
    when (stmt) {
        is Label -> {
            "${cIdent(stmt.name)}: ;\n"
        }

        is Assign -> {
            assignC(stmt.reg, stmt.src)
        }

        is Test -> {
            val condition = stmt.cond as? OpCond ?: error("the C backend needs an operation condition")
            "R_flag = is_true(machine_op(\"${condition.name}\", ${argsC(condition.args)}));\n"
        }

        is Branch -> {
            "if (R_flag) goto ${cIdent(stmt.label)};\n"
        }

        is Goto -> {
            when (val to = stmt.to) {
                is GotoTarget.Lbl -> "goto ${cIdent(to.name)};\n"
                is GotoTarget.ByReg -> "goto *(void*)R_${cIdent(to.reg)};\n"
            }
        }

        is Save -> {
            "spush(R_${cIdent(stmt.reg)});\n"
        }

        is Restore -> {
            "R_${cIdent(stmt.reg)} = spop_v();\n"
        }

        is Perform -> {
            val action = stmt.act as? OpAct ?: error("the C backend needs an operation action")
            "(void)machine_op(\"${action.name}\", ${argsC(action.args)});\n"
        }
    }

/** One assignment as C statements. */
private fun assignC(
    reg: String,
    src: Source,
): String = "R_${cIdent(reg)} = ${srcC(src)};\n"

/** The C run-time support: values, environments, the class and method
 *  tables, the `compiled-*` operation dispatch, and the section 3.7
 *  printing convention. */
private val runtimeC: String =
    """
    /* The C run-time support of the compiled code: one value model (scalars,
       lists, pairs, maps, objects, procedures, native member functions, and the
       collection carrier), one environment chain, the class and method tables of
       the program, and the machine's operation dispatch over the compiler's
       `compiled-*` vocabulary. Guest errors leave the process with status 1 and a
       `guest error: <Category>` line on stderr; anything the backend cannot do
       leaves with status 3 and a `C backend fault:` line. */
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
    #include <math.h>
    #include <limits.h>

    typedef struct Value Value;
    typedef struct Env Env;
    enum { T_LONG, T_DOUBLE, T_BOOL, T_STRING, T_NULL, T_UNIT, T_PROC, T_LIST, T_ENV, T_OBJECT, T_MAP, T_PAIR, T_NATIVE, T_COLLECTION, T_INT };
    struct Value {
        int tag;
        long n;
        double d;
        int b;              /* boolean payload */
        int flag;           /* object: structural; list: read as a set */
        int mut;            /* list, map: mutable */
        char *s;            /* string text; object: class; native: member name; collection: kind */
        void *entry;        /* procedure: label address */
        Env *env;           /* procedure: captured environment */
        int arity;
        Value **items;      /* list elements, pair halves, object fields, map keys */
        Value **vals;       /* map values */
        int length;
        char **params;      /* procedure parameters, object field names, native copy names */
        Value *bound;       /* procedure: bound receiver; collection: callback */
        Value *acc;         /* collection: accumulator */
    };
    struct Env {
        char **names;
        Value **slots;
        int count;
        Env *outer;
    };

    static Value TRUE_V = { .tag = T_BOOL, .b = 1 };
    static Value FALSE_V = { .tag = T_BOOL, .b = 0 };
    static Value NULL_V = { .tag = T_NULL };
    static Value UNIT_V = { .tag = T_UNIT };

    static void guest_error(const char *category) {
        fprintf(stderr, "guest error: %s\n", category);
        exit(1);
    }
    static void host_fault(const char *what, const char *detail) {
        fprintf(stderr, "C backend fault: %s %s\n", what, detail ? detail : "");
        exit(3);
    }
    static void *allocate(size_t bytes) {
        void *p = calloc(1, bytes ? bytes : 1);
        if (!p) host_fault("out of memory", NULL);
        return p;
    }
    static char *copy_text(const char *s) {
        size_t n = strlen(s) + 1;
        char *p = allocate(n);
        memcpy(p, s, n);
        return p;
    }

    static Value *alloc_value(int tag) { Value *v = allocate(sizeof *v); v->tag = tag; return v; }
    static Value *vbool(int b) { return b ? &TRUE_V : &FALSE_V; }
    static Value *vlong(long n) { Value *v = alloc_value(T_LONG); v->n = n; return v; }
    static Value *vint(long n) { Value *v = alloc_value(T_INT); v->n = (long)(int)n; return v; }
    static Value *vdouble(double d) { Value *v = alloc_value(T_DOUBLE); v->d = d; return v; }
    static Value *vstring(const char *s) { Value *v = alloc_value(T_STRING); v->s = copy_text(s); return v; }
    static Value *vlist(int count, Value **items) {
        Value *v = alloc_value(T_LIST);
        v->length = count;
        v->items = allocate(sizeof(Value *) * (size_t)count);
        if (items) for (int i = 0; i < count; i++) v->items[i] = items[i];
        return v;
    }
    static Value *vpair(Value *first, Value *second) {
        Value *v = alloc_value(T_PAIR);
        v->length = 2;
        v->items = allocate(sizeof(Value *) * 2);
        v->items[0] = first;
        v->items[1] = second;
        return v;
    }
    static Value *vmap(int mut) {
        Value *v = alloc_value(T_MAP);
        v->mut = mut;
        return v;
    }
    static Value *venv(Env *env) { Value *v = alloc_value(T_ENV); v->env = env; return v; }
    static Value *new_proc(void *entry, int arity, const char **params, Env *env) {
        Value *p = alloc_value(T_PROC);
        p->entry = entry;
        p->env = env;
        p->arity = arity;
        p->params = allocate(sizeof(char *) * (size_t)arity);
        for (int i = 0; i < arity; i++) p->params[i] = copy_text(params[i]);
        return p;
    }
    static Value *vproc(void *entry, int arity, const char **params) { return new_proc(entry, arity, params, NULL); }

    static void list_append(Value *list, Value *item) {
        list->items = realloc(list->items, sizeof(Value *) * (size_t)(list->length + 1));
        if (!list->items) host_fault("out of memory", NULL);
        list->items[list->length++] = item;
    }

    /* ---- environments ---- */
    static Env *root_env;
    static Env *globals(void) {
        if (!root_env) root_env = allocate(sizeof *root_env);
        return root_env;
    }
    static void define(Env *env, const char *name, Value *value) {
        for (int i = 0; i < env->count; i++)
            if (strcmp(env->names[i], name) == 0) { env->slots[i] = value; return; }
        env->names = realloc(env->names, sizeof(char *) * (size_t)(env->count + 1));
        env->slots = realloc(env->slots, sizeof(Value *) * (size_t)(env->count + 1));
        if (!env->names || !env->slots) host_fault("out of memory", NULL);
        env->names[env->count] = copy_text(name);
        env->slots[env->count++] = value;
    }
    static Env *child(Env *parent) {
        Env *env = allocate(sizeof *env);
        env->outer = parent;
        return env;
    }
    static Value **lookup_slot(Env *env, const char *name) {
        for (; env; env = env->outer)
            for (int i = env->count - 1; i >= 0; i--)
                if (strcmp(env->names[i], name) == 0) return &env->slots[i];
        return NULL;
    }
    static Env *as_env(Value *v) {
        if (!v || v->tag != T_ENV) host_fault("expected an environment", NULL);
        return v->env;
    }
    static Value *as_list(Value *v) {
        if (!v || v->tag != T_LIST) host_fault("expected an argument list", NULL);
        return v;
    }

    /* ---- the program's classes, objects, procedures, and methods ---- */
    typedef struct { char *name; int structural; } ClassInfo;
    static ClassInfo class_table[512];
    static int class_count;
    static void register_class(const char *name, int structural) {
        if (class_count == 512) host_fault("too many classes", NULL);
        class_table[class_count].name = copy_text(name);
        class_table[class_count++].structural = structural;
    }
    static int class_structural(const char *name) {
        for (int i = 0; i < class_count; i++)
            if (strcmp(class_table[i].name, name) == 0) return class_table[i].structural;
        host_fault("unknown class", name);
        return 0;
    }
    static Value *vobject(const char *class_name, int structural, int count) {
        Value *v = alloc_value(T_OBJECT);
        v->s = copy_text(class_name);
        v->flag = structural;
        v->length = count;
        v->params = allocate(sizeof(char *) * (size_t)count);
        v->items = allocate(sizeof(Value *) * (size_t)count);
        return v;
    }
    static void register_object(const char *name) {
        register_class(name, 1);
        define(globals(), name, vobject(name, 1, 0));
    }
    static void register_proc(const char *name, void *entry, int arity, const char **params) {
        define(globals(), name, new_proc(entry, arity, params, globals()));
    }
    typedef struct { char *owner; char *name; void *entry; int arity; char **params; } MethodInfo;
    static MethodInfo method_table[512];
    static int method_count;
    static void register_method(const char *owner, const char *name, void *entry, int arity, const char **params) {
        if (method_count == 512) host_fault("too many methods", NULL);
        MethodInfo *m = &method_table[method_count++];
        m->owner = copy_text(owner);
        m->name = copy_text(name);
        m->entry = entry;
        m->arity = arity;
        m->params = allocate(sizeof(char *) * (size_t)arity);
        for (int i = 0; i < arity; i++) m->params[i] = copy_text(params[i]);
    }
    static Value *method_proc(Value *receiver, const char *name) {
        if (receiver->tag != T_OBJECT) return NULL;
        for (int i = 0; i < method_count; i++) {
            MethodInfo *m = &method_table[i];
            if (strcmp(m->owner, receiver->s) == 0 && strcmp(m->name, name) == 0)
                return new_proc(m->entry, m->arity, (const char **)m->params, globals());
        }
        return NULL;
    }
    static Value **object_field(Value *object, const char *name) {
        for (int i = 0; i < object->length; i++)
            if (strcmp(object->params[i], name) == 0) return &object->items[i];
        return NULL;
    }

    /* ---- text ---- */
    static char *long_text(long n) {
        char buffer[32];
        snprintf(buffer, sizeof buffer, "%ld", n);
        return copy_text(buffer);
    }
    /* Double.toString: the shortest digits that read back exactly, positional
       for magnitudes in [1e-3, 1e7) and scientific with `E` otherwise. */
    static char *double_text(double d) {
        char buffer[64];
        if (isnan(d)) return copy_text("NaN");
        if (isinf(d)) return copy_text(d < 0 ? "-Infinity" : "Infinity");
        if (d == 0.0) return copy_text(signbit(d) ? "-0.0" : "0.0");
        for (int precision = 1; precision <= 17; precision++) {
            snprintf(buffer, sizeof buffer, "%.*e", precision - 1, d);
            if (strtod(buffer, NULL) == d) break;
        }
        char digits[40];
        int count = 0;
        int negative = 0;
        const char *p = buffer;
        if (*p == '-') { negative = 1; p++; }
        for (; *p && *p != 'e'; p++)
            if (*p != '.' && count < 39) digits[count++] = *p;
        int exponent = atoi(p + 1);
        while (count > 1 && digits[count - 1] == '0') count--;
        char out[96];
        int len = 0;
        if (negative) out[len++] = '-';
        if (exponent >= -3 && exponent < 7) {
            if (exponent >= 0) {
                for (int i = 0; i <= exponent; i++) out[len++] = i < count ? digits[i] : '0';
                out[len++] = '.';
                if (count > exponent + 1) for (int i = exponent + 1; i < count; i++) out[len++] = digits[i];
                else out[len++] = '0';
            } else {
                out[len++] = '0';
                out[len++] = '.';
                for (int i = 0; i < -exponent - 1; i++) out[len++] = '0';
                for (int i = 0; i < count; i++) out[len++] = digits[i];
            }
        } else {
            out[len++] = digits[0];
            out[len++] = '.';
            if (count > 1) for (int i = 1; i < count; i++) out[len++] = digits[i];
            else out[len++] = '0';
            len += snprintf(out + len, sizeof out - (size_t)len, "E%d", exponent);
        }
        out[len] = '\0';
        return copy_text(out);
    }
    /* The text a template fragment contributes: scalars print, everything else is empty. */
    static const char *text_of(Value *v) {
        switch (v->tag) {
        case T_STRING: return v->s;
        case T_LONG: case T_INT: return long_text(v->n);
        case T_DOUBLE: return double_text(v->d);
        case T_BOOL: return v->b ? "true" : "false";
        default: return "";
        }
    }
    /* Section 3.7 rendering: only Long, Double, Boolean, and String print. */
    static Value *render_value(Value *v) {
        switch (v->tag) {
        case T_STRING: return v;
        case T_LONG: return vstring(long_text(v->n));
        case T_DOUBLE: return vstring(double_text(v->d));
        case T_BOOL: return vstring(v->b ? "true" : "false");
        default: guest_error("ShapeFault");
        }
        return NULL;
    }

    /* ---- equality, ordering, arithmetic ---- */
    static int is_scalar(Value *v) {
        return v->tag == T_INT || v->tag == T_LONG || v->tag == T_DOUBLE || v->tag == T_BOOL || v->tag == T_STRING;
    }
    static int map_find(Value *map, Value *key);
    static int equal_values(Value *a, Value *b, int depth) {
        if (a == b) return 1;
        if (depth > 20000) return 0;
        if (a->tag != b->tag) return 0;
        switch (a->tag) {
        case T_INT: case T_LONG: return a->n == b->n;
        case T_DOUBLE: return a->d == b->d;
        case T_BOOL: return a->b == b->b;
        case T_STRING: return strcmp(a->s, b->s) == 0;
        case T_NULL: case T_UNIT: return 1;
        case T_PAIR: case T_LIST:
            if (a->length != b->length) return 0;
            for (int i = 0; i < a->length; i++)
                if (!equal_values(a->items[i], b->items[i], depth + 1)) return 0;
            return 1;
        case T_MAP:
            if (a->length != b->length) return 0;
            for (int i = 0; i < a->length; i++) {
                int at = map_find(b, a->items[i]);
                if (at < 0 || !equal_values(a->vals[i], b->vals[at], depth + 1)) return 0;
            }
            return 1;
        case T_OBJECT:
            if (!a->flag || !b->flag || strcmp(a->s, b->s) != 0 || a->length != b->length) return 0;
            for (int i = 0; i < a->length; i++) {
                Value **other = object_field(b, a->params[i]);
                if (!other || !equal_values(a->items[i], *other, depth + 1)) return 0;
            }
            return 1;
        default: return 0;
        }
    }
    static int identical(Value *a, Value *b) {
        if (is_scalar(a) && is_scalar(b)) return equal_values(a, b, 0);
        if (a->tag == T_NULL || b->tag == T_NULL) return a->tag == T_NULL && b->tag == T_NULL;
        if (a->tag == T_UNIT || b->tag == T_UNIT) return a->tag == T_UNIT && b->tag == T_UNIT;
        return a == b;
    }
    static int compare_values(Value *a, Value *b) {
        if ((a->tag == T_INT && b->tag == T_INT) || (a->tag == T_LONG && b->tag == T_LONG))
            return a->n < b->n ? -1 : a->n > b->n;
        if (a->tag == T_DOUBLE && b->tag == T_DOUBLE) {
            if (isnan(a->d)) return isnan(b->d) ? 0 : 1;
            if (isnan(b->d)) return -1;
            if (a->d == b->d) return signbit(a->d) == signbit(b->d) ? 0 : signbit(a->d) ? -1 : 1;
            return a->d < b->d ? -1 : 1;
        }
        if (a->tag == T_STRING && b->tag == T_STRING) {
            int c = strcmp(a->s, b->s);
            return c < 0 ? -1 : c > 0;
        }
        return 0;
    }
    static long wrap_op(char op, long a, long b, int is_int) {
        unsigned long x = (unsigned long)a;
        unsigned long y = (unsigned long)b;
        long r = op == '+' ? (long)(x + y) : op == '-' ? (long)(x - y) : (long)(x * y);
        return is_int ? (long)(int)r : r;
    }
    static Value *arithmetic(char op, Value *l, Value *r) {
        if ((l->tag == T_INT && r->tag == T_INT) || (l->tag == T_LONG && r->tag == T_LONG)) {
            int is_int = l->tag == T_INT;
            long result;
            if (op == '/' || op == '%') {
                if (r->n == 0) guest_error("DivisionByZero");
                if (r->n == -1) result = op == '/' ? wrap_op('-', 0, l->n, is_int) : 0;
                else result = op == '/' ? l->n / r->n : l->n % r->n;
            } else {
                result = wrap_op(op, l->n, r->n, is_int);
            }
            return is_int ? vint(result) : vlong(result);
        }
        if (l->tag == T_DOUBLE && r->tag == T_DOUBLE && op != '%') {
            return vdouble(op == '+' ? l->d + r->d : op == '-' ? l->d - r->d : op == '*' ? l->d * r->d : l->d / r->d);
        }
        guest_error("UnassignedRead");
        return NULL;
    }

    /* ---- lists and maps ---- */
    static int map_find(Value *map, Value *key) {
        for (int i = 0; i < map->length; i++)
            if (equal_values(map->items[i], key, 0)) return i;
        return -1;
    }
    static void map_remove_at(Value *map, int at) {
        for (int i = at; i + 1 < map->length; i++) {
            map->items[i] = map->items[i + 1];
            map->vals[i] = map->vals[i + 1];
        }
        map->length--;
    }
    static void map_append(Value *map, Value *key, Value *value) {
        map->items = realloc(map->items, sizeof(Value *) * (size_t)(map->length + 1));
        map->vals = realloc(map->vals, sizeof(Value *) * (size_t)(map->length + 1));
        if (!map->items || !map->vals) host_fault("out of memory", NULL);
        map->items[map->length] = key;
        map->vals[map->length++] = value;
    }
    /* mapOf(k to v, ...): a repeated key keeps its first place and takes the last value. */
    static void map_store(Value *map, Value *key, Value *value) {
        int at = map_find(map, key);
        if (at >= 0) map->vals[at] = value;
        else map_append(map, key, value);
    }
    /* put and index writes move a rewritten key to the end, as the direct engine does. */
    static void map_put(Value *map, Value *key, Value *value) {
        int at = map_find(map, key);
        if (at >= 0) map_remove_at(map, at);
        map_append(map, key, value);
    }
    static long position(Value *index) {
        if (index->tag == T_INT || index->tag == T_LONG) return index->n;
        guest_error("UnassignedRead");
        return 0;
    }
    static Value *list_element(Value *list, Value *index) {
        long at = position(index);
        if (at < 0 || at >= list->length) guest_error("IndexOutOfBounds");
        return list->items[at];
    }
    static Value *list_slice(Value *list, long from, long to) {
        if (from < 0) from = 0;
        if (from > list->length) from = list->length;
        if (to > list->length) to = list->length;
        if (to < from) to = from;
        Value *out = vlist((int)(to - from), list->items + from);
        out->flag = list->flag;
        return out;
    }
    static int truth(Value *v) {
        if (v && v->tag == T_BOOL) return v->b;
        guest_error("UnassignedRead");
        return 0;
    }
    static Value *list_plus(Value *list, Value *argument) {
        if (list->flag) {
            Value *out = vlist(list->length, list->items);
            out->flag = 1;
            for (int i = 0; i < list->length; i++)
                if (equal_values(list->items[i], argument, 0)) return out;
            list_append(out, argument);
            return out;
        }
        int extra = argument->tag == T_LIST ? argument->length : 1;
        Value *out = vlist(list->length + extra, list->items);
        if (argument->tag == T_LIST) for (int i = 0; i < extra; i++) out->items[list->length + i] = argument->items[i];
        else out->items[list->length] = argument;
        return out;
    }
    static Value *map_plus(Value *map, Value *pair) {
        if (pair->tag != T_PAIR) guest_error("UnassignedRead");
        Value *out = vmap(0);
        for (int i = 0; i < map->length; i++) map_append(out, map->items[i], map->vals[i]);
        map_store(out, pair->items[0], pair->items[1]);
        return out;
    }

    /* ---- member access ---- */
    static Value *property(Value *receiver, const char *name) {
        if (receiver->tag == T_LIST && strcmp(name, "size") == 0) return vint(receiver->length);
        if (receiver->tag == T_STRING && strcmp(name, "length") == 0) return vint((long)strlen(receiver->s));
        if (receiver->tag == T_PAIR) {
            if (strcmp(name, "first") == 0) return receiver->items[0];
            if (strcmp(name, "second") == 0) return receiver->items[1];
        }
        if (receiver->tag == T_MAP) {
            if (strcmp(name, "size") == 0) return vint(receiver->length);
            if (strcmp(name, "keys") == 0) { Value *out = vlist(receiver->length, receiver->items); out->flag = 1; return out; }
            if (strcmp(name, "values") == 0) { Value *out = vlist(receiver->length, receiver->vals); return out; }
        }
        if (receiver->tag == T_OBJECT) {
            Value **field = object_field(receiver, name);
            if (field) return *field;
        }
        guest_error("UnassignedRead");
        return NULL;
    }
    static Value *convert(Value *v, const char *name) {
        if (strcmp(name, "toInt") == 0) {
            if (v->tag == T_INT) return v;
            if (v->tag == T_LONG) return vint(v->n);
            if (v->tag == T_DOUBLE) {
                if (isnan(v->d)) return vint(0);
                if (v->d >= 2147483647.0) return vint(2147483647L);
                if (v->d <= -2147483648.0) return vint(-2147483647L - 1);
                return vint((long)v->d);
            }
        } else if (strcmp(name, "toLong") == 0) {
            if (v->tag == T_LONG) return v;
            if (v->tag == T_INT) return vlong(v->n);
            if (v->tag == T_DOUBLE) {
                if (isnan(v->d)) return vlong(0);
                if (v->d >= 9223372036854775807.0) return vlong(LONG_MAX);
                if (v->d <= -9223372036854775808.0) return vlong(LONG_MIN);
                return vlong((long)v->d);
            }
        } else if (strcmp(name, "toDouble") == 0) {
            if (v->tag == T_DOUBLE) return v;
            if (v->tag == T_INT || v->tag == T_LONG) return vdouble((double)v->n);
        }
        guest_error("UnassignedRead");
        return NULL;
    }
    static void require_mutable(Value *v) { if (!v->mut) guest_error("UnassignedRead"); }
    /* A member call on a built-in value: the receiver's own operations. */
    static Value *member(Value *receiver, const char *name, int argc, Value **argv) {
        if (strcmp(name, "toInt") == 0 || strcmp(name, "toLong") == 0 || strcmp(name, "toDouble") == 0) return convert(receiver, name);
        if (receiver->tag == T_STRING) {
            char *end;
            if (strcmp(name, "toLongOrNull") == 0) {
                long n = strtol(receiver->s, &end, 10);
                return (*receiver->s && !*end) ? vlong(n) : &NULL_V;
            }
            if (strcmp(name, "toDoubleOrNull") == 0) {
                double d = strtod(receiver->s, &end);
                return (*receiver->s && !*end) ? vdouble(d) : &NULL_V;
            }
        }
        if (receiver->tag == T_LIST) {
            if (strcmp(name, "isEmpty") == 0) return vbool(receiver->length == 0);
            if (strcmp(name, "contains") == 0) {
                for (int i = 0; i < receiver->length; i++)
                    if (equal_values(receiver->items[i], argv[0], 0)) return &TRUE_V;
                return &FALSE_V;
            }
            if (strcmp(name, "firstOrNull") == 0) return receiver->length ? receiver->items[0] : &NULL_V;
            if (strcmp(name, "get") == 0) return list_element(receiver, argv[0]);
            if (strcmp(name, "plus") == 0) return list_plus(receiver, argv[0]);
            if (strcmp(name, "drop") == 0) return list_slice(receiver, position(argv[0]), receiver->length);
            if (strcmp(name, "take") == 0) return list_slice(receiver, 0, position(argv[0]));
            if (strcmp(name, "toList") == 0) return vlist(receiver->length, receiver->items);
            if (strcmp(name, "add") == 0) { require_mutable(receiver); list_append(receiver, argv[0]); return &TRUE_V; }
            if (strcmp(name, "set") == 0) {
                require_mutable(receiver);
                long at = position(argv[0]);
                if (at < 0 || at >= receiver->length) guest_error("IndexOutOfBounds");
                receiver->items[at] = argv[1];
                return &UNIT_V;
            }
        }
        if (receiver->tag == T_MAP) {
            if (strcmp(name, "isEmpty") == 0) return vbool(receiver->length == 0);
            if (strcmp(name, "containsKey") == 0) return vbool(map_find(receiver, argv[0]) >= 0);
            if (strcmp(name, "get") == 0) { int at = map_find(receiver, argv[0]); return at >= 0 ? receiver->vals[at] : &NULL_V; }
            if (strcmp(name, "plus") == 0) return map_plus(receiver, argv[0]);
            if (strcmp(name, "put") == 0) {
                require_mutable(receiver);
                int at = map_find(receiver, argv[0]);
                Value *previous = at >= 0 ? receiver->vals[at] : &NULL_V;
                map_put(receiver, argv[0], argv[1]);
                return previous;
            }
            if (strcmp(name, "remove") == 0) {
                int at = map_find(receiver, argv[0]);
                if (at < 0) return &NULL_V;
                require_mutable(receiver);
                Value *previous = receiver->vals[at];
                map_remove_at(receiver, at);
                return previous;
            }
        }
        (void)argc;
        host_fault("unsupported member", name);
        return NULL;
    }
    static Value *native_fn(const char *name) {
        Value *v = alloc_value(T_NATIVE);
        v->s = copy_text(name);
        return v;
    }
    /* Applies a member function value to its argument list: receiver first. */
    static Value *apply_native(Value *fn, Value *argl) {
        argl = as_list(argl);
        if (argl->length < 1) guest_error("ShapeFault");
        Value *receiver = argl->items[0];
        if (fn->params) {
            /* a data class `copy`: the named properties are replaced, in order */
            if (receiver->tag != T_OBJECT) guest_error("ShapeFault");
            Value *out = vobject(receiver->s, receiver->flag, receiver->length);
            for (int i = 0; i < receiver->length; i++) { out->params[i] = receiver->params[i]; out->items[i] = receiver->items[i]; }
            for (int i = 0; i < fn->length; i++) {
                Value **field = object_field(out, fn->params[i]);
                if (!field || i + 1 >= argl->length) guest_error("ShapeFault");
                *field = argl->items[i + 1];
            }
            return out;
        }
        return member(receiver, fn->s, argl->length - 1, argl->items + 1);
    }

    /* ---- type tests ---- */
    static int type_test(const char *name, Value *v) {
        size_t n = strlen(name);
        int nullable = n > 0 && name[n - 1] == '?';
        if (v->tag == T_NULL && nullable) return 1;
        char core[128];
        if (n - (size_t)nullable >= sizeof core) host_fault("type name too long", name);
        memcpy(core, name, n - (size_t)nullable);
        core[n - (size_t)nullable] = '\0';
        if (strcmp(core, "Int") == 0) return v->tag == T_INT;
        if (strcmp(core, "Long") == 0) return v->tag == T_LONG;
        if (strcmp(core, "Double") == 0) return v->tag == T_DOUBLE;
        if (strcmp(core, "Boolean") == 0) return v->tag == T_BOOL;
        if (strcmp(core, "String") == 0) return v->tag == T_STRING;
        if (strcmp(core, "List") == 0 || strcmp(core, "Collection") == 0) return v->tag == T_LIST;
        if (strcmp(core, "MutableList") == 0) return v->tag == T_LIST && v->mut;
        if (strcmp(core, "Set") == 0) return v->tag == T_LIST && v->flag;
        if (strcmp(core, "Map") == 0 || strcmp(core, "MutableMap") == 0) return v->tag == T_MAP;
        if (strcmp(core, "Pair") == 0) return v->tag == T_PAIR;
        if (strcmp(core, "Function") == 0) return v->tag == T_PROC || v->tag == T_NATIVE;
        if (strcmp(core, "Nothing") == 0) return 0;
        if (strcmp(core, "Unit") == 0) return v->tag == T_UNIT;
        if (strcmp(core, "Null") == 0) return v->tag == T_NULL;
        return v->tag == T_OBJECT && strcmp(v->s, core) == 0;
    }

    /* ---- the machine's registers and stack ---- */
    static int R_flag;
    static Value **stack_v;
    static int sp_v, stack_cap;
    static void spush(Value *v) {
        if (sp_v == stack_cap) {
            stack_cap = stack_cap ? stack_cap * 2 : 4096;
            stack_v = realloc(stack_v, sizeof(Value *) * (size_t)stack_cap);
            if (!stack_v) host_fault("out of memory", NULL);
        }
        stack_v[sp_v++] = v;
    }
    static Value *spop_v(void) {
        if (sp_v <= 0) host_fault("restore past the stack bottom", NULL);
        return stack_v[--sp_v];
    }
    static int is_true(Value *v) { return v && v->tag == T_BOOL && v->b; }

    /* ---- procedures, lookups, construction ---- */
    static Value *bind_proc(Value *proc, Value *args) {
        args = as_list(args);
        if (!proc || proc->tag != T_PROC) guest_error("ShapeFault");
        int extra = proc->bound ? 1 : 0;
        if (args->length + extra != proc->arity) guest_error("ShapeFault");
        Env *env = child(proc->env ? proc->env : globals());
        if (proc->bound) define(env, proc->params[0], proc->bound);
        for (int i = 0; i < args->length; i++) define(env, proc->params[i + extra], args->items[i]);
        return venv(env);
    }
    static Value *lookup_compiled(Value *name, Value *address, Value *environment) {
        Env *env = as_env(environment);
        address = as_list(address);
        if (address->length != 2) host_fault("invalid lexical address", NULL);
        long distance = address->items[0]->n;
        if (distance < 0) {
            Value **slot = lookup_slot(env, name->s);
            if (!slot) guest_error("UnassignedRead");
            return *slot;
        }
        while (distance-- > 0 && env) env = env->outer;
        long at = address->items[1]->n;
        if (!env || at < 0 || at >= env->count || strcmp(env->names[at], name->s) != 0) guest_error("UnassignedRead");
        return env->slots[at];
    }
    static Value *construct(Value *class_name, Value *names, Value *values, Value *argument_names) {
        if (names->length != values->length || argument_names->length != values->length) guest_error("ShapeFault");
        Value *v = vobject(class_name->s, class_structural(class_name->s), names->length);
        for (int i = 0; i < names->length; i++) {
            int source = i;
            for (int j = 0; j < argument_names->length; j++)
                if (strcmp(argument_names->items[j]->s, names->items[i]->s) == 0) { source = j; break; }
            v->params[i] = names->items[i]->s;
            v->items[i] = values->items[source];
        }
        return v;
    }
    static Value *destructure(Value *source, long count) {
        Value **parts = source->tag == T_OBJECT || source->tag == T_LIST || source->tag == T_PAIR ? source->items : NULL;
        if (!parts || source->length < count) guest_error("ShapeFault");
        return vlist((int)count, parts);
    }
    static Value *capture(Value *proc, Value *environment) {
        if (!proc || proc->tag != T_PROC) guest_error("ShapeFault");
        Value *copy = alloc_value(T_PROC);
        *copy = *proc;
        copy->env = as_env(environment);
        return copy;
    }
    /* compiled-method-proc: the receiver's own method when the class has one,
       else a member function value applied later by compiled-apply-fn. */
    static Value *method_value(Value *args, Value *name, Value *argument_names) {
        args = as_list(args);
        if (args->length < 1) guest_error("ShapeFault");
        Value *found = method_proc(args->items[0], name->s);
        if (found) return found;
        if (strcmp(name->s, "copy") == 0 && args->items[0]->tag == T_OBJECT && args->items[0]->flag) {
            Value *fn = native_fn("copy");
            fn->length = argument_names->length;
            fn->params = allocate(sizeof(char *) * (size_t)(fn->length ? fn->length : 1));
            for (int i = 0; i < fn->length; i++) fn->params[i] = argument_names->items[i]->s;
            return fn;
        }
        return native_fn(name->s);
    }

    /* ---- collections: the carrier of map, filter, fold, any, all ---- */
    static Value *collection_start(Value *receiver, Value *kind, Value *arguments) {
        if (receiver->tag != T_LIST || kind->tag != T_STRING || arguments->tag != T_LIST) guest_error("ShapeFault");
        int folding = strcmp(kind->s, "fold") == 0;
        if (arguments->length != (folding ? 2 : 1)) guest_error("ShapeFault");
        Value *state = alloc_value(T_COLLECTION);
        state->items = receiver->items;
        state->length = receiver->length;
        state->s = kind->s;
        state->bound = arguments->items[arguments->length - 1];
        if (strcmp(kind->s, "map") == 0 || strcmp(kind->s, "filter") == 0) {
            state->acc = vlist(0, NULL);
            state->acc->flag = strcmp(kind->s, "filter") == 0 && receiver->flag;
        } else if (folding) state->acc = arguments->items[0];
        else if (strcmp(kind->s, "any") == 0) state->acc = &FALSE_V;
        else if (strcmp(kind->s, "all") == 0) state->acc = &TRUE_V;
        else guest_error("ShapeFault");
        return state;
    }
    static int collection_done(Value *state) {
        if (state->tag != T_COLLECTION) guest_error("ShapeFault");
        if (strcmp(state->s, "any") == 0 && state->acc->b) return 1;
        if (strcmp(state->s, "all") == 0 && !state->acc->b) return 1;
        return state->n >= state->length;
    }
    static Value *collection_args(Value *state) {
        Value *item = state->items[state->n];
        if (strcmp(state->s, "fold") == 0) return vlist(2, (Value *[]){state->acc, item});
        return vlist(1, (Value *[]){item});
    }
    static Value *collection_step(Value *state, Value *value) {
        Value *item = state->items[state->n];
        if (strcmp(state->s, "map") == 0) list_append(state->acc, value);
        else if (strcmp(state->s, "filter") == 0) { if (truth(value)) list_append(state->acc, item); }
        else if (strcmp(state->s, "fold") == 0) state->acc = value;
        else state->acc = vbool(truth(value));
        state->n++;
        return state;
    }

    /* ---- library functions of the admitted surface ---- */
    static void write_text(Value *v, int newline) {
        fputs(render_value(v)->s, stdout);
        if (newline) putchar('\n');
    }
    static Value *library(const char *name, Value *args) {
        Value **a = args->items;
        int n = args->length;
        if (strcmp(name, "print") == 0 || strcmp(name, "println") == 0) {
            int newline = strcmp(name, "println") == 0;
            if (n == 0 && newline) { putchar('\n'); return &UNIT_V; }
            if (n != 1) guest_error("ShapeFault");
            write_text(a[0], newline);
            return &UNIT_V;
        }
        if (strcmp(name, "listOf") == 0 || strcmp(name, "mutableListOf") == 0) {
            Value *v = vlist(n, a);
            v->mut = name[0] == 'm';
            return v;
        }
        if (strcmp(name, "emptyList") == 0) return vlist(0, NULL);
        if (strcmp(name, "setOf") == 0) {
            Value *v = vlist(0, NULL);
            v->flag = 1;
            for (int i = 0; i < n; i++) {
                int seen = 0;
                for (int j = 0; j < v->length && !seen; j++) seen = equal_values(v->items[j], a[i], 0);
                if (!seen) list_append(v, a[i]);
            }
            return v;
        }
        if (strcmp(name, "emptySet") == 0) { Value *v = vlist(0, NULL); v->flag = 1; return v; }
        if (strcmp(name, "mapOf") == 0 || strcmp(name, "mutableMapOf") == 0) {
            Value *v = vmap(name[0] == 'm');
            for (int i = 0; i < n; i++) {
                if (a[i]->tag != T_PAIR) guest_error("UnassignedRead");
                map_store(v, a[i]->items[0], a[i]->items[1]);
            }
            return v;
        }
        if (strcmp(name, "emptyMap") == 0) return vmap(0);
        if (strcmp(name, "abs") == 0 && n == 1) {
            if (a[0]->tag == T_INT) return vint(a[0]->n < 0 ? -a[0]->n : a[0]->n);
            if (a[0]->tag == T_LONG) return vlong(a[0]->n < 0 ? (long)(0ul - (unsigned long)a[0]->n) : a[0]->n);
            if (a[0]->tag == T_DOUBLE) return vdouble(fabs(a[0]->d));
        }
        if ((strcmp(name, "min") == 0 || strcmp(name, "max") == 0) && n == 2 && a[0]->tag == a[1]->tag) {
            if (a[0]->tag == T_DOUBLE && (isnan(a[0]->d) || isnan(a[1]->d))) return vdouble(NAN);
            int pick_first = strcmp(name, "min") == 0 ? compare_values(a[0], a[1]) <= 0 : compare_values(a[0], a[1]) >= 0;
            return pick_first ? a[0] : a[1];
        }
        if (strcmp(name, "addExact") == 0 || strcmp(name, "subtractExact") == 0 || strcmp(name, "multiplyExact") == 0) {
            if (n == 2 && ((a[0]->tag == T_INT && a[1]->tag == T_INT) || (a[0]->tag == T_LONG && a[1]->tag == T_LONG))) {
                int is_int = a[0]->tag == T_INT;
                long r;
                int overflow = name[0] == 'a' ? __builtin_add_overflow(a[0]->n, a[1]->n, &r)
                             : name[0] == 's' ? __builtin_sub_overflow(a[0]->n, a[1]->n, &r)
                                              : __builtin_mul_overflow(a[0]->n, a[1]->n, &r);
                if (overflow || (is_int && r != (long)(int)r)) guest_error("Overflow");
                return is_int ? vint(r) : vlong(r);
            }
        }
        if (strcmp(name, "negateExact") == 0 && n == 1 && (a[0]->tag == T_INT || a[0]->tag == T_LONG)) {
            int is_int = a[0]->tag == T_INT;
            if (is_int ? a[0]->n == INT_MIN : a[0]->n == LONG_MIN) guest_error("Overflow");
            return is_int ? vint(-a[0]->n) : vlong(-a[0]->n);
        }
        host_fault("unsupported library function", name);
        return NULL;
    }
    static Value *binary(const char *op, Value *l, Value *r);
    static Value *unary(const char *op, Value *v);
    static int is_binary_operator(const char *name) {
        static const char *operators[] = { "to", "+", "-", "*", "/", "%", "<", "<=", ">", ">=", "==", "!=", "===", "!==" };
        for (size_t i = 0; i < sizeof operators / sizeof operators[0]; i++)
            if (strcmp(name, operators[i]) == 0) return 1;
        return 0;
    }
    /* compiled-primitive: an operator applied as a value, else a library function. */
    static Value *primitive(Value *name, Value *args) {
        args = as_list(args);
        if (args->length == 2 && is_binary_operator(name->s)) return binary(name->s, args->items[0], args->items[1]);
        if (args->length == 1 && (strcmp(name->s, "-") == 0 || strcmp(name->s, "!") == 0)) return unary(name->s, args->items[0]);
        return library(name->s, args);
    }

    /* ---- the operation dispatch ---- */
    static Value *binary(const char *op, Value *l, Value *r) {
        if (strcmp(op, "to") == 0) return vpair(l, r);
        if (strcmp(op, "==") == 0) return vbool(equal_values(l, r, 0));
        if (strcmp(op, "!=") == 0) return vbool(!equal_values(l, r, 0));
        if (strcmp(op, "===") == 0) return vbool(identical(l, r));
        if (strcmp(op, "!==") == 0) return vbool(!identical(l, r));
        if (strcmp(op, "<") == 0) return vbool(compare_values(l, r) < 0);
        if (strcmp(op, "<=") == 0) return vbool(compare_values(l, r) <= 0);
        if (strcmp(op, ">") == 0) return vbool(compare_values(l, r) > 0);
        if (strcmp(op, ">=") == 0) return vbool(compare_values(l, r) >= 0);
        if (strcmp(op, "+") == 0) {
            if (l->tag == T_STRING && r->tag == T_STRING) {
                size_t a = strlen(l->s), b = strlen(r->s);
                char *joined = allocate(a + b + 1);
                memcpy(joined, l->s, a);
                memcpy(joined + a, r->s, b);
                Value *v = alloc_value(T_STRING);
                v->s = joined;
                return v;
            }
            if (l->tag == T_LIST) return list_plus(l, r);
            if (l->tag == T_MAP && r->tag == T_PAIR) return map_plus(l, r);
            return arithmetic('+', l, r);
        }
        if (strcmp(op, "-") == 0 || strcmp(op, "*") == 0 || strcmp(op, "/") == 0 || strcmp(op, "%") == 0) return arithmetic(op[0], l, r);
        guest_error("UnassignedRead");
        return NULL;
    }
    static Value *unary(const char *op, Value *v) {
        if (strcmp(op, "!") == 0) return vbool(!truth(v));
        if (v->tag == T_INT) return vint((long)(0ul - (unsigned long)v->n));
        if (v->tag == T_LONG) return vlong((long)(0ul - (unsigned long)v->n));
        if (v->tag == T_DOUBLE) return vdouble(-v->d);
        guest_error("UnassignedRead");
        return NULL;
    }
    static Value *machine_op(const char *name, Value *a1, Value *a2, Value *a3, Value *a4) {
        if (strcmp(name, "compiled-lookup") == 0) return lookup_compiled(a1, a2, a3);
        if (strcmp(name, "adjoin-arg") == 0 || strcmp(name, "append-arg") == 0) {
            a2 = as_list(a2);
            Value *result = vlist(a2->length + 1, NULL);
            if (strcmp(name, "adjoin-arg") == 0) {
                result->items[0] = a1;
                for (int i = 0; i < a2->length; i++) result->items[i + 1] = a2->items[i];
            } else {
                for (int i = 0; i < a2->length; i++) result->items[i] = a2->items[i];
                result->items[a2->length] = a1;
            }
            return result;
        }
        if (strcmp(name, "compiled-const") == 0) return a1;
        if (strcmp(name, "compiled-empty-args") == 0) return vlist(0, NULL);
        if (strcmp(name, "compiled-is-interpreted") == 0) return vbool(a1->tag == T_NATIVE);
        if (strcmp(name, "procedure-entry") == 0) {
            if (!a1 || a1->tag != T_PROC || !a1->entry) guest_error("ShapeFault");
            return (Value *)a1->entry;
        }
        if (strcmp(name, "compiled-bind") == 0) return bind_proc(a1, a2);
        if (strcmp(name, "is-true") == 0) return vbool(truth(a1));
        if (strcmp(name, "compiled-property") == 0) {
            if (a1->tag == T_OBJECT) {
                Value **field = object_field(a1, a2->s);
                if (field) return *field;
                Value *method = method_proc(a1, a2->s);
                if (method) { method->bound = a1; return method; }
            }
            return property(a1, a2->s);
        }
        if (strcmp(name, "compiled-is") == 0) return vbool(type_test(a1->s, a2));
        if (strcmp(name, "compiled-binary") == 0) return binary(a1->s, a2, a3);
        if (strcmp(name, "compiled-unary") == 0) return unary(a1->s, a2);
        if (strcmp(name, "compiled-equal") == 0) return vbool(equal_values(a1, a2, 0));
        if (strcmp(name, "compiled-index") == 0) {
            if (a1->tag == T_LIST && a2->tag == T_INT) return list_element(a1, a2);
            if (a1->tag == T_MAP) { int at = map_find(a1, a2); return at >= 0 ? a1->vals[at] : &NULL_V; }
            guest_error("ShapeFault");
        }
        if (strcmp(name, "compiled-method-proc") == 0) return method_value(a1, a2, a3);
        if (strcmp(name, "compiled-apply-fn") == 0) {
            if (!a1 || a1->tag != T_NATIVE) guest_error("ShapeFault");
            return apply_native(a1, a2);
        }
        if (strcmp(name, "compiled-construct") == 0) return construct(a1, a2, a3, a4);
        if (strcmp(name, "compiled-primitive") == 0) return primitive(a1, a2);
        if (strcmp(name, "compiled-pair-args") == 0) return vlist(2, (Value *[]){a1, a2});
        if (strcmp(name, "compiled-singleton") == 0) return vlist(1, (Value *[]){a1});
        if (strcmp(name, "compiled-render") == 0) return render_value(a1);
        if (strcmp(name, "compiled-concat") == 0) {
            const char *left = text_of(a1);
            const char *right = text_of(a2);
            size_t a = strlen(left), b = strlen(right);
            char *joined = allocate(a + b + 1);
            memcpy(joined, left, a);
            memcpy(joined + a, right, b);
            Value *v = alloc_value(T_STRING);
            v->s = joined;
            return v;
        }
        if (strcmp(name, "declare-local") == 0) { define(as_env(a3), a1->s, a2); return &UNIT_V; }
        if (strcmp(name, "compiled-assign") == 0) {
            Value **slot = lookup_slot(as_env(a3), a1->s);
            if (!slot) guest_error("UnassignedRead");
            *slot = a2;
            return a2;
        }
        if (strcmp(name, "child-env") == 0) return venv(child(as_env(a1)));
        if (strcmp(name, "compiled-globals") == 0 || strcmp(name, "session-root") == 0) return venv(globals());
        if (strcmp(name, "compiled-unit") == 0) return &UNIT_V;
        if (strcmp(name, "compiled-true") == 0) return &TRUE_V;
        if (strcmp(name, "compiled-null") == 0) return &NULL_V;
        if (strcmp(name, "compiled-is-null") == 0) return vbool(a1->tag == T_NULL);
        if (strcmp(name, "compiled-is-procedure") == 0) return vbool(a1 && a1->tag == T_PROC);
        if (strcmp(name, "compiled-write-index") == 0) {
            if (a1->tag == T_LIST) {
                require_mutable(a1);
                long at = position(a2);
                if (at < 0 || at >= a1->length) guest_error("IndexOutOfBounds");
                a1->items[at] = a3;
            } else if (a1->tag == T_MAP) {
                require_mutable(a1);
                map_put(a1, a2, a3);
            } else guest_error("UnassignedRead");
            return a3;
        }
        if (strcmp(name, "compiled-write-property") == 0) {
            Value **field = a1->tag == T_OBJECT && a2->tag == T_STRING ? object_field(a1, a2->s) : NULL;
            if (!field) guest_error("ShapeFault");
            *field = a3;
            return a3;
        }
        if (strcmp(name, "compiled-destructure") == 0) return destructure(a1, a2->n);
        if (strcmp(name, "compiled-capture") == 0) return capture(a1, a2);
        if (strcmp(name, "collection-start") == 0) return collection_start(a1, a2, a3);
        if (strcmp(name, "collection-done") == 0) return vbool(collection_done(a1));
        if (strcmp(name, "collection-args") == 0) return collection_args(a1);
        if (strcmp(name, "collection-callback") == 0) return a1->bound;
        if (strcmp(name, "collection-step") == 0) return collection_step(a1, a2);
        if (strcmp(name, "collection-result") == 0) return a1->acc;
        if (strcmp(name, "stack-peek") == 0) {
            if (sp_v <= 0) host_fault("peek at the stack bottom", NULL);
            return stack_v[sp_v - 1];
        }
        if (strcmp(name, "error-value") == 0) guest_error("ShapeFault");
        host_fault("unsupported compiled operation", name);
        return NULL;
    }

    """.trimIndent()

/** The classes, objects, procedures, and methods of the program, registered
 *  at the head of the emitted function. The order mirrors the compiler's
 *  global frame -- objects, then procedures -- because the compiler's
 *  lexical addresses count slots in that order. */
private fun registrationsC(
    checked: CheckedProgram,
    instructions: List<Stmt>,
): List<String> {
    val labels = instructions.filterIsInstance<Label>().map { it.name }

    fun labelOf(pattern: Regex): String =
        labels.firstOrNull { pattern.matches(it) } ?: error("the compiled entry ${pattern.pattern} is missing")
    val declarations = checked.syntax.declarations
    val lines = mutableListOf<String>()
    for (declaration in declarations) {
        when (declaration) {
            is DataClass -> {
                lines.add("register_class(\"${cEscape(declaration.name)}\", 1);")
            }

            is PlainClass -> {
                lines.add("register_class(\"${cEscape(declaration.name)}\", 0);")
            }

            else -> {}
        }
    }
    for (declaration in declarations.filterIsInstance<DataObject>()) lines.add("register_object(\"${cEscape(declaration.name)}\");")
    for (declaration in declarations.filterIsInstance<FunctionDecl>()) {
        val label = labelOf(Regex("${Regex.escape(declaration.name)}-entry-\\d+"))
        val names = declaration.parameters.map { it.name }
        lines.add("register_proc(\"${cEscape(declaration.name)}\", (void*)&&${cIdent(label)}, ${names.size}, ${paramsC(names)});")
    }
    for (declaration in declarations.filterIsInstance<PlainClass>()) {
        for (method in declaration.methods) {
            val label = labelOf(Regex("${Regex.escape(declaration.name)}-${Regex.escape(method.name)}-\\d+"))
            val names = listOf("this") + method.parameters.map { it.name }
            val owner = cEscape(declaration.name)
            lines.add(
                "register_method(\"$owner\", \"${cEscape(method.name)}\", (void*)&&${cIdent(label)}, ${names.size}, ${paramsC(names)});",
            )
        }
    }
    return lines
}

/** The emitter's program body: one C function carrying the compiler's
 *  complete instruction sequence from its prologue. */
public fun emitCompiledProgram(checked: CheckedProgram): String {
    val instructions =
        Compiler.compile(checked).fold(
            { error -> error("the compilation failed: $error") },
            { it },
        )
    val registers = registersOf(instructions).let { (writes, reads) -> (writes + reads) - "flag" }.sorted()
    val body = instructions.joinToString(separator = "") { stmtC(it) }
    return buildString {
        appendLine("/* generated by the exercise 5.52 C backend -- artifact text; compile externally */")
        appendLine("static Value ${registers.joinToString(", ") { "*R_${cIdent(it)}" }};")
        appendLine("static void compiled_program(void);")
        appendLine("static void compiled_program(void) {")
        for (line in registrationsC(checked, instructions)) appendLine(line)
        append(body)
        appendLine("finish: ;")
        appendLine("}")
    }
}

/** The reference facility's complete, executable C artifact for the same
 *  checked program, kept distinct from this exercise's emitter. */
public fun referenceCArtifact(checked: CheckedProgram): String = CBackend.emitProgram(checked)

/** The emitted artifact's structural verdicts: the exercise's emitter
 *  really produced a C function over the compiler's labels and
 *  registers, and the reference facility produced its own artifact. */
public fun compiledCVerdicts(source: String): List<String> {
    val checked = admitProgram(source)
    val artifact = emitCompiledProgram(checked)
    val reference = referenceCArtifact(checked)
    val instructions = Compiler.compile(checked).fold({ error -> error("the compilation failed: $error") }, { it })
    val labels = instructions.filterIsInstance<Label>().count()
    val registers = registersOf(instructions)
    return listOf(
        "the emitter produced a C function: ${artifact.contains("static void compiled_program(void)")}",
        "the artifact carries the compilation's labels and registers: ${labels > 0 && registers.first.isNotEmpty()}",
        "the reference closure artifact answers like direct: ${runReferenceC(reference) == outputLines(sicp.ch4.Direct.run(checked))}",
    )
}

/** One finished process: status and both streams, kept separate because
 *  only stdout is a guest result and stderr carries the error category. */
private class ProcessResult(
    val status: Int,
    val stdout: String,
    val stderr: String,
)

/** Runs [command] in [directory] with a real deadline even for a C program
 *  that writes continuously. */
private fun runProcess(
    directory: File,
    command: List<String>,
    label: String,
): ProcessResult {
    val stdout = File(directory, "$label.stdout")
    val stderr = File(directory, "$label.stderr")
    val process =
        ProcessBuilder(command)
            .directory(directory)
            .redirectOutput(stdout)
            .redirectError(stderr)
            .start()
    if (!process.waitFor(300, TimeUnit.SECONDS)) {
        process.destroyForcibly()
        error("$label timed out; stderr: ${stderr.readText()}")
    }
    return ProcessResult(process.exitValue(), stdout.readText(), stderr.readText())
}

/** Runs a command that must succeed and answers its stdout. */
private fun runCCommand(
    directory: File,
    command: List<String>,
    label: String,
): String {
    val result = runProcess(directory, command, label)
    check(result.status == 0) { "$label failed: ${result.stderr}\n${result.stdout}" }
    return result.stdout
}

/** Compiles and executes the complete reference artifact, including its
 *  closure entry point. This is a native execution comparison, not a
 *  signature or text-only assertion. */
private fun runReferenceC(source: String): List<String> {
    val dir = Files.createTempDirectory("sicp_5_52_reference").toFile()
    try {
        File(dir, "reference.c").writeText(source)
        runCCommand(dir, listOf("cc", "-O1", "-o", "reference", "reference.c"), "reference-build")
        val out = runCCommand(dir, listOf("./reference"), "reference-run")
        return out.split("\n").filter { it.isNotEmpty() }
    } finally {
        dir.deleteRecursively()
    }
}

/** What one execution observed: the output lines and the guest error
 *  category, if the run stopped on one. */
private data class Observed(
    val lines: List<String>,
    val category: String?,
) {
    fun describe(): String =
        (if (lines.isEmpty()) "no output" else lines.joinToString(" / ")) + (category?.let { ", error $it" } ?: ", no error")
}

private fun observedOf(result: RunResult): Observed = Observed(outputLines(result), result.error?.category)

/** The emitted program built with `cc -O1` and run: its stdout lines, and
 *  the category when it left with a guest error. A fault of the backend
 *  itself, or any other failure, is a harness bug. */
private fun emittedObservation(checked: CheckedProgram): Observed {
    val entryPoint = "\nint main(void) {\n    compiled_program();\n    return 0;\n}\n"
    val artifact = runtimeC + "\n" + emitCompiledProgram(checked) + entryPoint
    val dir = Files.createTempDirectory("sicp_5_52").toFile()
    try {
        File(dir, "compiled.c").writeText(artifact)
        runCCommand(dir, listOf("cc", "-O1", "-o", "compiled", "compiled.c"), "compiled-build")
        val run = runProcess(dir, listOf("./compiled"), "compiled-run")
        val lines = run.stdout.split("\n").filter { it.isNotEmpty() }
        val category = Regex("guest error: (\\w+)").find(run.stderr)?.groupValues?.get(1)
        val stoppedOnGuestError = run.status == 1 && category != null
        check(run.status == 0 || stoppedOnGuestError) { "compiled-run failed: ${run.stderr}\n${run.stdout}" }
        return Observed(lines, category)
    } finally {
        dir.deleteRecursively()
    }
}

/** The emitted program built with `cc -O1` and run: the exercise's
 *  observable is the C output answering like the direct run of the same
 *  checked source. */
public fun compiledCRuns(source: String): List<String> {
    val checked = admitProgram(source)
    val emitted = emittedObservation(checked)
    val direct = observedOf(Direct.run(checked))
    return emitted.lines + listOfNotNull(emitted.category?.let { "error category: $it" }) +
        "the emitted C answers like the direct run: ${emitted == direct}"
}

/** The comparison the exercise asks for: guest output and error category
 *  from the direct engine, the explicit-control evaluator, the compiled
 *  machine, and the emitted C program, and whether all four agree. */
public fun compiledCAgreement(source: String): List<String> {
    val checked = admitProgram(source)
    val runs =
        listOf(
            "direct" to observedOf(Direct.run(checked)),
            "explicit-control" to observedOf(ExplicitControl.run(checked)),
            "compiled machine" to observedOf(Compiler.compileAndRun(checked)),
            "emitted C" to emittedObservation(checked),
        )
    return runs.map { (name, observed) -> "$name: ${observed.describe()}" } +
        "all four agree: ${runs.map { it.second }.distinct().size == 1}"
}
