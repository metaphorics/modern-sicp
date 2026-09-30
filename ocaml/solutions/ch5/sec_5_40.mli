(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.40: the compile-time environment.

    A compile-time environment is the list of frames, innermost first,
    that the compiled code's run-time environment will have at a point
    of the program.  It grows exactly where compiled code extends the
    run-time environment: a [fun]'s parameters, the names of a [let]
    group (whose body compiles as the body of a [fun] of those names),
    the named bindings of a [let rec] group before its right-hand sides,
    the variables of a [match] case's pattern, and each top-level
    binding.  [environments] computes it for every expression of a unit
    in one walk that mirrors those extensions, and the lexical compiler
    of exercise 5.42 consults it at each variable it compiles. *)

(** A compile-time environment, innermost frame first. *)
type frames = string list list

(** The compile-time environments of one unit. *)
type t

(** [environments items] is the compile-time environment of every
    expression of [items]. *)
val environments : Sicp_common.Ast.item list -> t

(** [find t e] is the compile-time environment [e] compiles in, or
    [None] when [e] is not an expression of the unit. *)
val find : t -> Sicp_common.Ast.expr -> frames option

(** [variables t] is every variable reference of the unit, in source
    order, with the compile-time environment it compiles in. *)
val variables : t -> (string * frames) list

(** [frames_to_string frames] renders [frames] innermost first. *)
val frames_to_string : frames -> string

(** [nested_example] is the section's nested [fun] example applied to
    [3 4] and then to [1 2 3 4 5]. *)
val nested_example : string

(** [ex_5_40 ()] dumps the compile-time environment of every variable
    of [nested_example]. *)
val ex_5_40 : unit -> (string list, Sicp_common.Eval_error.t) result
