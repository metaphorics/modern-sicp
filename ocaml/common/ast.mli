(* SPDX-License-Identifier: GPL-3.0-only *)

(** The checked syntax of the OCaml host subset.  [Check] is the only
    producer of these values from source; the direct evaluator, the
    analyzer, the lazy and search experiments, the query engine, the
    register machine, the explicit-control evaluator, and the compiler
    all consume the same [expr].  An [expr] exists only for source that
    parsed under the closed grammar of
    [spec/host-subsets/ocaml/grammar.md], resolved every name inside it,
    and type-checked under the pinned OCaml compiler. *)

(** A place in the source text.  Lines count from one, columns from
    zero. *)
type position =
  { line : int
  ; column : int
  }

(** One source span, from its first to just past its last character. *)
type span =
  { start : position
  ; stop : position
  }

(** The admitted scalar literals. *)
type scalar =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Unit

(** The arithmetic operators.  [Rem] is integer remainder with the sign
    of its left operand.  The float operations are distinct from the
    integer ones; the subset admits no implicit conversion. *)
type arith =
  | Add
  | Sub
  | Mul
  | Div
  | Rem
  | Addf
  | Subf
  | Mulf
  | Divf

(** The admitted comparisons.  [Eq] and [Ne] hold only on [unit], [int],
    [float], [bool], and [string].  The ordered comparisons hold only on
    [int], [float], and [string]. *)
type comparison =
  | Eq
  | Ne
  | Lt
  | Le
  | Gt
  | Ge

(** One expression of the checked syntax. *)
type expr

(** One pattern of a [Match] case. *)
type pattern

(** One binding of a [Let] group.  [name] is [None] for the [_ = e] and
    [() = e] forms. *)
type binding =
  { name : string option
  ; rhs : expr
  }

(** The shape of one expression. *)
type view =
  | Scalar of scalar (** A literal. *)
  | Var of string (** A resolved variable reference. *)
  | Let of bool * binding list * expr
  (** [Let (is_rec, bindings, body)] is one [let] group.  A recursive
      group binds its names before its right-hand sides run; a parallel
      group evaluates its right-hand sides in the enclosing scope. *)
  | Fun of string list * expr (** A curried function of one or more parameters. *)
  | Apply of expr * expr list
  (** An application.  The operator runs first, then the operands in
      list order, the teaching evaluators' standardized left-to-right
      rule. *)
  | If of expr * expr * expr (** A conditional.  Only its condition and one branch run. *)
  | Match of expr * (pattern * expr) list
  (** A match.  The scrutinee runs once, then cases in source order; the
      case set is exhaustive. *)
  | Tuple of expr list (** A tuple of two or more components. *)
  | Construct of string * expr list
  (** A variant construction.  Its payload fields are in declaration
      order. *)
  | Record of (string * expr) list
  (** A record construction supplying every declared field exactly
      once. *)
  | Field of expr * string (** A record field access. *)
  | Sequence of expr * expr
  (** A sequence.  Its left expression runs first and its value is
      discarded. *)
  | And of expr * expr
  (** A conjunction.  Its right expression runs only when the left one
      is [true]. *)
  | Or of expr * expr
  (** A disjunction.  Its right expression runs only when the left one
      is [false]. *)
  | Arith of arith * expr * expr (** An arithmetic operation. *)
  | Compare of comparison * expr * expr (** A comparison. *)
  | Nil (** The empty list. *)
  | Cons of expr * expr (** A list construction. *)
  | Concat of expr * expr (** A string concatenation. *)
  | Not of expr (** A Boolean negation. *)
  | Neg of expr (** An integer or float negation. *)
  | Deref of expr (** A reference read. *)
  | Assign of expr * expr (** A reference write, answering [unit]. *)
  | Make_ref of expr (** A reference allocation. *)

(** The shape of one pattern. *)
type pattern_view =
  | PWildcard
  | PVar of string
  | PScalar of scalar
  | PTuple of pattern list
  | PConstruct of string * pattern list
  | PNil
  | PCons of pattern * pattern

(** The shape of one type declaration.  [constructors] pairs each
    constructor with its payload arity and [fields] lists record fields
    in declaration order; an abbreviation has both empty. *)
type type_shape =
  { type_name : string
  ; type_params : string list
  ; constructors : (string * int) list
  ; fields : string list
  }

(** One top-level declaration.  A complete admitted unit is
    [Check.program], which only [Check.check] produces. *)
type item =
  | Type_item of type_shape
  | Value_item of bool * binding list
  (** A top-level [let] group, recursive when its flag is [true]. *)

(** [view e] is the shape of [e]. *)
val view : expr -> view

(** [view_pattern p] is the shape of [p]. *)
val view_pattern : pattern -> pattern_view

(** [at e] is the source span of [e]. *)
val at : expr -> span

(** [pattern_span p] is the source span of [p]. *)
val pattern_span : pattern -> span

(** {1 Smart constructors}

    These build checked nodes for derived-expression rewrites and for
    tests.  Source programs reach these shapes only through
    [Check.check]. *)

val scalar : ?at:span -> scalar -> expr
val var : ?at:span -> string -> expr
val let_ : ?at:span -> bool -> binding list -> expr -> expr
val fun_ : ?at:span -> string list -> expr -> expr
val apply : ?at:span -> expr -> expr list -> expr
val if_ : ?at:span -> expr -> expr -> expr -> expr
val match_ : ?at:span -> expr -> (pattern * expr) list -> expr
val tuple : ?at:span -> expr list -> expr
val construct : ?at:span -> string -> expr list -> expr
val record : ?at:span -> (string * expr) list -> expr
val field : ?at:span -> expr -> string -> expr
val sequence : ?at:span -> expr -> expr -> expr
val and_ : ?at:span -> expr -> expr -> expr
val or_ : ?at:span -> expr -> expr -> expr
val arith : ?at:span -> arith -> expr -> expr -> expr
val compare_ : ?at:span -> comparison -> expr -> expr -> expr
val nil : ?at:span -> unit -> expr
val cons : ?at:span -> expr -> expr -> expr
val concat : ?at:span -> expr -> expr -> expr
val not_ : ?at:span -> expr -> expr
val neg : ?at:span -> expr -> expr
val deref : ?at:span -> expr -> expr
val assign : ?at:span -> expr -> expr -> expr
val make_ref : ?at:span -> expr -> expr

(** [wildcard ()] is the [_] pattern. *)
val wildcard : ?at:span -> unit -> pattern

(** [pvar name] is a variable pattern. *)
val pvar : ?at:span -> string -> pattern

(** [pscalar s] is a literal pattern. *)
val pscalar : ?at:span -> scalar -> pattern

(** [ptuple ps] is a tuple pattern. *)
val ptuple : ?at:span -> pattern list -> pattern

(** [pconstruct name ps] is a constructor pattern. *)
val pconstruct : ?at:span -> string -> pattern list -> pattern

(** [pnil ()] is the empty-list pattern. *)
val pnil : ?at:span -> unit -> pattern

(** [pcons head tail] is a list cons pattern. *)
val pcons : ?at:span -> pattern -> pattern -> pattern

(** [map_children f e] is [e] with [f] applied to each immediate
    subexpression, in source order, keeping [e]'s span: the building
    block of the derived-expression rewrites. *)
val map_children : (expr -> expr) -> expr -> expr
