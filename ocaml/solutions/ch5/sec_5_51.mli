(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.51: the explicit-control evaluator translated to C.

    The C program is the controller of section 5.4 written as one C
    function: registers are local variables, each label of the
    controller is a case of a dispatch loop, [continue] holds a label
    number, and the stack is an array with the same save/restore
    discipline and monitors.  The run-time support the exercise asks for
    is [runtime_c]: tagged values (integers, Booleans, unit, strings,
    tuples, constructors, lists, references, arrays, closures, and
    primitives), environments as chains of bindings with [let rec]
    cells, pattern matching, and the printing and array primitives.

    The C program does not parse OCaml.  Source is checked by the
    edition's admission (the [Check] of every engine) and handed to the
    C evaluator as its syntax tree, serialized by [serialize] into a
    prefix text the C reader rebuilds.  Floats and records are outside
    this rudimentary evaluator and are reported as unsupported before
    anything runs. *)

(** [runtime_c] is the C run-time support: values, environments,
    patterns, and primitives. *)
val runtime_c : string

(** [eceval_c] is the C explicit-control evaluator and its driver, to
    follow [runtime_c]. *)
val eceval_c : string

(** [serialize items] is the prefix text of [items] the C reader reads,
    or an [Invalid_form] error naming an unsupported construct. *)
val serialize : Sicp_common.Ast.item list -> (string, Sicp_common.Eval_error.t) result

(** [c_compiler] is the system C compiler command. *)
val c_compiler : string

(** [build_and_run ~c_source ~inputs] compiles [c_source] with
    [c_compiler] in a fresh temporary directory, writes each input file,
    runs the program with their paths as arguments, removes the
    directory, and answers the program's standard output and standard
    error. *)
val build_and_run
  :  c_source:string
  -> inputs:(string * string) list
  -> (string * string, Sicp_common.Eval_error.t) result

(** [ocaml_native_run source] compiles [source] with the pinned OCaml
    compiler under the subset flags and answers its standard output
    exactly, byte for byte. *)
val ocaml_native_run : string -> (string, Sicp_common.Eval_error.t) result

(** [evaluate program] runs [program] on the C evaluator and answers its
    output and its stack statistics line. *)
val evaluate
  :  Sicp_common.Check.program
  -> (string * string, Sicp_common.Eval_error.t) result

(** [programs] is the session: the factorial, a list length, and a
    counter closed over a reference. *)
val programs : (string * string) list

(** [ex_5_51 ()] runs each of [programs] on the C evaluator and on the
    direct evaluator of 4.1 and reports both outputs and the C
    evaluator's stack statistics. *)
val ex_5_51 : unit -> (string list, Sicp_common.Eval_error.t) result
