(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.52: the compiler's C backend.  The instruction sequences
    the 5.5 compiler emits are translated one statement at a time into
    the statements of a single C function: registers are globals,
    labels are C labels (dashes become underscores), [continue] and
    procedure entries hold label addresses (the GNU computed-goto
    extension the system compiler accepts), and the machine's
    operations are one C dispatch mirroring the compiled operations
    table.  The run-time support is the object world of 5.50's
    machine: pairs, symbols, the primitive table, and the environment
    procedures behind id symbols.  Compiling the adapted metacircular
    source with this backend produces a Scheme interpreter in C that
    runs its object program; the build uses the system C compiler. *)

module C = Sicp_ch5.Sec_5_5
module Value = Sicp_common.Value

let ( >>= ) = Result.bind

(** [c_ident] is a label name as a C identifier. *)
let c_ident name = String.map (fun c -> if c = '-' then '_' else c) name

(** [runtime_c] is the run-time support: values, the environment-id
    table, the primitive table, the machine operations, and the
    driver. *)
let runtime_c =
  {|/* The C runtime of the compiled evaluator. */
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
static Value *sym(const char *s) { Value *v = calloc(1, sizeof *v); v->tag = 1; v->sym = strdup(s); return v; }
static Value *num(long n) { Value *v = calloc(1, sizeof *v); v->tag = 0; v->i = n; return v; }
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
    if (!strcmp(n, "cadr")) return car(cdr(car(args)));
    if (!strcmp(n, "caddr")) return car(cdr(cdr(car(args))));
    if (!strcmp(n, "cadddr")) return car(cdr(cdr(cdr(car(args)))));
    if (!strcmp(n, "caadr")) return car(car(cdr(car(args))));
    if (!strcmp(n, "cdadr")) return cdr(car(cdr(car(args))));
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
static Value *estack[400000]; static void *lstack[400000]; static int esp = 0;
static void spush(Value *v) { estack[esp] = v; esp++; }
static void spush_label(void *l) { lstack[esp] = l; esp++; }
static Value *spop_v(void) { esp--; return estack[esp]; }
static void *spop_l(void) { esp--; return lstack[esp]; }

/* The compile-time constants, built by the emitted initializer. */
static Value *K[4096];
static Value *const_arg(const char *s) {
    /* names are minted 1-based; the K table is 0-based */
    if (!strncmp(s, "compile-time-constant-", 22)) return K[atoi(s + 22) - 1];
    if (!strcmp(s, "#t")) return &TRUE_V;
    if (!strcmp(s, "#f")) return &FALSE_V;
    if (s[0] >= '0' && s[0] <= '9') return num(atol(s));
    return sym(s);
}
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
        if (w1->tag != 3) { fprintf(stderr, "apply of a non-primitive\\n"); exit(4); }
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
    names = names->tag == 2 ? names : &NIL_V;
    /* reverse into order */
    { Value *r = &NIL_V; for (Value *p = names; p && p->tag == 2; p = p->cdr) r = pair(p->car, r); names = r; }
    Value *frame = pair(names, args);
    Value *frames = (base_id->tag == 1 && !strcmp(base_id->sym, "the-empty"))
        ? pair(frame, &NIL_V)
        : pair(frame, env_of_id(base_id->sym));
    return env_register(frames);
}
|}
  ^ {|
/* The driver: run the compiled program, print the value. */
extern void compiled_program(void);
static void init_constants(void);
int main(void) {
    TRUE_V.tag = 5; TRUE_V.i = 1;
    FALSE_V.tag = 5; FALSE_V.i = 0;
    NIL_V.tag = 6;
    init_constants();
    /* the top-level environment: one empty frame over the-empty, with
       the primitive table and the booleans bound, as the 5.5.7
       machine's global environment is set up */
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
|}
;;

(** [c_value state v] is a C expression building the compile-time
    constant [v]. *)
let rec c_value state (v : Value.t) : string =
  match Value.view v with
  | Value.Int n -> Printf.sprintf "num(%d)" n
  | Value.Bool b -> Printf.sprintf "boolean(%d)" (if b then 1 else 0)
  | Value.String s -> Printf.sprintf "string_const(\"%s\")" (String.escaped s)
  | Value.Symbol s -> Printf.sprintf "sym(\"%s\")" (String.escaped s)
  | Value.Nil -> "&NIL_V"
  | Value.Pair (a, d) -> Printf.sprintf "pair(%s, %s)" (c_value state a) (c_value state d)
  | _ -> "&NIL_V"
;;

(** [constants_c state] is the initializer: one constant per slot. *)
let constants_c state =
  let bindings = C.registered_constants state in
  let slots =
    String.concat
      ""
      (List.mapi (fun i _ -> Printf.sprintf "  K[%d] = &NIL_V;\n" i) bindings)
  in
  let builds =
    String.concat
      ""
      (List.mapi
         (fun i (_name, v) -> Printf.sprintf "  K[%d] = %s;\n" i (c_value state v))
         bindings)
  in
  let string_helper =
    {|static Value *string_const(const char *s) {
    Value *v = calloc(1, sizeof *v);
    v->tag = 7; v->str = strdup(s);
    return v;
}
|}
  in
  string_helper ^ "static void init_constants(void) {\n" ^ slots ^ builds ^ "}\n"
;;

(** [stmt_c line] is one controller statement as C statements.  The
    statement grammar of the 5.5 compiler: assign, test, branch, goto,
    save, restore, perform over operands [(reg r)], [(const c)], and
    [(label l)]. *)
let stmt_c line =
  let trimmed = String.trim line in
  if trimmed = ""
  then ""
  else if trimmed.[0] <> '('
  then Printf.sprintf "%s: ;\n" (c_ident trimmed)
  else (
    let inner = String.sub trimmed 1 (String.length trimmed - 2) in
    let tokens = String.split_on_char ' ' inner |> List.filter (fun s -> s <> "") in
    let strip name =
      let n = String.length name in
      let a = if n > 0 && name.[0] = '(' then 1 else 0 in
      let b = if n > a && name.[n - 1] = ')' then 1 else 0 in
      String.sub name a (n - a - b)
    in
    let rec operands acc = function
      | k :: v :: rest when k.[0] = '(' ->
        let w =
          match strip k with
          | "reg" -> Some (`Reg (strip v))
          | "const" -> Some (`Const (strip v))
          | "label" -> Some (`Label (strip v))
          | _ -> None
        in
        (match w with
         | Some o -> operands (o :: acc) rest
         | None -> None)
      | [] -> Some (List.rev acc)
      | _ -> None
    in
    let arg_c = function
      | `Reg r -> Printf.sprintf "R_%s" r
      | `Const c -> Printf.sprintf "const_arg(\"%s\")" c
      | `Label l -> Printf.sprintf "&&%s" (c_ident l)
    in
    let args_c = function
      | [] -> "NULL, NULL, NULL"
      | [ a ] -> Printf.sprintf "%s, NULL, NULL" (arg_c a)
      | [ a; b ] -> Printf.sprintf "%s, %s, NULL" (arg_c a) (arg_c b)
      | [ a; b; c ] -> Printf.sprintf "%s, %s, %s" (arg_c a) (arg_c b) (arg_c c)
      | _ -> "NULL, NULL, NULL"
    in
    (* an op site: the operation name token, then its operands *)
    let op_site = function
      | name :: rest ->
        (match operands [] rest with
         | Some ops -> Some (strip name, ops)
         | None -> None)
      | [] -> None
    in
    let comment () = Printf.sprintf "/* %s */\n" trimmed in
    match tokens with
    | [ "assign"; target; "(label"; l ] ->
      Printf.sprintf "R_%s = &&%s;\n" target (c_ident (strip l))
    | [ "assign"; target; "(const"; v ] ->
      Printf.sprintf "R_%s = const_arg(\"%s\");\n" target (strip v)
    | "assign" :: target :: "(op" :: site ->
      let out = "R_" ^ target in
      (match op_site site with
       | None -> comment ()
       | Some (op, ops) ->
         (match op with
          | "compiled-procedure-entry" ->
            Printf.sprintf "%s = R_proc;\nR_entry = R_proc->entry;\n" out
          | "make-compiled-procedure" ->
            (match ops with
             | [ `Const entry ] | [ `Const entry; `Reg "env" ] ->
               Printf.sprintf
                 "%s = make_compiled_procedure(&&%s, R_env);\n"
                 out
                 (c_ident entry)
             | _ -> Printf.sprintf "%s = make_compiled_procedure(NULL, R_env);\n" out)
          | "extend-environment" ->
            let names =
              List.filter_map
                (function
                  | `Const c -> Some c
                  | _ -> None)
                ops
            in
            Printf.sprintf
              "%s = extend_compile(\"%s\", R_argl, R_env);\n"
              out
              (String.concat " " names)
          | _ -> Printf.sprintf "%s = machine_op(\"%s\", %s);\n" out op (args_c ops)))
    | "test" :: "(op" :: site ->
      (match op_site site with
       | Some ("false?", _) -> "R_flag = !is_true(R_val);\n"
       | Some (op, ops) ->
         Printf.sprintf "R_flag = is_true(machine_op(\"%s\", %s));\n" op (args_c ops)
       | None -> comment ())
    | [ "branch"; "(label"; l ] ->
      Printf.sprintf "if (R_flag) goto %s;\n" (c_ident (strip l))
    | [ "goto"; "(label"; l ] -> Printf.sprintf "goto %s;\n" (c_ident (strip l))
    | [ "goto"; "(reg"; r ] ->
      (match strip r with
       | "continue" -> "goto *R_continue;\n"
       | "val" -> "goto *R_entry;\n"
       | _ -> comment ())
    | [ "save"; r ] -> Printf.sprintf "spush(R_%s);\n" (strip r)
    | [ "restore"; r ] -> Printf.sprintf "R_%s = spop_v();\n" (strip r)
    | "perform" :: "(op" :: site ->
      (match op_site site with
       | Some (op, ops) ->
         Printf.sprintf "(void)machine_op(\"%s\", %s);\n" op (args_c ops)
       | None -> comment ())
    | _ -> comment ())
;;

(** [compile_to_c source] is the whole C file: the runtime, the
    constant initializer, and the compiled program as one function. *)
let compile_to_c source =
  let state = C.new_state () in
  C.compile_block state source
  >>= fun (entry, block) ->
  let body = String.concat "" (List.map stmt_c (String.split_on_char '\n' block)) in
  let head =
    let first =
      String.split_on_char '\n' body
      |> List.find_opt (fun l -> String.trim l <> "")
      |> Option.map String.trim
      |> Option.value ~default:""
    in
    let entry_line = c_ident entry ^ ":" in
    if
      String.length first >= String.length entry_line
      && String.sub first 0 (String.length entry_line) = entry_line
    then ""
    else c_ident entry ^ ": ;\n"
  in
  Ok
    (runtime_c
     ^ "\n"
     ^ constants_c state
     ^ "\nvoid compiled_program(void) {\n"
     ^ "/* the top-level continuation must be a label of this function:\n"
     ^ "   a computed goto cannot cross function boundaries. */\n"
     ^ "if (!R_continue) R_continue = &&finish;\n"
     ^ head
     ^ body
     ^ "\nfinish: ;\n"
     ^ "print_value_pub(R_val);\n"
     ^ "printf(\"\\n\");\n"
     ^ "}\n")
;;

let input_line_opt ic =
  try Some (input_line ic) with
  | End_of_file -> None
;;

(** [build_and_run c_source] writes, builds, and runs the program,
    answering the output. *)
let build_and_run c_source =
  let dir = Filename.temp_dir "sicp_5_52" "" in
  let cfile = Filename.concat dir "compiled.c" in
  let out = Filename.concat dir "compiled" in
  let oc = open_out cfile in
  output_string oc c_source;
  close_out oc;
  let build =
    Printf.sprintf "cd %s && cc -O1 -o compiled compiled.c 2>&1" (Filename.quote dir)
  in
  let ic = Unix.open_process_in build in
  let first_line = input_line_opt ic in
  let status = Unix.close_process_in ic in
  match status, first_line with
  | Unix.WEXITED 0, None ->
    let ic = Unix.open_process_in out in
    let buffer = Buffer.create 256 in
    (try
       while true do
         Buffer.add_string buffer (input_line ic);
         Buffer.add_char buffer '\n'
       done
     with
     | End_of_file -> ());
    ignore (Unix.close_process_in ic);
    Ok (Buffer.contents buffer)
  | _, Some line -> Error (C.Op_failed ("the C backend failed to build: " ^ line))
  | _ -> Error (C.Op_failed "the C backend failed to build")
;;

(** [ex_5_52 ()] compiles the adapted metacircular source to C, builds
    it, and runs it: the object program's factorial answers 120
    through the C-compiled interpreter. *)
let ex_5_52 () =
  let source =
    Sicp_ch5.Metacircular.source ^ "\n(m-eval '(factorial 5) the-global-environment)\n"
  in
  compile_to_c source >>= build_and_run >>= fun output -> Ok [ output ]
;;
