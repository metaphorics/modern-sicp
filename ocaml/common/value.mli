(* SPDX-License-Identifier: GPL-3.0-only *)

(** The runtime values of the OCaml host subset: the data the direct and
    analyzed evaluators compute, the explicit-control evaluator moves
    through registers, the compiler's target machine stores, and the
    named lazy and search experiments observe.

    The representation follows the guest evaluator-kernel witness of
    [spec/host-subsets/ocaml/grammar.md] section 11: scalars, tuples,
    variant constructors, lists, closures over lexical environments,
    references, arrays, hash tables, and -- for the lazy experiment --
    explicit thunk cells carrying an expression, an environment, and an
    optional memoized result (section 10).

    [t] and the environment type [env] are mutually recursive through
    closures, so both live in this module; the environment operations
    publish through [Env], whose [t] is [Value.env]. *)

(** One runtime value. *)
type t

(** A lexical environment: immutable bindings, newest first.  Built and
    searched through [Env]. *)
type env

(** The lazy experiment's thunk cell (grammar section 10): a delayed
    expression with its environment, or the memoized result of its first
    forcing.  The cell is a reference so [force] can memoize in place
    and the experiment can count forces explicitly. *)
type thunk_state =
  | Delayed of Ast.expr * env
  | Forced of t

(** The evaluator's own application function, handed to primitives so
    they can call guest procedures (the [List] higher-order members). *)
type apply_fun = t -> t list -> (t, Eval_error.t) result

(** A primitive procedure: applied to its saturated operands, it answers
    the result or a recoverable failure through the one shared error
    channel. *)
type primitive =
  { prim_name : string
  ; prim_arity : int
  ; prim_apply : apply_fun -> t list -> (t, Eval_error.t) result
  }

(** A compiled procedure of section 5.5: the entry label of its code in
    the compiled-code machine, its parameters, and the environment it
    closes over. *)
type compiled =
  { entry : string
  ; entry_parameters : string list
  ; entry_env : env
  }

(** A guest hash table: immutable structural keys to values (grammar
    section 6). *)
type table

(** The parts of a closure that evaluation and printing need. *)
type closure_view =
  { name : string option
  ; parameters : string list
  ; body : Ast.expr
  ; env : env
  }

(** The shape of a value. *)
type view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Unit
  | Tuple of t list
  | Constructor of string * t list
  | Nil
  | Cons of t * t
  | Record of (string * t) list
  | Closure of closure_view
  | Primitive of primitive
  | Partial of primitive * t list
  (** A primitive applied to fewer operands than its arity; the next
      application continues it. *)
  | Compiled of compiled (** A procedure whose body is compiled machine code. *)
  | Ref of t ref
  | Array of t array
  | Table of table
  | Thunk of thunk_state ref

(** [view v] exposes the shape of [v] without exposing the
    representation. *)
val view : t -> view

(** [int n] is the machine integer [n]; arithmetic wraps at the target
    width (grammar section 6). *)
val int : int -> t

(** [float f] is the host float [f]. *)
val float : float -> t

(** [bool b] is the Boolean [b]. *)
val bool : bool -> t

(** [string s] is the string [s]. *)
val string : string -> t

(** [unit] is the unit value. *)
val unit : t

(** [tuple parts] is the tuple of [parts]. *)
val tuple : t list -> t

(** [construct name fields] is the variant value whose constructor is
    [name] and whose payload fields are [fields]. *)
val construct : string -> t list -> t

(** [nil] is the empty list. *)
val nil : t

(** [cons head tail] is the list cell ([head], [tail]). *)
val cons : t -> t -> t

(** [record fields] is the record whose fields are [fields], each field
    appearing exactly once. *)
val record : (string * t) list -> t

(** [closure ~name ~parameters ~body ~env] is the closure of a [fun]. *)
val closure
  :  name:string option
  -> parameters:string list
  -> body:Ast.expr
  -> env:env
  -> t

(** [primitive ~name ~arity apply] is the primitive procedure [name]. *)
val primitive
  :  name:string
  -> arity:int
  -> (apply_fun -> t list -> (t, Eval_error.t) result)
  -> t

(** [partial p args] is the primitive [p] under-applied to [args]. *)
val partial : primitive -> t list -> t

(** [compiled ~entry ~parameters ~env] is a compiled procedure. *)
val compiled : entry:string -> parameters:string list -> env:env -> t

(** [ref_value v] is a fresh reference cell holding [v]. *)
val ref_value : t -> t

(** [array a] is the array value of [a]. *)
val array : t array -> t

(** [table ()] is a fresh empty hash table. *)
val table : unit -> t

(** [thunk ~expr ~env] is a fresh delayed thunk cell. *)
val thunk : expr:Ast.expr -> env:env -> t

(** [forced v] is a thunk cell already holding the memoized [v]. *)
val forced : t -> t

(** [thunk_state_of v] is the state of the thunk cell [v], or [None]
    when [v] is not a thunk. *)
val thunk_state_of : t -> thunk_state ref option

(** [set_thunk_state cell state] memoizes [state] in [cell]. *)
val set_thunk_state : thunk_state ref -> thunk_state -> unit

(** [key_equal a b] is the structural equality of grammar section 6 for
    hash-table keys: immutable integers, strings, tuples, and
    constructor trees. *)
val key_equal : t -> t -> bool

(** [table_find tbl key] is the value bound to [key], or [None]. *)
val table_find : t -> t -> t option

(** [table_replace tbl key value] binds [key] to [value], replacing any
    binding. *)
val table_replace : t -> t -> t -> unit

(** [table_remove tbl key] removes [key]. *)
val table_remove : t -> t -> unit

(** [table_length tbl] is the number of bindings. *)
val table_length : t -> int

(** [to_string v] is the printed diagnostic form of [v]: strings quoted,
    closures as [closure name]. *)
val to_string : t -> string

(** [compare_scalars a b] orders the scalars [a] and [b] under the
    ordered comparison rules of grammar section 3 ([int], [float],
    [string]); it is [Error] on every other pair. *)
val compare_scalars : t -> t -> (int, Eval_error.t) result

(** [equal_scalars a b] is equality under grammar section 3 ([unit],
    [int], [float], [bool], [string]); it is [Error] on every other
    pair. *)
val equal_scalars : t -> t -> (bool, Eval_error.t) result

(** {1 Environments}

    The representation is owned here because closures capture it; [Env]
    publishes the operations.  Do not call these directly. *)

(** See [Env.empty]. *)
val env_empty : unit -> env

(** See [Env.extend]. *)
val env_extend : (string * t) list -> env -> env

(** See [Env.extend_recursive]. *)
val env_extend_recursive : string list -> env -> env * t option ref list

(** See [Env.fill]. *)
val env_fill : t option ref -> t -> unit

(** See [Env.find]. *)
val env_find : env -> string -> t option

(** See [Env.find_at]. *)
val env_find_at : env -> int -> (string * t) option
