(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.48: compiling and running from inside an interpreted
    session.

    A session is a sequence of top-level items, each either evaluated by
    the explicit-control evaluator or compiled and run.  Compiling adds
    a labelled block of code to the controller; since a controller is
    assembled once, the session carries on in the machine assembled with
    the grown controller, keeping the global environment, so later
    interpreted items call the compiled procedures through the interface
    of exercise 5.45.  Item source is checked with the whole unit before
    the session starts, the edition's admission rule for every engine. *)

(** How the session runs one item. *)
type command =
  | Evaluate
  | Compile_and_run

(** [describe env item value] is the transcript text of [item]: each
    name it binds with its value in [env], or [value] when it binds
    none. *)
val describe : Sicp_common.Env.t -> Sicp_common.Ast.item -> Sicp_common.Value.t -> string

(** [run_session ~emit commands] runs each item with its command and
    answers one transcript line per item, naming the bindings it
    made. *)
val run_session
  :  emit:(string -> unit)
  -> (command * Sicp_common.Ast.item) list
  -> (string list, Sicp_common.Eval_error.t) result

(** [source] is the factorial, a doubling procedure, and a call of
    both. *)
val source : string

(** [ex_5_48 ()] compiles and runs the factorial, then evaluates the
    rest of [source]; the last item answers [120]. *)
val ex_5_48 : unit -> (string list, Sicp_common.Eval_error.t) result
