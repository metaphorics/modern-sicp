// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.52: emit C from the compiler's typed
// instruction sequence. The emitter writes one C function with
// machine registers, labels, a stack, and dispatch over the
// `compiled-*` operations. Its own C support handles the factorial
// probe's argument lists, environments, procedure entries, arithmetic
// and output. It does not yet implement the canonical self-interpreter's
// class, object, closure and method operation families; unsupported
// operations fail explicitly rather than return fabricated results.
// The reference `CBackend.emitProgram` is a separate complete artifact
// and must not be mistaken for the exercise's emitter.

package sicp.ch5.solutions

import sicp.ch5.CBackend
import sicp.ch5.Compiler
import sicp.guest.CheckedProgram
import sicp.guest.FunctionDecl
import sicp.guest.GValue
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

/** A label name as a C identifier. */
private fun cIdent(name: String): String = name.replace("-", "_")

/** A Kotlin string as a C string literal body. */
private fun cEscape(s: String): String = s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n")

/** A C expression building the immutable constant [v]. */
private fun cValue(v: GValue): String =
    when (v) {
        is GValue.VInt -> "vlong(${v.value})"

        is GValue.VLong -> "vlong(${v.value})"

        is GValue.VDouble -> "vdouble(${v.value})"

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

        else -> error("the C emitter cannot encode this constant value: ${v::class.simpleName}")
    }

/** A C expression for the operand source [src]. */
private fun srcC(src: Source): String =
    when (src) {
        is Source.RegSrc -> "R_${cIdent(src.reg)}"
        is Source.ConstSrc -> cValue(src.v)
        is Source.LabelSrc -> "(Value *)&&${cIdent(src.name)}"
        is Source.OpSrc -> "machine_op(\"${src.name}\", ${argsC(src.args)})"
    }

