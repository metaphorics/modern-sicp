(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.33: the alternative factorial compiled; exercise 5.33a:
    its compiled code hand-optimized and measured.  The module also
    holds the measuring harness the section's solutions share: compiled
    code runs on the machine of [Sicp_ch5.Sec_5_5.load] under the
    monitored stack of 5.2.4 and the machine's instruction counter. *)

(** [program ~filename source] is the admitted unit of [source]; a
    rejection is reported as an [Invalid_form] error carrying the
    admission diagnostic. *)
val program
  :  filename:string
  -> string
  -> (Sicp_common.Check.program, Sicp_common.Eval_error.t) result

(** One measured run of compiled code: the value left in [val], the
    guest's printed output, the instructions executed, and the total
    pushes and maximum depth of the stack. *)
type run =
  { value : Sicp_common.Value.t
  ; output : string
  ; steps : int
  ; pushes : int
  ; depth : int
  }

(** [run_code ?operations code] loads [code] with [operations] ahead of
    the runtime operations and runs it from its first instruction. *)
val run_code
  :  ?operations:(string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op) list
  -> Sicp_ch5.Sec_5_5.seq
  -> (run, Sicp_common.Eval_error.t) result

(** [run_program ?operations program] is [run_code] of the compilation
    of [program]. *)
val run_program
  :  ?operations:(string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op) list
  -> Sicp_common.Check.program
  -> (run, Sicp_common.Eval_error.t) result

(** [factorial] is the section's recursive factorial definition. *)
val factorial : string

(** [factorial_alt] is the exercise's alternative, which multiplies
    [n] on the left of the recursive call. *)
val factorial_alt : string

(** [definition_code source] is the compilation, with target [val] and
    linkage [Next], of the right-hand side of the one recursive
    definition [source] consists of. *)
val definition_code : string -> (Sicp_ch5.Sec_5_5.seq, Sicp_common.Eval_error.t) result

(** [ex_5_33 ()] is one line per definition: its statement count, the
    stack operations of its compilation, and its answer and step count
    for [5].  The book's factorial saves [env] around the recursive call
    to look [n] up afterwards; the alternative looks [n] up first and
    saves [arg1] instead, so both do the same work. *)
val ex_5_33 : unit -> (string list, Sicp_common.Eval_error.t) result

(** [optimized_body] is the hand-optimized code of [factorial_alt]'s
    body: the constant [1] stays in [arg2] for the subtraction, the
    argument list is built from a constant, the call jumps straight to
    the entry of the procedure already in [proc], and the primitive
    branch and unreferenced labels are gone. *)
val optimized_body : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [ex_5_33a ()] runs the naive and the hand-optimized compilations of
    [factorial_alt 5] and reports both instruction counts, both answers,
    and the win. *)
val ex_5_33a : unit -> (string list, Sicp_common.Eval_error.t) result
