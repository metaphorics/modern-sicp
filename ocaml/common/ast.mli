(* SPDX-License-Identifier: GPL-3.0-only *)

(** The typed abstract syntax of the shared Scheme subset. The [Reader]
    produces it; the chapter 4 evaluator, the chapter 4 analyzer, the
    chapter 5 explicit-control evaluator, and the chapter 5 compiler all
    consume the same [expr]. *)

(** One object-language expression or definition. *)
type expr

(** One [define] form: a variable definition or a function definition. *)
type definition

type datum =
  | DInt of int
  | DFloat of float
  | DBool of bool
  | DString of string
  | DSymbol of string
  | DNil
  | DPair of datum * datum
  (** A quoted datum, as written after ['(quote ...)]. The chain of [DPair]
    constructors ends in [DNil] for proper lists and in any other datum for
    improper, dotted lists. *)

type view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Variable of string
  | Quote of datum
  | Definition of definition
  | Set of string * expr
  | If of expr * expr * expr option
  | Cond of (expr * expr list) list * expr list option
  | And of expr list
  | Or of expr list
  | Sequence of expr list
  | Let of (string * expr) list * expr list
  | Lambda of string list * expr list
  | Application of expr * expr list
  (** [view e] exposes the shape of [e]. A [Cond] clause is its test and the
    sequence of its body expressions; a clause written [(p)] alone carries
    [[]] as its body. [Sequence] is [begin]. The [Some] branch of [Cond] is
    the body of a trailing [(else ...)] clause. *)

type definition_view =
  | Define_variable of string * expr
  | Define_function of
      { name : string
      ; parameters : string list
      ; body : expr list
      } (** [view_definition d] exposes the shape of the definition [d]. *)

(** [view e] exposes the shape of [e] for evaluation, analysis, and
    compilation. *)
val view : expr -> view

(** [view_definition d] exposes the shape of the definition [d]. *)
val view_definition : definition -> definition_view

(** [int n] is the self-evaluating integer [n]. *)
val int : int -> expr

(** [float f] is the self-evaluating float [f]. *)
val float : float -> expr

(** [bool b] is the self-evaluating boolean [b]. *)
val bool : bool -> expr

(** [string s] is the self-evaluating string [s]. *)
val string : string -> expr

(** [variable name] is a reference to [name]. *)
val variable : string -> expr

(** [quote datum] is ['datum], the datum itself, unevaluated. *)
val quote : datum -> expr

(** [definition d] is the expression form of the definition [d]; a body or
    a program is a list of these. *)
val definition : definition -> expr

(** [set name e] is [(set! name e)]. *)
val set : string -> expr -> expr

(** [if_ c t alternative] is [(if c t)] or [(if c t a)]. *)
val if_ : expr -> expr -> expr option -> expr

(** [cond clauses else_body] is [(cond ...)] with one clause per test and
    body pair and, when present, the body of a trailing [else] clause.
    [Error (Invalid_form _)] when the clause list is empty or the [else]
    body is empty. *)
val cond : (expr * expr list) list -> expr list option -> (expr, Eval_error.t) result

(** [and_ operands] is [(and ...)]; [[]] is the empty conjunction. *)
val and_ : expr list -> expr

(** [or_ operands] is [(or ...)]; [[]] is the empty disjunction. *)
val or_ : expr list -> expr

(** [sequence body] is [(begin ...)]. [Error (Invalid_form _)] when [body]
    is empty. *)
val sequence : expr list -> (expr, Eval_error.t) result

(** [let_ bindings body] is [(let ((name e) ...) ...)]. [Error
    (Invalid_form _)] when [body] is empty. *)
val let_ : (string * expr) list -> expr list -> (expr, Eval_error.t) result

(** [lambda parameters body] is [(lambda (x ...) ...)]. [Error (Invalid_form
    _)] when [body] is empty. *)
val lambda : string list -> expr list -> (expr, Eval_error.t) result

(** [application operator operands] is the call of [operator] to the
    evaluated [operands]. *)
val application : expr -> expr list -> expr

(** [define_variable name e] is [(define name e)]. *)
val define_variable : string -> expr -> definition

(** [define_function name parameters body] is [(define (name x ...) ...)].
    [Error (Invalid_form _)] when [body] is empty. *)
val define_function
  :  string
  -> string list
  -> expr list
  -> (definition, Eval_error.t) result