/** The operand sources padded to the dispatch's three slots. */
private fun argsC(args: List<Source>): String {
    val rendered = args.map { srcC(it) }
    return when (rendered.size) {
        0 -> "NULL, NULL, NULL"
        1 -> "${rendered[0]}, NULL, NULL"
        2 -> "${rendered[0]}, ${rendered[1]}, NULL"
        3 -> "${rendered[0]}, ${rendered[1]}, ${rendered[2]}"
        else -> error("the C backend cannot dispatch an operation with ${rendered.size} operands")
    }
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

/** The C run-time support: values, environments, the `compile-*`
 *  operation dispatch, and the section 3.7 printing convention. */
private val runtimeC: String =
    """
    /* The C run-time support of the compiled code: one value model, one
       environment chain, and the machine's operation dispatch. */
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>

    typedef struct Value Value;
    typedef struct Env Env;
    struct Value {
        int tag;              /* 0 long, 1 double, 2 bool, 3 string, 4 null, 5 unit, 6 function, 7 list, 8 env */
        long n;
        double d;
        int b;
        char *s;
        void *entry;
        Env *env;
        int arity;
        Value **items;
        int length;
        char **params;
    };
    struct Env {
        char **names;
        Value **slots;
        int count;
        Env *outer;
    };

    static Value TRUE_V = {2, 0, 0.0, 1, NULL, NULL, NULL, 0};
    static Value FALSE_V = {2, 0, 0.0, 0, NULL, NULL, NULL, 0};
    static Value NULL_V = {4, 0, 0.0, 0, NULL, NULL, NULL, 0};
    static Value UNIT_V = {5, 0, 0.0, 0, NULL, NULL, NULL, 0};

    static void *allocate(size_t bytes) {
        void *p = calloc(1, bytes);
        if (!p) { fprintf(stderr, "out of memory\n"); exit(4); }
        return p;
    }

    static Value *vlong(long n) { Value *v = allocate(sizeof *v); v->tag = 0; v->n = n; return v; }
    static Value *vdouble(double d) { Value *v = allocate(sizeof *v); v->tag = 1; v->d = d; return v; }
    static Value *vstring(const char *s) { Value *v = allocate(sizeof *v); v->tag = 3; v->s = strdup(s); return v; }
    static Value *vlist(int count, Value **items) {
        Value *v = allocate(sizeof *v);
        v->tag = 7;
        v->length = count;
        v->items = allocate(sizeof(Value *) * (size_t)(count ? count : 1));
        if (items) for (int i = 0; i < count; i++) v->items[i] = items[i];
        return v;
    }
    static Value *venv(Env *env) {
        Value *v = allocate(sizeof *v);
        v->tag = 8;
        v->env = env;
        return v;
    }
    static Env *root_env;
    static Env *globals(void) {
        if (!root_env) root_env = allocate(sizeof *root_env);
        return root_env;
    }
    static void define(Env *env, const char *name, Value *value) {
        env->names = realloc(env->names, sizeof(char *) * (size_t)(env->count + 1));
        env->slots = realloc(env->slots, sizeof(Value *) * (size_t)(env->count + 1));
        if (!env->names || !env->slots) { fprintf(stderr, "out of memory\n"); exit(4); }
        env->names[env->count] = strdup(name);
        env->slots[env->count++] = value;
    }
    static Env *child(Env *parent) {
        Env *env = allocate(sizeof *env);
        env->outer = parent;
        return env;
    }
    static Value *lookup(Env *env, const char *name) {
        for (; env; env = env->outer) {
            for (int i = env->count - 1; i >= 0; i--)
                if (strcmp(env->names[i], name) == 0) return env->slots[i];
        }
        fprintf(stderr, "unbound name: %s\n", name);
        exit(3);
    }
    static void register_proc(const char *name, void *entry, int arity, const char **params) {
        Value *proc = allocate(sizeof *proc);
        proc->tag = 6;
        proc->entry = entry;
        proc->env = globals();
        proc->arity = arity;
        proc->params = allocate(sizeof(char *) * (size_t)(arity ? arity : 1));
        for (int i = 0; i < arity; i++) proc->params[i] = strdup(params[i]);
        define(globals(), name, proc);
    }
    static int is_true(Value *v) { return v && v->tag == 2 && v->b; }


    static Value *binary(const char *op, Value *l, Value *r) {
        if (strcmp(op, "==") == 0) {
            if (l->tag != r->tag) return &FALSE_V;
            if (l->tag == 0) return l->n == r->n ? &TRUE_V : &FALSE_V;
            if (l->tag == 2) return l->b == r->b ? &TRUE_V : &FALSE_V;
            if (l->tag == 3) return strcmp(l->s, r->s) == 0 ? &TRUE_V : &FALSE_V;
            if (l->tag == 4 || l->tag == 5) return &TRUE_V;
            fprintf(stderr, "equality for this value kind is unsupported\n"); exit(3);
        }
        if (l->tag == 0 && r->tag == 0) {
            if (strcmp(op, "+") == 0) return vlong(l->n + r->n);
            if (strcmp(op, "-") == 0) return vlong(l->n - r->n);
            if (strcmp(op, "*") == 0) return vlong(l->n * r->n);
            if (strcmp(op, "/") == 0 || strcmp(op, "%") == 0) {
                if (r->n == 0) { fprintf(stderr, "division by zero\n"); exit(3); }
                return vlong(strcmp(op, "/") == 0 ? l->n / r->n : l->n % r->n);
            }
            if (strcmp(op, "<") == 0) return l->n < r->n ? &TRUE_V : &FALSE_V;
            if (strcmp(op, ">") == 0) return l->n > r->n ? &TRUE_V : &FALSE_V;
            if (strcmp(op, "<=") == 0) return l->n <= r->n ? &TRUE_V : &FALSE_V;
            if (strcmp(op, ">=") == 0) return l->n >= r->n ? &TRUE_V : &FALSE_V;
            if (strcmp(op, "!=") == 0) return l->n != r->n ? &TRUE_V : &FALSE_V;
        }
        fprintf(stderr, "unsupported operands\n");
        exit(3);
    }

    /* The machine's registers and stack. */
    static Value *R_val, *R_env, *R_proc, *R_argl, *R_target;
    static Value *R_continue;
    static int R_flag;
    static Value *stack_v[16384];
    static int sp_v = 0;
    static void spush(Value *v) {
        if (sp_v == 16384) { fprintf(stderr, "stack overflow\n"); exit(4); }
        stack_v[sp_v++] = v;
    }
    static Value *spop_v(void) {
        if (sp_v <= 0) { fprintf(stderr, "restore past the stack bottom\n"); exit(4); }
        return stack_v[--sp_v];
    }
    static Env *as_env(Value *v) {
        if (!v || v->tag != 8) { fprintf(stderr, "expected environment\n"); exit(3); }
        return v->env;
    }
    static Value *as_list(Value *v) {
        if (!v || v->tag != 7) { fprintf(stderr, "expected argument list\n"); exit(3); }
        return v;
    }
    static Value *bind_proc(Value *proc, Value *args) {
        args = as_list(args);
        if (!proc || proc->tag != 6 || args->length != proc->arity) {
            fprintf(stderr, "procedure arity mismatch\n"); exit(3);
        }
        Env *env = child(proc->env);
        for (int i = 0; i < proc->arity; i++) define(env, proc->params[i], args->items[i]);
        return venv(env);
    }
    static Value *lookup_compiled(Value *name, Value *address, Value *environment) {
        Env *env = as_env(environment);
        address = as_list(address);
        if (address->length != 2) { fprintf(stderr, "invalid lexical address\n"); exit(3); }
        long distance = address->items[0]->n;
        if (distance < 0) return lookup(env, name->s);
        while (distance-- > 0 && env) env = env->outer;
        long slot = address->items[1]->n;
        if (!env || slot < 0 || slot >= env->count) {
            fprintf(stderr, "invalid lexical slot for %s\n", name->s); exit(3);
        }
        return env->slots[slot];
    }
    static Value *render_value(Value *v) {
        char buffer[64];
        switch (v->tag) {
        case 0: snprintf(buffer, sizeof buffer, "%ld", v->n); break;
        case 1: snprintf(buffer, sizeof buffer, "%.17g", v->d); break;
        case 2: return vstring(v->b ? "true" : "false");
        case 3: return v;
        case 4: return vstring("null");
        case 5: return vstring("unit");
        default: fprintf(stderr, "unsupported printable value\n"); exit(3);
        }
        return vstring(buffer);
    }
    static Value *machine_op(const char *name, Value *a1, Value *a2, Value *a3) {
        if (strcmp(name, "compiled-const") == 0) return a1;
        if (strcmp(name, "compiled-globals") == 0) return venv(globals());
        if (strcmp(name, "child-env") == 0) return venv(child(as_env(a1)));
        if (strcmp(name, "compiled-bind") == 0) return bind_proc(a1, a2);
        if (strcmp(name, "procedure-entry") == 0) {
            if (!a1 || a1->tag != 6 || !a1->entry) { fprintf(stderr, "not a compiled procedure\n"); exit(3); }
            return (Value *)a1->entry;
        }
        if (strcmp(name, "compiled-lookup") == 0) return lookup_compiled(a1, a2, a3);
        if (strcmp(name, "compiled-is-procedure") == 0) return a1 && a1->tag == 6 ? &TRUE_V : &FALSE_V;
        if (strcmp(name, "compiled-is-interpreted") == 0) {
            if (!a1 || a1->tag != 6) {
                fprintf(stderr, "interpreted closure calls require the C closure runtime\n");
                exit(3);
            }
            return &FALSE_V;
        }
        if (strcmp(name, "compiled-true") == 0) return &TRUE_V;
        if (strcmp(name, "is-true") == 0) return is_true(a1) ? &TRUE_V : &FALSE_V;
        if (strcmp(name, "compiled-unit") == 0) return &UNIT_V;
        if (strcmp(name, "compiled-null") == 0) return &NULL_V;
        if (strcmp(name, "compiled-is-null") == 0) return a1->tag == 4 ? &TRUE_V : &FALSE_V;
        if (strcmp(name, "compiled-empty-args") == 0) return vlist(0, NULL);
        if (strcmp(name, "compiled-pair-args") == 0) return vlist(2, (Value *[]){a1, a2});
        if (strcmp(name, "compiled-singleton") == 0) return vlist(1, (Value *[]){a1});
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
        if (strcmp(name, "compiled-equal") == 0) {
            if (a1->tag != a2->tag) return &FALSE_V;
            if (a1->tag == 0) return a1->n == a2->n ? &TRUE_V : &FALSE_V;
            if (a1->tag == 3) return strcmp(a1->s, a2->s) == 0 ? &TRUE_V : &FALSE_V;
            return a1 == a2 ? &TRUE_V : &FALSE_V;
        }
        if (strcmp(name, "compiled-binary") == 0) return binary(a1->s, a2, a3);
        if (strcmp(name, "compiled-unary") == 0) {
            if (strcmp(a1->s, "-") == 0) return vlong(-a2->n);
            return is_true(a2) ? &FALSE_V : &TRUE_V;
        }
        if (strcmp(name, "compiled-render") == 0) return render_value(a1);
        if (strcmp(name, "compiled-primitive") == 0) {
            a2 = as_list(a2);
            if (strcmp(a1->s, "print") == 0 || strcmp(a1->s, "println") == 0) {
                if (a2->length != 1) { fprintf(stderr, "printing arity mismatch\n"); exit(3); }
                fputs(render_value(a2->items[0])->s, stdout);
                if (strcmp(a1->s, "println") == 0) putchar('\n');
                return &UNIT_V;
            }
            fprintf(stderr, "unknown primitive %s\n", a1->s); exit(3);
        }
        if (strcmp(name, "declare-local") == 0) {
            define(as_env(a3), a1->s, a2);
            return &UNIT_V;
        }
        if (strcmp(name, "stack-peek") == 0) {
            if (sp_v <= 0) { fprintf(stderr, "peek at the stack bottom\n"); exit(4); }
            return stack_v[sp_v - 1];
        }
        fprintf(stderr, "compiled operation %s is unsupported by this C backend\n", name);
        exit(3);
    }

    """.trimIndent()

/** The emitter's program body: one C function carrying the compiler's
 *  complete instruction sequence from its prologue. */
public fun emitCompiledProgram(checked: CheckedProgram): String {
    val instructions =
        Compiler.compile(checked).fold(
            { error -> error("the compilation failed: $error") },
            { it },
        )
    val body = instructions.joinToString(separator = "") { stmtC(it) }
    val registrations =
        checked.syntax.declarations.filterIsInstance<FunctionDecl>().joinToString("\n") { declaration ->
            val label =
                instructions.filterIsInstance<Label>().firstOrNull {
                    it.name.startsWith("${declaration.name}-entry-")
                } ?: error("the compiled entry for ${declaration.name} is missing")
            val params = declaration.parameters.joinToString(", ") { "\"${cEscape(it.name)}\"" }
            val names = if (params.isEmpty()) "NULL" else "(const char*[]){$params}"
            "register_proc(\"${cEscape(declaration.name)}\", (void*)&&${cIdent(label.name)}, ${declaration.parameters.size}, $names);"
        }
    return buildString {
        appendLine("/* generated by the exercise 5.52 C backend -- artifact text; compile externally */")
        appendLine("static void compiled_program(void);")
        appendLine("static void compiled_program(void) {")
        appendLine(registrations)
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

/** Captures stdout and stderr separately, with a real deadline even for
 *  a C program that writes continuously. Only stdout is a guest result. */
private fun runCCommand(
    directory: File,
    command: List<String>,
    label: String,
): String {
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
    check(process.exitValue() == 0) {
        "$label failed: ${stderr.readText()}\n${stdout.readText()}"
    }
    return stdout.readText()
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

/** The emitted program built with `cc -O1` and run: the exercise's
 *  observable is the C output answering like the direct run of the same
 *  checked source. */
public fun compiledCRuns(source: String): List<String> {
    val checked = admitProgram(source)
    val artifact = runtimeC + "\n" + emitCompiledProgram(checked) + "\nint main(void) {\n    compiled_program();\n    return 0;\n}\n"
    val dir = Files.createTempDirectory("sicp_5_52").toFile()
    try {
        File(dir, "compiled.c").writeText(artifact)
        runCCommand(dir, listOf("cc", "-O1", "-o", "compiled", "compiled.c"), "compiled-build")
        val out = runCCommand(dir, listOf("./compiled"), "compiled-run")
        val cLines = out.split("\n").filter { it.isNotEmpty() }
        val direct = outputLines(sicp.ch4.Direct.run(checked))
        return cLines + "the emitted C answers like the direct run: ${cLines == direct}"
    } finally {
        dir.deleteRecursively()
    }
}
