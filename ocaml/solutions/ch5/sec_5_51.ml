(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error

let ( let* ) = Result.bind

let unsupported what =
  Error (Eval_error.Invalid_form ("the C evaluator does not support " ^ what))
;;

let text s = Printf.sprintf "%d:%s" (String.length s) s

let arith_code = function
  | Ast.Add -> Ok 0
  | Ast.Sub -> Ok 1
  | Ast.Mul -> Ok 2
  | Ast.Div -> Ok 3
  | Ast.Rem -> Ok 4
  | Ast.Addf | Ast.Subf | Ast.Mulf | Ast.Divf -> unsupported "float arithmetic"
;;

let comparison_code = function
  | Ast.Eq -> 5
  | Ast.Ne -> 6
  | Ast.Lt -> 7
  | Ast.Le -> 8
  | Ast.Gt -> 9
  | Ast.Ge -> 10
;;

let scalar = function
  | Ast.Int n -> Ok (Printf.sprintf "i %d" n)
  | Ast.Bool b -> Ok (Printf.sprintf "b %d" (Bool.to_int b))
  | Ast.Unit -> Ok "u"
  | Ast.String s -> Ok ("s " ^ text s)
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

let counted tag f xs =
  let* parts = all f xs in
  Ok (String.concat " " ((tag ^ " " ^ string_of_int (List.length xs)) :: parts))
;;

let rec pattern p =
  match Ast.view_pattern p with
  | Ast.PWildcard -> Ok "_"
  | Ast.PVar x -> Ok ("v " ^ text x)
  | Ast.PScalar s -> scalar s
  | Ast.PTuple ps -> counted "t" pattern ps
  | Ast.PConstruct (c, ps) -> counted ("c " ^ text c) pattern ps
  | Ast.PNil -> Ok "n"
  | Ast.PCons (h, t) ->
    let* h = pattern h in
    let* t = pattern t in
    Ok (": " ^ h ^ " " ^ t)
;;

let binding_name (b : Ast.binding) = Option.value b.name ~default:"_"

let rec expr e =
  let two tag a b =
    let* a = expr a in
    let* b = expr b in
    Ok (String.concat " " [ tag; a; b ])
  in
  let one tag a =
    let* a = expr a in
    Ok (tag ^ " " ^ a)
  in
  match Ast.view e with
  | Ast.Scalar s -> scalar s
  | Ast.Nil -> Ok "n"
  | Ast.Var x -> Ok ("v " ^ text x)
  | Ast.Fun (ps, body) ->
    let* body = expr body in
    Ok
      (String.concat
         " "
         ((("f " ^ string_of_int (List.length ps)) :: List.map text ps) @ [ body ]))
  | Ast.Apply (f, args) ->
    let* f = expr f in
    let* args = counted "" expr args in
    Ok ("a " ^ f ^ args)
  | Ast.If (c, t, f) ->
    let* c = expr c in
    two ("? " ^ c) t f
  | Ast.Sequence (a, b) -> two ";" a b
  | Ast.And (a, b) -> two "&" a b
  | Ast.Or (a, b) -> two "|" a b
  | Ast.Arith (op, a, b) ->
    let* code = arith_code op in
    two ("o " ^ string_of_int code) a b
  | Ast.Compare (op, a, b) -> two ("o " ^ string_of_int (comparison_code op)) a b
  | Ast.Concat (a, b) -> two "o 11" a b
  | Ast.Cons (a, b) -> two "o 12" a b
  | Ast.Assign (a, b) -> two "o 13" a b
  | Ast.Not a -> one "p 0" a
  | Ast.Neg a -> one "p 1" a
  | Ast.Deref a -> one "p 2" a
  | Ast.Make_ref a -> one "p 3" a
  | Ast.Tuple es -> counted "t" expr es
  | Ast.Construct (c, es) -> counted ("c " ^ text c) expr es
  | Ast.Let (recursive, bindings, body) ->
    let* bindings = bindings_text bindings in
    let* body = expr body in
    Ok (String.concat " " [ (if recursive then "r" else "l"); bindings; body ])
  | Ast.Match (s, cases) ->
    let* s = expr s in
    let* cases =
      counted
        ""
        (fun (p, body) ->
           let* p = pattern p in
           let* body = expr body in
           Ok (p ^ " " ^ body))
        cases
    in
    Ok ("m " ^ s ^ cases)
  | Ast.Record _ | Ast.Field _ -> unsupported "records"

and bindings_text bindings =
  let* parts =
    all
      (fun (b : Ast.binding) ->
         let* rhs = expr b.rhs in
         Ok (text (binding_name b) ^ " " ^ rhs))
      bindings
  in
  Ok (String.concat " " (string_of_int (List.length bindings) :: parts))
;;

let serialize items =
  let* lines =
    all
      (function
        | Ast.Type_item _ -> Ok ""
        | Ast.Value_item (recursive, bindings) ->
          let* bindings = bindings_text bindings in
          Ok (Printf.sprintf "D %d %s\n" (Bool.to_int recursive) bindings))
      items
  in
  Ok (String.concat "" lines ^ "E\n")
;;

let read_file path =
  let ic = open_in_bin path in
  Fun.protect
    ~finally:(fun () -> close_in ic)
    (fun () -> really_input_string ic (in_channel_length ic))
;;

let write_file path contents =
  let oc = open_out_bin path in
  Fun.protect ~finally:(fun () -> close_out oc) (fun () -> output_string oc contents)
;;

let run_process program arguments ~stdout_path ~stderr_path =
  let out =
    Unix.openfile stdout_path [ Unix.O_WRONLY; Unix.O_CREAT; Unix.O_TRUNC ] 0o600
  in
  let err =
    Unix.openfile stderr_path [ Unix.O_WRONLY; Unix.O_CREAT; Unix.O_TRUNC ] 0o600
  in
  let pid =
    Fun.protect
      ~finally:(fun () ->
        Unix.close out;
        Unix.close err)
      (fun () ->
         Unix.create_process
           program
           (Array.of_list (program :: arguments))
           Unix.stdin
           out
           err)
  in
  match snd (Unix.waitpid [] pid) with
  | Unix.WEXITED code -> code
  | Unix.WSIGNALED n | Unix.WSTOPPED n -> 128 + n
;;

let c_compiler = "cc"

let build_and_run ~c_source ~inputs =
  let dir = Filename.temp_dir "sicp_ex_5_5" "" in
  let path name = Filename.concat dir name in
  let files = ("program.c" :: List.map fst inputs) @ [ "program"; "out"; "err" ] in
  Fun.protect
    ~finally:(fun () ->
      List.iter (fun f -> if Sys.file_exists (path f) then Sys.remove (path f)) files;
      Sys.rmdir dir)
    (fun () ->
       write_file (path "program.c") c_source;
       List.iter (fun (name, contents) -> write_file (path name) contents) inputs;
       let built =
         run_process
           c_compiler
           [ "-std=c11"; "-O1"; "-o"; path "program"; path "program.c" ]
           ~stdout_path:(path "out")
           ~stderr_path:(path "err")
       in
       if built <> 0
       then
         Error
           (Eval_error.Invalid_form ("the C compiler failed: " ^ read_file (path "err")))
       else (
         let status =
           run_process
             (path "program")
             (List.map (fun (name, _) -> path name) inputs)
             ~stdout_path:(path "out")
             ~stderr_path:(path "err")
         in
         let out = read_file (path "out") in
         let err = read_file (path "err") in
         if status = 0 then Ok (out, err) else Error (Eval_error.User_error (out ^ err))))
;;

(* The native oracle of grammar section 12: the same guest source
   the teaching engines run, executed by the host toolchain. *)
let ocaml_native_run source =
  let dir = Filename.temp_dir "sicp_ex_5_5_native" "" in
  let path name = Filename.concat dir name in
  Fun.protect
    ~finally:(fun () ->
      Array.iter
        (fun f -> if f <> "." && f <> ".." then Sys.remove (Filename.concat dir f))
        (Sys.readdir dir);
      Sys.rmdir dir)
    (fun () ->
       write_file (path "program.ml") source;
       let ocamlc =
         [ "exec"
         ; "--"
         ; "ocamlc"
         ; "-warn-error"
         ; "+8"
         ; "-o"
         ; path "program"
         ; path "program.ml"
         ]
       in
       let run switch =
         run_process
           "opam"
           (switch @ ocamlc)
           ~stdout_path:(path "out")
           ~stderr_path:(path "err")
       in
       (* The pinned switch keeps the oracle on the contract's compiler
          when the ambient switch is older; the gates and CI already run
          under the pinned switch, where no switch is named 5.5.1, so the
          ambient exec is the fallback. *)
       let built =
         match run [ "exec"; "--switch=5.5.1" ] with
         | 0 -> 0
         | _ -> run []
       in
       if built <> 0
       then
         Error
           (Eval_error.Invalid_form
              ("the native compiler rejected the guest source: " ^ read_file (path "err")))
       else (
         let status =
           run_process
             (path "program")
             []
             ~stdout_path:(path "out")
             ~stderr_path:(path "err")
         in
         let out = read_file (path "out") in
         if status = 0
         then Ok out
         else
           Error
             (Eval_error.User_error
                ("the native program failed: " ^ out ^ read_file (path "err")))))
;;

let runtime_c =
  {c|#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Value Value;
typedef struct Env Env;
typedef struct Node Node;
typedef struct Pat Pat;
typedef struct Args Args;

enum {
  T_INT, T_BOOL, T_UNIT, T_STRING, T_TUPLE, T_CTOR, T_NIL, T_CONS, T_REF,
  T_ARRAY, T_CLOSURE, T_PRIM, T_COMPILED, T_INDIRECT
};

struct Env {
  const char *name;
  Value *v;
  Env *next;
};

struct Value {
  int tag;
  long i;
  const char *s;
  int n;
  Value **items;
  const char **params;
  Node *body;
  Env *env;
  Env *cell;
};

struct Args {
  int n;
  Value **vs;
};

static void fail(const char *what)
{
  fflush(stdout);
  fprintf(stderr, "error: %s\n", what);
  exit(2);
}

static void *allocate(size_t size)
{
  void *p = calloc(1, size);
  if (p == NULL) fail("out of memory");
  return p;
}

static Value *make(int tag)
{
  Value *v = allocate(sizeof *v);
  v->tag = tag;
  return v;
}

static Value *make_int(long i)
{
  Value *v = make(T_INT);
  v->i = i;
  return v;
}

static Value *make_bool(int b)
{
  Value *v = make(T_BOOL);
  v->i = b != 0;
  return v;
}

static Value *make_unit(void) { return make(T_UNIT); }
static Value *make_nil(void) { return make(T_NIL); }

static Value *make_string(const char *s)
{
  Value *v = make(T_STRING);
  v->s = s;
  return v;
}

static Value *make_items(int tag, const char *name, int n, Value **items)
{
  Value *v = make(tag);
  v->s = name;
  v->n = n;
  v->items = allocate(sizeof(Value *) * (size_t)(n > 0 ? n : 1));
  if (n > 0) memcpy(v->items, items, sizeof(Value *) * (size_t)n);
  return v;
}

static Value *make_cons(Value *head, Value *tail)
{
  Value *parts[2];
  parts[0] = head;
  parts[1] = tail;
  return make_items(T_CONS, NULL, 2, parts);
}

static Value *resolve(Value *v)
{
  while (v != NULL && v->tag == T_INDIRECT) {
    if (v->cell->v == NULL) fail("recursive binding used before it is defined");
    v = v->cell->v;
  }
  if (v == NULL) fail("missing value");
  return v;
}

static Env *extend(Env *env, const char *name, Value *v)
{
  Env *e = allocate(sizeof *e);
  e->name = name;
  e->v = v;
  e->next = env;
  return e;
}

static Env *extend_all(Env *env, int n, const char **names, Value **vs)
{
  for (int k = n - 1; k >= 0; k--) env = extend(env, names[k], vs[k]);
  return env;
}

static Value *lookup(Env *env, const char *name)
{
  for (; env != NULL; env = env->next) {
    if (strcmp(env->name, name) == 0) {
      if (env->v != NULL) return env->v;
      Value *v = make(T_INDIRECT);
      v->cell = env;
      return v;
    }
  }
  fail("unbound variable");
  return NULL;
}

static Args *empty_args(void) { return allocate(sizeof(Args)); }

static Args *adjoin(Args *a, Value *v)
{
  Args *b = allocate(sizeof *b);
  b->n = a->n + 1;
  b->vs = allocate(sizeof(Value *) * (size_t)b->n);
  if (a->n > 0) memcpy(b->vs, a->vs, sizeof(Value *) * (size_t)a->n);
  b->vs[a->n] = v;
  return b;
}

static Args *drop_args(Args *a, int k)
{
  Args *b = allocate(sizeof *b);
  if (k > a->n) k = a->n;
  b->n = a->n - k;
  b->vs = a->vs + k;
  return b;
}

static int truth(Value *v)
{
  v = resolve(v);
  if (v->tag != T_BOOL) fail("condition is not a bool");
  return (int)v->i;
}

static long as_int(Value *v)
{
  v = resolve(v);
  if (v->tag != T_INT) fail("expected an int");
  return v->i;
}

static const char *as_string(Value *v)
{
  v = resolve(v);
  if (v->tag != T_STRING) fail("expected a string");
  return v->s;
}

enum {
  P_PRINT_STRING, P_PRINT_ENDLINE, P_PRINT_INT, P_PRINT_NEWLINE, P_STRING_OF_INT,
  P_ARRAY_MAKE, P_ARRAY_GET, P_ARRAY_SET, P_ARRAY_LENGTH
};

static Value *make_prim(long id, int arity, int n, Value **gathered)
{
  Value *v = make_items(T_PRIM, NULL, n, gathered);
  v->i = id | ((long)arity << 16);
  return v;
}

static int prim_id(Value *p) { return (int)(p->i & 0xffff); }
static int prim_arity(Value *p) { return (int)(p->i >> 16); }

static Value *prim_apply(int id, Value **a)
{
  char buffer[32];
  switch (id) {
  case P_PRINT_STRING: fputs(as_string(a[0]), stdout); return make_unit();
  case P_PRINT_ENDLINE: fputs(as_string(a[0]), stdout); fputc('\n', stdout); return make_unit();
  case P_PRINT_INT: printf("%ld", as_int(a[0])); return make_unit();
  case P_PRINT_NEWLINE: fputc('\n', stdout); return make_unit();
  case P_STRING_OF_INT: {
    int length = snprintf(buffer, sizeof buffer, "%ld", as_int(a[0]));
    if (length < 0 || (size_t)length >= sizeof buffer) fail("string_of_int overflow");
    char *s = allocate((size_t)length + 1);
    memcpy(s, buffer, (size_t)length + 1);
    return make_string(s);
  }
  case P_ARRAY_MAKE: {
    long n = as_int(a[0]);
    if (n < 0) fail("Array.make: negative length");
    Value *v = make_items(T_ARRAY, NULL, 0, NULL);
    v->items = allocate(sizeof(Value *) * (size_t)(n > 0 ? n : 1));
    v->n = (int)n;
    for (long k = 0; k < n; k++) v->items[k] = a[1];
    return v;
  }
  case P_ARRAY_GET: {
    Value *arr = resolve(a[0]);
    long k = as_int(a[1]);
    if (arr->tag != T_ARRAY) fail("Array.get: not an array");
    if (k < 0 || k >= arr->n) fail("Array.get: index out of bounds");
    return arr->items[k];
  }
  case P_ARRAY_SET: {
    Value *arr = resolve(a[0]);
    long k = as_int(a[1]);
    if (arr->tag != T_ARRAY) fail("Array.set: not an array");
    if (k < 0 || k >= arr->n) fail("Array.set: index out of bounds");
    arr->items[k] = a[2];
    return make_unit();
  }
  case P_ARRAY_LENGTH: {
    Value *arr = resolve(a[0]);
    if (arr->tag != T_ARRAY) fail("Array.length: not an array");
    return make_int(arr->n);
  }
  }
  fail("unknown primitive");
  return NULL;
}

static Env *initial_env(void)
{
  static const struct { const char *name; int id; int arity; } prims[] = {
    { "print_string", P_PRINT_STRING, 1 }, { "print_endline", P_PRINT_ENDLINE, 1 },
    { "print_int", P_PRINT_INT, 1 }, { "print_newline", P_PRINT_NEWLINE, 1 },
    { "string_of_int", P_STRING_OF_INT, 1 }, { "Array.make", P_ARRAY_MAKE, 2 },
    { "Array.get", P_ARRAY_GET, 2 }, { "Array.set", P_ARRAY_SET, 3 },
    { "Array.length", P_ARRAY_LENGTH, 1 }
  };
  Env *env = NULL;
  for (size_t k = 0; k < sizeof prims / sizeof prims[0]; k++)
    env = extend(env, prims[k].name, make_prim(prims[k].id, prims[k].arity, 0, NULL));
  return env;
}

enum {
  O_ADD, O_SUB, O_MUL, O_DIV, O_MOD, O_EQ, O_NE, O_LT, O_LE, O_GT, O_GE,
  O_CONCAT, O_CONS, O_ASSIGN
};

static int scalar_equal(Value *a, Value *b)
{
  a = resolve(a);
  b = resolve(b);
  if (a->tag != b->tag) fail("equality on different kinds");
  switch (a->tag) {
  case T_UNIT: return 1;
  case T_INT: case T_BOOL: return a->i == b->i;
  case T_STRING: return strcmp(a->s, b->s) == 0;
  }
  fail("equality on a non-scalar");
  return 0;
}

static int scalar_compare(Value *a, Value *b)
{
  a = resolve(a);
  b = resolve(b);
  if (a->tag == T_INT && b->tag == T_INT) return (a->i > b->i) - (a->i < b->i);
  if (a->tag == T_STRING && b->tag == T_STRING) return strcmp(a->s, b->s);
  fail("ordered comparison on a non-scalar");
  return 0;
}

static Value *binary(int op, Value *a, Value *b)
{
  switch (op) {
  case O_ADD: return make_int(as_int(a) + as_int(b));
  case O_SUB: return make_int(as_int(a) - as_int(b));
  case O_MUL: return make_int(as_int(a) * as_int(b));
  case O_DIV: if (as_int(b) == 0) fail("division by zero"); return make_int(as_int(a) / as_int(b));
  case O_MOD: if (as_int(b) == 0) fail("division by zero"); return make_int(as_int(a) % as_int(b));
  case O_EQ: return make_bool(scalar_equal(a, b));
  case O_NE: return make_bool(!scalar_equal(a, b));
  case O_LT: return make_bool(scalar_compare(a, b) < 0);
  case O_LE: return make_bool(scalar_compare(a, b) <= 0);
  case O_GT: return make_bool(scalar_compare(a, b) > 0);
  case O_GE: return make_bool(scalar_compare(a, b) >= 0);
  case O_CONCAT: {
    const char *x = as_string(a), *y = as_string(b);
    size_t nx = strlen(x), ny = strlen(y);
    char *s = allocate(nx + ny + 1);
    memcpy(s, x, nx);
    memcpy(s + nx, y, ny + 1);
    return make_string(s);
  }
  case O_CONS: return make_cons(a, b);
  case O_ASSIGN: {
    Value *r = resolve(a);
    if (r->tag != T_REF) fail(":= target is not a reference");
    r->items[0] = b;
    return make_unit();
  }
  }
  fail("unknown binary operator");
  return NULL;
}

enum { U_NOT, U_NEG, U_DEREF, U_REF };

static Value *unary(int op, Value *a)
{
  switch (op) {
  case U_NOT: return make_bool(!truth(a));
  case U_NEG: return make_int(-as_int(a));
  case U_DEREF: {
    Value *r = resolve(a);
    if (r->tag != T_REF) fail("! target is not a reference");
    return r->items[0];
  }
  case U_REF: return make_items(T_REF, NULL, 1, &a);
  }
  fail("unknown unary operator");
  return NULL;
}

enum { PK_WILD, PK_VAR, PK_INT, PK_BOOL, PK_UNIT, PK_STRING, PK_TUPLE, PK_CTOR, PK_NIL, PK_CONS };

struct Pat {
  int kind;
  long i;
  const char *s;
  int n;
  Pat **subs;
};

static Pat *make_pat(int kind, long i, const char *s, int n, Pat **subs)
{
  Pat *p = allocate(sizeof *p);
  p->kind = kind;
  p->i = i;
  p->s = s;
  p->n = n;
  p->subs = allocate(sizeof(Pat *) * (size_t)(n > 0 ? n : 1));
  if (n > 0) memcpy(p->subs, subs, sizeof(Pat *) * (size_t)n);
  return p;
}

static int bind(Pat *p, Value *v, Env **env);

static int match_items(Pat **ps, Value **vs, int n, Env **env)
{
  for (int k = 0; k < n; k++)
    if (!bind(ps[k], vs[k], env)) return 0;
  return 1;
}

static int bind(Pat *p, Value *v, Env **env)
{
  if (p->kind == PK_WILD) return 1;
  if (p->kind == PK_VAR) {
    *env = extend(*env, p->s, v);
    return 1;
  }
  v = resolve(v);
  switch (p->kind) {
  case PK_INT: return v->tag == T_INT && v->i == p->i;
  case PK_BOOL: return v->tag == T_BOOL && v->i == p->i;
  case PK_UNIT: return v->tag == T_UNIT;
  case PK_STRING: return v->tag == T_STRING && strcmp(v->s, p->s) == 0;
  case PK_TUPLE: return v->tag == T_TUPLE && v->n == p->n && match_items(p->subs, v->items, p->n, env);
  case PK_CTOR:
    return v->tag == T_CTOR && strcmp(v->s, p->s) == 0 && v->n == p->n
           && match_items(p->subs, v->items, p->n, env);
  case PK_NIL: return v->tag == T_NIL;
  case PK_CONS: return v->tag == T_CONS && match_items(p->subs, v->items, 2, env);
  }
  fail("unknown pattern");
  return 0;
}
|c}
;;

let eceval_c =
  {c|enum {
  K_SCALAR, K_NIL, K_VAR, K_FUN, K_APPLY, K_IF, K_SEQ, K_AND, K_OR, K_BIN, K_UN,
  K_TUPLE, K_CTOR, K_LET, K_LETREC, K_MATCH
};

struct Node {
  int kind;
  int op;
  const char *s;
  int n;
  Value *value;
  const char **names;
  Node **kids;
  Pat **pats;
};

static const char *input;

static void skip(void)
{
  while (*input == ' ' || *input == '\n') input++;
}

static char read_tag(void)
{
  skip();
  if (*input == '\0') fail("unexpected end of input");
  return *input++;
}

static long read_long(void)
{
  skip();
  char *end;
  long n = strtol(input, &end, 10);
  if (end == input) fail("expected a number");
  input = end;
  return n;
}

static const char *read_text(void)
{
  long n = read_long();
  if (*input != ':' || n < 0) fail("expected a text");
  input++;
  if (memchr(input, 0, (size_t)n) != NULL) fail("text runs past the input");
  char *s = allocate((size_t)n + 1);
  memcpy(s, input, (size_t)n);
  input += n;
  return s;
}

static Pat *read_pattern(void)
{
  char tag = read_tag();
  switch (tag) {
  case '_': return make_pat(PK_WILD, 0, NULL, 0, NULL);
  case 'v': return make_pat(PK_VAR, 0, read_text(), 0, NULL);
  case 'i': return make_pat(PK_INT, read_long(), NULL, 0, NULL);
  case 'b': return make_pat(PK_BOOL, read_long(), NULL, 0, NULL);
  case 'u': return make_pat(PK_UNIT, 0, NULL, 0, NULL);
  case 's': return make_pat(PK_STRING, 0, read_text(), 0, NULL);
  case 'n': return make_pat(PK_NIL, 0, NULL, 0, NULL);
  case ':': {
    Pat *parts[2];
    parts[0] = read_pattern();
    parts[1] = read_pattern();
    return make_pat(PK_CONS, 0, NULL, 2, parts);
  }
  case 't': case 'c': {
    const char *name = tag == 'c' ? read_text() : NULL;
    int n = (int)read_long();
    Pat **subs = allocate(sizeof(Pat *) * (size_t)(n > 0 ? n : 1));
    for (int k = 0; k < n; k++) subs[k] = read_pattern();
    return make_pat(tag == 'c' ? PK_CTOR : PK_TUPLE, 0, name, n, subs);
  }
  }
  fail("unknown pattern tag");
  return NULL;
}

static Node *node(int kind, int n)
{
  Node *e = allocate(sizeof *e);
  e->kind = kind;
  e->n = n;
  e->kids = allocate(sizeof(Node *) * (size_t)(n + 2));
  e->names = allocate(sizeof(char *) * (size_t)(n + 1));
  e->pats = allocate(sizeof(Pat *) * (size_t)(n + 1));
  return e;
}

static Node *read_expr(void)
{
  char tag = read_tag();
  Node *e;
  switch (tag) {
  case 'i': e = node(K_SCALAR, 0); e->value = make_int(read_long()); return e;
  case 'b': e = node(K_SCALAR, 0); e->value = make_bool((int)read_long()); return e;
  case 'u': e = node(K_SCALAR, 0); e->value = make_unit(); return e;
  case 's': e = node(K_SCALAR, 0); e->value = make_string(read_text()); return e;
  case 'n': return node(K_NIL, 0);
  case 'v': e = node(K_VAR, 0); e->s = read_text(); return e;
  case 'f': {
    int n = (int)read_long();
    e = node(K_FUN, n);
    for (int k = 0; k < n; k++) e->names[k] = read_text();
    e->kids[0] = read_expr();
    return e;
  }
  case 'a': {
    Node *f = read_expr();
    int n = (int)read_long();
    e = node(K_APPLY, n);
    e->kids[0] = f;
    for (int k = 1; k <= n; k++) e->kids[k] = read_expr();
    return e;
  }
  case '?': e = node(K_IF, 3); for (int k = 0; k < 3; k++) e->kids[k] = read_expr(); return e;
  case ';': case '&': case '|':
    e = node(tag == ';' ? K_SEQ : tag == '&' ? K_AND : K_OR, 2);
    e->kids[0] = read_expr();
    e->kids[1] = read_expr();
    return e;
  case 'o':
    e = node(K_BIN, 2);
    e->op = (int)read_long();
    e->kids[0] = read_expr();
    e->kids[1] = read_expr();
    return e;
  case 'p':
    e = node(K_UN, 1);
    e->op = (int)read_long();
    e->kids[0] = read_expr();
    return e;
  case 't': case 'c': {
    const char *name = tag == 'c' ? read_text() : NULL;
    int n = (int)read_long();
    e = node(tag == 'c' ? K_CTOR : K_TUPLE, n);
    e->s = name;
    for (int k = 0; k < n; k++) e->kids[k] = read_expr();
    return e;
  }
  case 'l': case 'r': {
    int n = (int)read_long();
    e = node(tag == 'l' ? K_LET : K_LETREC, n);
    for (int k = 0; k < n; k++) {
      e->names[k] = read_text();
      e->kids[k] = read_expr();
    }
    e->kids[n] = read_expr();
    return e;
  }
  case 'm': {
    Node *scrutinee = read_expr();
    int n = (int)read_long();
    e = node(K_MATCH, n);
    e->kids[0] = scrutinee;
    for (int k = 0; k < n; k++) {
      e->pats[k] = read_pattern();
      e->kids[k + 1] = read_expr();
    }
    return e;
  }
  }
  fail("unknown expression tag");
  return NULL;
}

typedef union {
  Value *v;
  Node *e;
  Env *env;
  Args *args;
  long k;
} Word;

static Word *stack;
static long depth, capacity, pushes, max_depth;

static void push(Word w)
{
  if (depth == capacity) {
    capacity = capacity == 0 ? 1024 : capacity * 2;
    stack = realloc(stack, sizeof(Word) * (size_t)capacity);
    if (stack == NULL) fail("out of memory");
  }
  stack[depth++] = w;
  pushes++;
  if (depth > max_depth) max_depth = depth;
}

static Word pop(void)
{
  if (depth == 0) fail("stack underflow");
  return stack[--depth];
}

#define SAVE_V(x) push((Word){ .v = (x) })
#define SAVE_E(x) push((Word){ .e = (x) })
#define SAVE_ENV(x) push((Word){ .env = (x) })
#define SAVE_A(x) push((Word){ .args = (x) })
#define SAVE_K(x) push((Word){ .k = (x) })

enum {
  EVAL_DISPATCH, IF_DECIDE, SEQUENCE_REST, LOGIC_DECIDE, UNARY_APPLY, BINARY_RIGHT,
  BINARY_APPLY, COLLECT_LOOP, COLLECT_ACCUMULATE, LET_REC_LOOP, LET_REC_FILL,
  MATCH_CASES, APPL_DID_OPERATOR, APPL_OPERAND_LOOP, APPL_ACCUMULATE_ARG,
  APPL_ACCUM_LAST_ARG, APPLY_DISPATCH, COMPOUND_EXCESS, APPLY_EXCESS, DONE
};

static Value *build(Node *e, Args *argl)
{
  if (e->kind == K_TUPLE) return make_items(T_TUPLE, NULL, argl->n, argl->vs);
  return make_items(T_CTOR, e->s, argl->n, argl->vs);
}

static Env *cell_at(Env *env, long k)
{
  while (k-- > 0) env = env->next;
  return env;
}

static Value *eceval(Node *start, Env *start_env)
{
  Node *exp = start;
  Env *env = start_env;
  Value *val = NULL, *proc = NULL;
  Args *argl = empty_args();
  long unev = 0, cont = DONE, pc = EVAL_DISPATCH;
  for (;;) {
    switch (pc) {
    case EVAL_DISPATCH:
      switch (exp->kind) {
      case K_SCALAR: val = exp->value; pc = cont; break;
      case K_NIL: val = make_nil(); pc = cont; break;
      case K_VAR: val = lookup(env, exp->s); pc = cont; break;
      case K_FUN:
        val = make(T_CLOSURE);
        val->n = exp->n;
        val->params = exp->names;
        val->body = exp->kids[0];
        val->env = env;
        pc = cont;
        break;
      case K_IF: case K_SEQ: case K_AND: case K_OR: case K_MATCH:
        SAVE_E(exp); SAVE_ENV(env); SAVE_K(cont);
        cont = exp->kind == K_IF ? IF_DECIDE
               : exp->kind == K_SEQ ? SEQUENCE_REST
               : exp->kind == K_MATCH ? MATCH_CASES : LOGIC_DECIDE;
        exp = exp->kids[0];
        break;
      case K_UN:
        SAVE_E(exp); SAVE_K(cont);
        cont = UNARY_APPLY;
        exp = exp->kids[0];
        break;
      case K_BIN:
        SAVE_K(cont); SAVE_E(exp); SAVE_ENV(env);
        cont = BINARY_RIGHT;
        exp = exp->kids[0];
        break;
      case K_TUPLE: case K_CTOR: case K_LET:
        SAVE_K(cont); SAVE_E(exp);
        argl = empty_args();
        unev = 0;
        pc = COLLECT_LOOP;
        break;
      case K_LETREC:
        for (int k = exp->n - 1; k >= 0; k--) env = extend(env, exp->names[k], NULL);
        unev = 0;
        pc = LET_REC_LOOP;
        break;
      case K_APPLY:
        SAVE_K(cont); SAVE_ENV(env); SAVE_E(exp);
        cont = APPL_DID_OPERATOR;
        exp = exp->kids[0];
        break;
      default: fail("unknown expression");
      }
      break;
    case IF_DECIDE:
      cont = pop().k; env = pop().env; exp = pop().e;
      exp = truth(val) ? exp->kids[1] : exp->kids[2];
      pc = EVAL_DISPATCH;
      break;
    case SEQUENCE_REST:
      cont = pop().k; env = pop().env; exp = pop().e;
      exp = exp->kids[1];
      pc = EVAL_DISPATCH;
      break;
    case LOGIC_DECIDE:
      cont = pop().k; env = pop().env; exp = pop().e;
      if (truth(val) == (exp->kind == K_OR)) { pc = cont; break; }
      exp = exp->kids[1];
      pc = EVAL_DISPATCH;
      break;
    case UNARY_APPLY:
      cont = pop().k; exp = pop().e;
      val = unary(exp->op, val);
      pc = cont;
      break;
    case BINARY_RIGHT:
      env = pop().env; exp = pop().e;
      SAVE_E(exp); SAVE_V(val);
      cont = BINARY_APPLY;
      exp = exp->kids[1];
      pc = EVAL_DISPATCH;
      break;
    case BINARY_APPLY: {
      Value *left = pop().v;
      exp = pop().e; cont = pop().k;
      val = binary(exp->op, left, val);
      pc = cont;
      break;
    }
    case COLLECT_LOOP:
      if (unev == exp->n) {
        exp = pop().e; cont = pop().k;
        if (exp->kind == K_LET) {
          env = extend_all(env, exp->n, exp->names, argl->vs);
          exp = exp->kids[exp->n];
          pc = EVAL_DISPATCH;
        } else {
          val = build(exp, argl);
          pc = cont;
        }
        break;
      }
      SAVE_A(argl); SAVE_ENV(env); SAVE_K(unev); SAVE_E(exp);
      cont = COLLECT_ACCUMULATE;
      exp = exp->kids[unev];
      pc = EVAL_DISPATCH;
      break;
    case COLLECT_ACCUMULATE:
      exp = pop().e; unev = pop().k; env = pop().env; argl = pop().args;
      argl = adjoin(argl, val);
      unev++;
      pc = COLLECT_LOOP;
      break;
    case LET_REC_LOOP:
      if (unev == exp->n) {
        exp = exp->kids[exp->n];
        pc = EVAL_DISPATCH;
        break;
      }
      SAVE_K(cont); SAVE_E(exp); SAVE_ENV(env); SAVE_K(unev);
      cont = LET_REC_FILL;
      exp = exp->kids[unev];
      pc = EVAL_DISPATCH;
      break;
    case LET_REC_FILL:
      unev = pop().k; env = pop().env; exp = pop().e; cont = pop().k;
      cell_at(env, unev)->v = val;
      unev++;
      pc = LET_REC_LOOP;
      break;
    case MATCH_CASES: {
      cont = pop().k; env = pop().env; exp = pop().e;
      Node *chosen = NULL;
      for (int k = 0; k < exp->n && chosen == NULL; k++) {
        Env *bound = env;
        if (bind(exp->pats[k], val, &bound)) {
          env = bound;
          chosen = exp->kids[k + 1];
        }
      }
      if (chosen == NULL) fail("no case matched");
      exp = chosen;
      pc = EVAL_DISPATCH;
      break;
    }
    case APPL_DID_OPERATOR:
      exp = pop().e; env = pop().env;
      argl = empty_args();
      proc = val;
      unev = 0;
      SAVE_V(proc);
      pc = APPL_OPERAND_LOOP;
      break;
    case APPL_OPERAND_LOOP:
      SAVE_A(argl);
      if (unev == exp->n - 1) {
        cont = APPL_ACCUM_LAST_ARG;
        exp = exp->kids[unev + 1];
        pc = EVAL_DISPATCH;
        break;
      }
      SAVE_ENV(env); SAVE_K(unev); SAVE_E(exp);
      cont = APPL_ACCUMULATE_ARG;
      exp = exp->kids[unev + 1];
      pc = EVAL_DISPATCH;
      break;
    case APPL_ACCUMULATE_ARG:
      exp = pop().e; unev = pop().k; env = pop().env; argl = pop().args;
      argl = adjoin(argl, val);
      unev++;
      pc = APPL_OPERAND_LOOP;
      break;
    case APPL_ACCUM_LAST_ARG:
      argl = pop().args;
      argl = adjoin(argl, val);
      proc = pop().v;
      pc = APPLY_DISPATCH;
      break;
    case APPLY_DISPATCH:
      proc = resolve(proc);
      if (proc->tag == T_PRIM) {
        int arity = prim_arity(proc), gathered = proc->n;
        Args *all = empty_args();
        for (int k = 0; k < gathered; k++) all = adjoin(all, proc->items[k]);
        for (int k = 0; k < argl->n; k++) all = adjoin(all, argl->vs[k]);
        cont = pop().k;
        if (all->n < arity) {
          val = make_prim(prim_id(proc), arity, all->n, all->vs);
          pc = cont;
          break;
        }
        val = prim_apply(prim_id(proc), all->vs);
        argl = drop_args(all, arity);
        pc = APPLY_EXCESS;
        break;
      }
      if (proc->tag == T_CLOSURE) {
        if (argl->n < proc->n) {
          Value *partial = make(T_CLOSURE);
          partial->n = proc->n - argl->n;
          partial->params = proc->params + argl->n;
          partial->body = proc->body;
          partial->env = extend_all(proc->env, argl->n, proc->params, argl->vs);
          val = partial;
          cont = pop().k;
          pc = cont;
          break;
        }
        env = extend_all(proc->env, proc->n, proc->params, argl->vs);
        exp = proc->body;
        argl = drop_args(argl, proc->n);
        if (argl->n == 0) {
          cont = pop().k;
          pc = EVAL_DISPATCH;
          break;
        }
        SAVE_A(argl);
        cont = COMPOUND_EXCESS;
        pc = EVAL_DISPATCH;
        break;
      }
      fail("not applicable");
      break;
    case COMPOUND_EXCESS:
      argl = pop().args;
      cont = pop().k;
      pc = APPLY_EXCESS;
      break;
    case APPLY_EXCESS:
      if (argl->n == 0) { pc = cont; break; }
      proc = val;
      SAVE_K(cont);
      pc = APPLY_DISPATCH;
      break;
    case DONE:
      return val;
    default:
      fail("unknown label");
    }
  }
}

static char *slurp(const char *path)
{
  FILE *f = fopen(path, "rb");
  if (f == NULL) fail("cannot open the program");
  size_t size = 0, used = 0;
  char *text = NULL;
  for (;;) {
    if (used + 4096 + 1 > size) {
      size = size == 0 ? 8192 : size * 2;
      text = realloc(text, size);
      if (text == NULL) fail("out of memory");
    }
    size_t got = fread(text + used, 1, 4096, f);
    used += got;
    if (got < 4096) break;
  }
  fclose(f);
  text[used] = '\0';
  return text;
}

int main(int argc, char **argv)
{
  if (argc != 2) fail("usage: eceval PROGRAM");
  input = slurp(argv[1]);
  Env *global = initial_env();
  for (;;) {
    char tag = read_tag();
    if (tag == 'E') break;
    if (tag != 'D') fail("expected an item");
    int recursive = (int)read_long();
    int n = (int)read_long();
    const char **names = allocate(sizeof(char *) * (size_t)(n > 0 ? n : 1));
    Node **rhs = allocate(sizeof(Node *) * (size_t)(n > 0 ? n : 1));
    for (int k = 0; k < n; k++) {
      names[k] = read_text();
      rhs[k] = read_expr();
    }
    if (recursive) {
      for (int k = n - 1; k >= 0; k--) global = extend(global, names[k], NULL);
      for (int k = 0; k < n; k++) cell_at(global, k)->v = eceval(rhs[k], global);
    } else {
      Value **values = allocate(sizeof(Value *) * (size_t)(n > 0 ? n : 1));
      for (int k = 0; k < n; k++) values[k] = eceval(rhs[k], global);
      global = extend_all(global, n, names, values);
    }
  }
  fflush(stdout);
  fprintf(stderr, "pushes %ld, maximum depth %ld\n", pushes, max_depth);
  return 0;
}
|c}
;;

let evaluate program =
  let* serialized = serialize (Check.items program) in
  build_and_run ~c_source:(runtime_c ^ eceval_c) ~inputs:[ "program.txt", serialized ]
;;

let programs =
  [ "factorial", Sec_5_33.factorial ^ "\nlet () = print_int (factorial 5)\n"
  ; ( "list length"
    , "let rec length xs = match xs with [] -> 0 | _ :: rest -> 1 + length rest\n\
       let () = print_int (length [ 1; 2; 3 ])\n" )
  ; ( "counter"
    , "let make_counter start = let cell = ref start in fun step -> cell := !cell + \
       step; !cell\n\
       let counter = make_counter 0\n\
       let () = print_int (counter 5); print_string \" \"; print_int (counter 7)\n" )
  ]
;;

let direct program =
  let out = Buffer.create 16 in
  let* _ = Sicp_ch4.Sec_4_1.run ~emit:(Buffer.add_string out) program in
  Ok (Buffer.contents out)
;;

let ex_5_51 () =
  all
    (fun (name, source) ->
       let* program = Sec_5_33.program ~filename:"ex_5_51.ml" source in
       let* c_output, statistics = evaluate program in
       let* direct_output = direct program in
       Ok
         (Printf.sprintf
            "%s: C evaluator %S, direct %S; %s"
            name
            c_output
            direct_output
            (String.trim statistics)))
    programs
;;
