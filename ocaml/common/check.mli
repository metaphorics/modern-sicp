(* SPDX-License-Identifier: GPL-3.0-only *)

(** Source admission for the OCaml host subset.  [check] parses the
    closed grammar of [spec/host-subsets/ocaml/grammar.md], rejects every
    construct outside it, type-checks the whole unit with the pinned
    compiler, enforces the subset typing rules the compiler cannot, and
    only then answers a [program].  No engine entry point accepts source
    that has not passed here. *)

(** The admission stage that rejected a source. *)
type diagnostic_kind =
  | Syntax (** The source is not the closed concrete syntax of the subset. *)
  | Unsupported (** The source is valid host OCaml that the subset does not admit. *)
  | Type_error (** The pinned type checker rejects the source. *)
  | Contract
  (** The pinned compiler accepts the source but a subset rule does
      not, such as a non-exhaustive match or a comparison on a
      non-scalar type. *)

(** One admission rejection, with its source span. *)
type diagnostic =
  { kind : diagnostic_kind
  ; at : Ast.span
  ; detail : string
  }

(** A complete admitted compilation unit.  Only [check] produces these. *)
type program

(** [check ~filename source] is the admitted program of [source], or the
    first rejection found.  A rejection names the first failing stage;
    the stages run in order: subset lexical scan, pinned parser,
    closed-grammar walk for excluded constructs, pinned type checker,
    name resolution against the fixed standard-library surface, then
    subset contract rules.  A rejected source has no effects. *)
val check : filename:string -> string -> (program, diagnostic) result

(** The named experiments of grammar section 10 extend the admitted
    value surface: the lazy experiment admits the names [delay] and
    [force] and the search experiment admits [amb] and [require].  All
    other admission stages run exactly as [check] runs them. *)
type experiment =
  | Core
  | Lazy
  | Search

(** [check_experiment ~experiment ~filename source] is [check] with the
    experiment's value surface admitted. *)
val check_experiment
  :  experiment:experiment
  -> filename:string
  -> string
  -> (program, diagnostic) result

(** [check_experiment_with ~experiment ~forms ~filename source] is
    [check_experiment] with each named form of [forms] admitted as well:
    [(name, stub)] adds [name] to the value surface and types it through
    [stub], a declaration of [name] in OCaml, for instance
    [("if_fail", "let if_fail x _y = x")].  An exercise that implements a
    new special form admits its programs this way. *)
val check_experiment_with
  :  experiment:experiment
  -> forms:(string * string) list
  -> filename:string
  -> string
  -> (program, diagnostic) result

(** [items p] is the declarations of [p] in source order. *)
val items : program -> Ast.item list

(** [program_span p] is the source span of [p]. *)
val program_span : program -> Ast.span

(** [diagnostic_to_string d] is [d] as one human-readable line. *)
val diagnostic_to_string : diagnostic -> string

(** [kind_to_string k] is the name of [k]. *)
val kind_to_string : diagnostic_kind -> string
