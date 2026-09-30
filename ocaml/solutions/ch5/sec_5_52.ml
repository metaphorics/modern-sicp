(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let unsupported what =
  Error (Eval_error.Invalid_form ("the C backend does not support " ^ what))
;;

let c_string s =
  let b = Buffer.create (String.length s + 2) in
  Buffer.add_char b '"';
  String.iter
    (fun ch ->
       match ch with
       | '"' -> Buffer.add_string b "\\\""
       | '\\' -> Buffer.add_string b "\\\\"
       | ' ' .. '~' -> Buffer.add_char b ch
       | _ -> Buffer.add_string b (Printf.sprintf "\\%03o" (Char.code ch)))
    s;
  Buffer.add_char b '"';
  Buffer.contents b
;;

type emitter =
  { labels : (string, int) Hashtbl.t
  ; globals : Buffer.t
  ; init : Buffer.t
  ; mutable next : int
  }

let fresh em prefix =
  em.next <- em.next + 1;
  Printf.sprintf "%s_%d" prefix em.next
;;

let label em l =
  match Hashtbl.find_opt em.labels l with
  | Some n -> n
  | None ->
    let n = Hashtbl.length em.labels + 1 in
    Hashtbl.replace em.labels l n;
    n
;;

let name_array em names =
  let g = fresh em "names" in
  Printf.bprintf
    em.globals
    "static const char *%s[] = { %s };\n"
    g
    (String.concat
       ", "
       (List.map
          (function
            | Some n -> c_string n
            | None -> "NULL")
          names
        @ [ "NULL" ]));
  g
;;

let scalar_c = function
  | Ast.Int n -> Ok (Printf.sprintf "make_int(%dL)" n)
  | Ast.Bool b -> Ok (Printf.sprintf "make_bool(%d)" (Bool.to_int b))
  | Ast.Unit -> Ok "make_unit()"
  | Ast.String s -> Ok (Printf.sprintf "make_string(%s)" (c_string s))
  | Ast.Float _ -> unsupported "floats"
;;

let all f xs =
  List.fold_right
    (fun x acc ->
       let* rest = acc in
       let* y = f x in
       Ok (y :: rest))
    xs
    (Ok [])
;;

let rec pattern_c p =
  let items kind name ps =
    let* parts = all pattern_c ps in
    Ok
      (Printf.sprintf
         "make_pat(%s, 0, %s, %d, (Pat *[]){ %s })"
         kind
         name
         (List.length ps)
         (String.concat ", " (if parts = [] then [ "NULL" ] else parts)))
  in
  match Ast.view_pattern p with
  | Ast.PWildcard -> Ok "make_pat(PK_WILD, 0, NULL, 0, NULL)"
  | Ast.PVar x -> Ok (Printf.sprintf "make_pat(PK_VAR, 0, %s, 0, NULL)" (c_string x))
  | Ast.PScalar (Ast.Int n) ->
    Ok (Printf.sprintf "make_pat(PK_INT, %dL, NULL, 0, NULL)" n)
  | Ast.PScalar (Ast.Bool b) ->
    Ok (Printf.sprintf "make_pat(PK_BOOL, %d, NULL, 0, NULL)" (Bool.to_int b))
  | Ast.PScalar Ast.Unit -> Ok "make_pat(PK_UNIT, 0, NULL, 0, NULL)"
  | Ast.PScalar (Ast.String s) ->
    Ok (Printf.sprintf "make_pat(PK_STRING, 0, %s, 0, NULL)" (c_string s))
  | Ast.PScalar (Ast.Float _) -> unsupported "float patterns"
  | Ast.PTuple ps -> items "PK_TUPLE" "NULL" ps
  | Ast.PConstruct (c, ps) -> items "PK_CTOR" (c_string c) ps
  | Ast.PNil -> Ok "make_pat(PK_NIL, 0, NULL, 0, NULL)"
  | Ast.PCons (h, t) -> items "PK_CONS" "NULL" [ h; t ]
;;

let binary_code e =
  match Ast.view e with
  | Ast.Arith (Ast.Add, _, _) -> Ok "O_ADD"
  | Ast.Arith (Ast.Sub, _, _) -> Ok "O_SUB"
  | Ast.Arith (Ast.Mul, _, _) -> Ok "O_MUL"
  | Ast.Arith (Ast.Div, _, _) -> Ok "O_DIV"
  | Ast.Arith (Ast.Rem, _, _) -> Ok "O_MOD"
  | Ast.Arith (_, _, _) -> unsupported "float arithmetic"
  | Ast.Compare (Ast.Eq, _, _) -> Ok "O_EQ"
  | Ast.Compare (Ast.Ne, _, _) -> Ok "O_NE"
  | Ast.Compare (Ast.Lt, _, _) -> Ok "O_LT"
  | Ast.Compare (Ast.Le, _, _) -> Ok "O_LE"
  | Ast.Compare (Ast.Gt, _, _) -> Ok "O_GT"
  | Ast.Compare (Ast.Ge, _, _) -> Ok "O_GE"
  | Ast.Concat _ -> Ok "O_CONCAT"
  | Ast.Cons _ -> Ok "O_CONS"
  | Ast.Assign _ -> Ok "O_ASSIGN"
  | _ -> unsupported "this binary operator"
;;

let unary_code e =
  match Ast.view e with
  | Ast.Not _ -> Ok "U_NOT"
  | Ast.Neg _ -> Ok "U_NEG"
  | Ast.Deref _ -> Ok "U_DEREF"
  | Ast.Make_ref _ -> Ok "U_REF"
  | _ -> unsupported "records"
;;

let reg r =
  "R_"
  ^ String.map
      (function
        | '-' -> '_'
        | ch -> ch)
      r
;;

let source em field = function
  | M.Reg r -> Ok (reg r ^ "." ^ field)
  | M.Label_ref l -> Ok (string_of_int (label em l))
  | M.Const (W.Args []) -> Ok "empty_args()"
  | M.Const (W.V v) ->
    (match Value.view v with
     | Value.Int n -> scalar_c (Ast.Int n)
     | Value.Bool b -> scalar_c (Ast.Bool b)
     | Value.Unit -> scalar_c Ast.Unit
     | Value.String s -> scalar_c (Ast.String s)
     | Value.Nil -> Ok "make_nil()"
     | _ -> unsupported ("the constant " ^ Value.to_string v))
  | M.Const w -> unsupported ("the constant " ^ W.word_to_string w)
;;

let expression sources i =
  match List.nth_opt sources i with
  | Some (M.Const (W.Exp e)) -> Ok e
  | _ -> unsupported "an operation without its syntax constant"
;;

(* [operation em op sources] is the C field an operation writes and the C
   expression computing it. *)
let operation em op sources =
  let src field i =
    match List.nth_opt sources i with
    | Some s -> source em field s
    | None -> unsupported (op ^ " with too few operands")
  in
  let call field fmt args =
    let* args = all (fun (f, i) -> src f i) args in
    Ok (field, fmt args)
  in
  let two f a b = f ^ "(" ^ a ^ ", " ^ b ^ ")" in
  match op with
  | "lookup-variable-value" ->
    let* e = expression sources 0 in
    (match Ast.view e with
     | Ast.Var x ->
       let* env = src "env" 1 in
       Ok ("v", Printf.sprintf "lookup(%s, %s)" env (c_string x))
     | _ -> unsupported "lookup of a non-variable")
  | "apply-binary" ->
    let* e = expression sources 0 in
    let* code = binary_code e in
    call
      "v"
      (fun a -> Printf.sprintf "binary(%s, %s)" code (String.concat ", " a))
      [ "v", 1; "v", 2 ]
  | "apply-unary" ->
    let* e = expression sources 0 in
    let* code = unary_code e in
    call "v" (fun a -> Printf.sprintf "unary(%s, %s)" code (List.hd a)) [ "v", 1 ]
  | "build" ->
    let* e = expression sources 0 in
    let* argl = src "args" 1 in
    (match Ast.view e with
     | Ast.Tuple _ ->
       Ok ("v", Printf.sprintf "make_items(T_TUPLE, NULL, %s->n, %s->vs)" argl argl)
     | Ast.Construct (c, _) ->
       Ok
         ( "v"
         , Printf.sprintf "make_items(T_CTOR, %s, %s->n, %s->vs)" (c_string c) argl argl
         )
     | _ -> unsupported "records")
  | "make-compiled-procedure" ->
    let* entry = src "label" 0 in
    let* e = expression sources 1 in
    let* env = src "env" 2 in
    (match Ast.view e with
     | Ast.Fun (ps, _) ->
       let names = name_array em (List.map Option.some ps) in
       Ok
         ( "v"
         , Printf.sprintf "make_compiled(%s, %d, %s, %s)" entry (List.length ps) names env
         )
     | _ -> unsupported "a procedure without parameters")
  | "compiled-procedure-bind" ->
    call
      "env"
      (fun a -> two "compiled_bind" (List.hd a) (List.nth a 1))
      [ "v", 0; "args", 1 ]
  | "adjoin-arg" ->
    call "args" (fun a -> two "adjoin" (List.nth a 1) (List.hd a)) [ "v", 0; "args", 1 ]
  | "let-rec-group" ->
    let* e = expression sources 0 in
    let* env = src "env" 1 in
    (match Ast.view e with
     | Ast.Let (true, bindings, _) ->
       let names = name_array em (List.map (fun (b : Ast.binding) -> b.name) bindings) in
       Ok
         ( "pending"
         , Printf.sprintf "let_rec_group(%d, %s, %s)" (List.length bindings) names env )
     | _ -> unsupported "a recursive group without its syntax")
  | "group-environment" -> call "env" (fun a -> List.hd a ^ "->env") [ "pending", 0 ]
  | "rest-pending" ->
    call "pending" (fun a -> "rest_pending(" ^ List.hd a ^ ")") [ "pending", 0 ]
  | "try-pattern" ->
    (match sources with
     | M.Const (W.Pat p) :: _ ->
       let* pat = pattern_c p in
       let g = fresh em "pattern" in
       Printf.bprintf em.globals "static Pat *%s;\n" g;
       Printf.bprintf em.init "  %s = %s;\n" g pat;
       call
         "env"
         (fun a -> Printf.sprintf "try_pattern(%s, %s, %s)" g (List.hd a) (List.nth a 1))
         [ "v", 1; "env", 2 ]
     | _ -> unsupported "try-pattern without its pattern")
  | "let-environment" ->
    let* e = expression sources 0 in
    (match Ast.view e with
     | Ast.Let (false, bindings, _) ->
       let names = name_array em (List.map (fun (b : Ast.binding) -> b.name) bindings) in
       call
         "env"
         (fun a ->
            Printf.sprintf
              "let_environment(%d, %s, %s, %s)"
              (List.length bindings)
              names
              (List.hd a)
              (List.nth a 1))
         [ "args", 1; "env", 2 ]
     | _ -> unsupported "a top-level binding without its syntax")
  | "last-argument" ->
    call "v" (fun a -> "last_argument(" ^ List.hd a ^ ")") [ "args", 0 ]
  | "apply-primitive-procedure" ->
    call
      "v"
      (fun a -> two "apply_primitive" (List.hd a) (List.nth a 1))
      [ "v", 0; "args", 1 ]
  | "primitive-excess-arguments" ->
    call
      "args"
      (fun a -> two "primitive_excess" (List.hd a) (List.nth a 1))
      [ "v", 0; "args", 1 ]
  | "compiled-excess-arguments" ->
    call
      "args"
      (fun a ->
         Printf.sprintf "drop_args(%s, compiled(%s)->n)" (List.nth a 1) (List.hd a))
      [ "v", 0; "args", 1 ]
  | "compiled-exact-arguments" ->
    call
      "args"
      (fun a ->
         Printf.sprintf "take_args(%s, compiled(%s)->n)" (List.nth a 1) (List.hd a))
      [ "v", 0; "args", 1 ]
  | "compiled-entry" ->
    call "label" (fun a -> "compiled_entry(" ^ List.hd a ^ ")") [ "v", 0 ]
  | "partial-compiled" ->
    call
      "v"
      (fun a -> two "partial_compiled" (List.hd a) (List.nth a 1))
      [ "v", 0; "args", 1 ]
  | _ -> unsupported ("the operation " ^ op)
;;

let test em op sources =
  let src field i =
    match List.nth_opt sources i with
    | Some s -> source em field s
    | None -> unsupported (op ^ " with too few operands")
  in
  match op with
  | "false?" -> Result.map (fun a -> "!truth(" ^ a ^ ")") (src "v" 0)
  | "true?" -> Result.map (fun a -> "truth(" ^ a ^ ")") (src "v" 0)
  | "matched?" -> Result.map (fun a -> a ^ " != &unmatched") (src "env" 0)
  | "primitive-procedure?" -> Result.map (fun a -> "is_primitive(" ^ a ^ ")") (src "v" 0)
  | "compiled-procedure?" -> Result.map (fun a -> "is_compiled(" ^ a ^ ")") (src "v" 0)
  | "no-arguments?" -> Result.map (fun a -> a ^ "->n == 0") (src "args" 0)
  | "primitive-exact?" | "compiled-partial?" ->
    let* p = src "v" 0 in
    let* a = src "args" 1 in
    Ok
      (if op = "primitive-exact?"
       then Printf.sprintf "primitive_exact(%s, %s)" p a
       else Printf.sprintf "%s->n < compiled(%s)->n" a p)
  | _ -> unsupported ("the test " ^ op)
;;

let perform em op sources =
  match op, sources with
  | "fill-first-pending", [ p; v ] ->
    let* p = source em "pending" p in
    let* v = source em "v" v in
    Ok (Printf.sprintf "fill_first_pending(%s, %s);" p v)
  | "signal-match-failure", _ -> Ok "fail(\"no case matched\");"
  | "signal-not-applicable", _ -> Ok "fail(\"not applicable\");"
  | _ -> unsupported ("the action " ^ op)
;;

let statement em = function
  | M.Label l ->
    let n = label em l in
    Ok (Printf.sprintf "case %d: lab_%d:;" n n)
  | M.Assign (t, M.Reg r) -> Ok (Printf.sprintf "%s = %s;" (reg t) (reg r))
  | M.Assign (t, (M.Label_ref _ as s)) ->
    Result.map (fun s -> Printf.sprintf "%s.label = %s;" (reg t) s) (source em "label" s)
  | M.Assign (t, (M.Const (W.Args _) as s)) ->
    Result.map (fun s -> Printf.sprintf "%s.args = %s;" (reg t) s) (source em "args" s)
  | M.Assign (t, s) ->
    Result.map (fun s -> Printf.sprintf "%s.v = %s;" (reg t) s) (source em "v" s)
  | M.Assign_op (t, op, sources) ->
    let* field, e = operation em op sources in
    Ok (Printf.sprintf "%s.%s = %s;" (reg t) field e)
  | M.Test (op, sources) -> Result.map (fun c -> "flag = " ^ c ^ ";") (test em op sources)
  | M.Branch l -> Ok (Printf.sprintf "if (flag) goto lab_%d;" (label em l))
  | M.Goto l -> Ok (Printf.sprintf "goto lab_%d;" (label em l))
  | M.Goto_reg r -> Ok (Printf.sprintf "pc = %s.label; goto dispatch;" (reg r))
  | M.Save r -> Ok (Printf.sprintf "push(%s);" (reg r))
  | M.Restore r -> Ok (Printf.sprintf "%s = pop();" (reg r))
  | M.Perform (op, sources) -> perform em op sources
;;

let emit (code : C.seq) =
  let em =
    { labels = Hashtbl.create 64
    ; globals = Buffer.create 4096
    ; init = Buffer.create 4096
    ; next = 0
    }
  in
  let* lines = all (statement em) code.statements in
  let registers =
    String.concat
      ""
      (List.map
         (fun r -> Printf.sprintf "static Word %s;\n" (reg r))
         C.compiled_registers)
  in
  Ok
    (String.concat
       ""
       [ Buffer.contents em.globals
       ; registers
       ; "\nstatic void init_constants(void)\n{\n"
       ; Buffer.contents em.init
       ; "}\n\n\
          static void run(void)\n\
          {\n\
         \  long pc = 0;\n\
         \  int flag = 0;\n\
         \  R_env.env = initial_env();\n\
          dispatch:\n\
         \  switch (pc) {\n\
         \  case 0:\n"
       ; String.concat "" (List.map (fun l -> "  " ^ l ^ "\n") lines)
       ; "  return;\n\
         \  default:\n\
         \    fail(\"unknown label\");\n\
         \  }\n\
         \  (void)flag;\n\
          }\n\n\
          int main(void)\n\
          {\n\
         \  init_constants();\n\
         \  run();\n\
         \  fflush(stdout);\n\
         \  return 0;\n\
          }\n"
       ])
;;

let compiled_runtime_c =
  {c|typedef struct Pending {
  Env *env;
  int n;
  int position;
  Env **cells;
} Pending;

typedef union {
  Value *v;
  Env *env;
  Args *args;
  long label;
  Pending *pending;
} Word;

static Env unmatched;
static Word *stack;
static long depth, capacity;

static void push(Word w)
{
  if (depth == capacity) {
    capacity = capacity == 0 ? 1024 : capacity * 2;
    stack = realloc(stack, sizeof(Word) * (size_t)capacity);
    if (stack == NULL) fail("out of memory");
  }
  stack[depth++] = w;
}

static Word pop(void)
{
  if (depth == 0) fail("stack underflow");
  return stack[--depth];
}

static Value *make_compiled(long entry, int n, const char **params, Env *env)
{
  Value *v = make(T_COMPILED);
  v->i = entry;
  v->n = n;
  v->params = params;
  v->env = env;
  return v;
}

static Value *compiled(Value *v)
{
  v = resolve(v);
  if (v->tag != T_COMPILED) fail("expected a compiled procedure");
  return v;
}

static Env *compiled_bind(Value *proc, Args *argl)
{
  proc = compiled(proc);
  if (argl->n != proc->n) fail("compiled-procedure-bind: arity mismatch");
  return extend_all(proc->env, proc->n, proc->params, argl->vs);
}

static Args *take_args(Args *a, int k)
{
  Args *b = allocate(sizeof *b);
  b->n = k < a->n ? k : a->n;
  b->vs = a->vs;
  return b;
}

static Value *partial_compiled(Value *proc, Args *argl)
{
  proc = compiled(proc);
  int k = argl->n;
  return make_compiled(proc->i, proc->n - k, proc->params + k,
                       extend_all(proc->env, k, proc->params, argl->vs));
}

static Pending *let_rec_group(int n, const char **names, Env *env)
{
  Pending *p = allocate(sizeof *p);
  p->n = n;
  p->cells = allocate(sizeof(Env *) * (size_t)(n > 0 ? n : 1));
  for (int k = n - 1; k >= 0; k--)
    if (names[k] != NULL) env = extend(env, names[k], NULL);
  Env *walk = env;
  for (int k = 0; k < n; k++)
    if (names[k] != NULL) {
      p->cells[k] = walk;
      walk = walk->next;
    }
  p->env = env;
  return p;
}

static void fill_first_pending(Pending *p, Value *v)
{
  if (p->position >= p->n) fail("fill-first-pending: no binding left");
  if (p->cells[p->position] != NULL) p->cells[p->position]->v = v;
}

static Pending *rest_pending(Pending *p)
{
  Pending *q = allocate(sizeof *q);
  *q = *p;
  q->position++;
  return q;
}

static Env *try_pattern(Pat *p, Value *v, Env *env)
{
  Env *bound = env;
  return bind(p, v, &bound) ? bound : &unmatched;
}

static Env *let_environment(int n, const char **names, Args *argl, Env *env)
{
  for (int k = n - 1; k >= 0; k--)
    if (names[k] != NULL) env = extend(env, names[k], argl->vs[k]);
  return env;
}

static Value *last_argument(Args *argl)
{
  return argl->n == 0 ? make_unit() : argl->vs[argl->n - 1];
}

static int is_primitive(Value *v) { return resolve(v)->tag == T_PRIM; }
static int is_compiled(Value *v) { return resolve(v)->tag == T_COMPILED; }

static int primitive_exact(Value *proc, Args *argl)
{
  proc = resolve(proc);
  return proc->tag == T_PRIM && proc->n + argl->n == prim_arity(proc);
}

static Args *all_primitive_arguments(Value *proc, Args *argl)
{
  Args *all = empty_args();
  for (int k = 0; k < proc->n; k++) all = adjoin(all, proc->items[k]);
  for (int k = 0; k < argl->n; k++) all = adjoin(all, argl->vs[k]);
  return all;
}

static Value *apply_primitive(Value *proc, Args *argl)
{
  proc = resolve(proc);
  if (proc->tag != T_PRIM) fail("expected a primitive");
  Args *all = all_primitive_arguments(proc, argl);
  if (all->n < prim_arity(proc)) return make_prim(prim_id(proc), prim_arity(proc), all->n, all->vs);
  return prim_apply(prim_id(proc), all->vs);
}

static Args *primitive_excess(Value *proc, Args *argl)
{
  proc = resolve(proc);
  return drop_args(argl, prim_arity(proc) - proc->n);
}

static long compiled_entry(Value *proc) { return compiled(proc)->i; }
|c}
;;

let c_program code =
  let* generated = emit code in
  Ok (Sec_5_51.runtime_c ^ compiled_runtime_c ^ generated)
;;

let compile_to_c program =
  c_program (C.compile_program (C.new_state ()) (Check.items program))
;;

let run_c program =
  let* c_source = compile_to_c program in
  let* out, _ = Sec_5_51.build_and_run ~c_source ~inputs:[] in
  Ok out
;;

let ex_5_52 () =
  let* metacircular =
    Sec_5_33.program
      ~filename:"ex_5_52.ml"
      (Sicp_ch5.Metacircular.with_guest Sec_5_50.guest_factorial)
  in
  let* counter =
    Sec_5_33.program
      ~filename:"ex_5_52.ml"
      (Sicp_ch5.Metacircular.with_guest Sec_5_50.guest_counter)
  in
  let* c_source = compile_to_c metacircular in
  let* c_output, _ = Sec_5_51.build_and_run ~c_source ~inputs:[] in
  let* counter_output = run_c counter in
  let machine = Buffer.create 16 in
  let* _ = C.run ~emit:(Buffer.add_string machine) metacircular in
  let machine_output = Buffer.contents machine in
  let* native_output =
    Sec_5_51.ocaml_native_run (Sicp_ch5.Metacircular.with_guest Sec_5_50.guest_factorial)
  in
  let* () =
    if c_output = machine_output && machine_output = native_output
    then Ok ()
    else
      Error
        (Sicp_common.Eval_error.User_error
           (Printf.sprintf
              "engines disagree: C %S, machine %S, native %S"
              c_output
              machine_output
              native_output))
  in
  Ok
    [ Printf.sprintf
        "compiled metacircular in C: %S (machine %S); counter %S"
        c_output
        machine_output
        counter_output
    ; Printf.sprintf "native oracle: %S" native_output
    ; Printf.sprintf
        "generated C: %d lines"
        (List.length (String.split_on_char '\n' c_source))
    ]
;;
