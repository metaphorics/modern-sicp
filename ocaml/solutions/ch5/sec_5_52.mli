(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.52: a C back end for the compiler, and the metacircular
    evaluator compiled to C.

    The back end translates the compiler's typed instruction sequences
    statement by statement into one C function: each register is a
    global word, each label a case of a dispatch switch that [goto]
    reaches directly, [continue] and compiled-procedure entries hold
    label numbers, and the stack keeps the machine's save/restore
    discipline.  Each machine operation becomes a call into the run-time
    support: [Sec_5_51.runtime_c] for values, environments, patterns,
    and primitives, and [compiled_runtime_c] for the compiled-procedure
    operations.  Syntax constants become C data: variable names become
    strings, operators become operation codes, parameter lists and
    binding groups become name arrays, and patterns become pattern trees
    built at start-up.

    Compiling [Sicp_ch5.Metacircular.with_guest] this way produces a C
    program that is the section 4.1 evaluator running its guest program;
    the system C compiler builds it and its output is compared with the
    compiled code's run on the section 5.5 machine and with the native
    run of the same guest source. *)

(** [compiled_runtime_c] is the C support for the compiled-procedure
    operations, to follow [Sec_5_51.runtime_c]. *)
val compiled_runtime_c : string

(** [c_string s] is [s] as a C string literal. *)
val c_string : string -> string

(** [emit code] is the C translation of [code]: its data, its dispatch
    function, and [main]; an operation or constant the back end does not
    translate is an [Invalid_form] error. *)
val emit : Sicp_ch5.Sec_5_5.seq -> (string, Sicp_common.Eval_error.t) result

(** [compile_to_c program] is the complete C program of the compilation
    of [program]. *)
val compile_to_c : Sicp_common.Check.program -> (string, Sicp_common.Eval_error.t) result

(** [run_c program] builds and runs [compile_to_c program] and answers
    its output. *)
val run_c : Sicp_common.Check.program -> (string, Sicp_common.Eval_error.t) result

(** [ex_5_52 ()] compiles the metacircular evaluator with the guest
    factorial and with the guest counter of exercise 5.50 to C, runs
    both, and requires the factorial's output to agree byte for byte
    across the C program, the machine run, and the native run.  Any
    disagreement is an error, not a line. *)
val ex_5_52 : unit -> (string list, Sicp_common.Eval_error.t) result
