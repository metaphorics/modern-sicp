(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.37: [preserving] made to save unconditionally.

    With the mechanism intact a register is saved around a sequence only
    when the sequence modifies it and the code after it needs it.  The
    blind variant saves every register the compiler names, so every
    extra [save] it emits is one whose register the first sequence
    leaves alone or the second sequence never reads. *)

(** [always_preserving regs first second] runs [first] then [second],
    saving and restoring every register of [regs] around [first]. *)
val always_preserving
  :  string list
  -> Sicp_ch5.Sec_5_5.seq
  -> Sicp_ch5.Sec_5_5.seq
  -> Sicp_ch5.Sec_5_5.seq

(** [compile_without_preserving state e target linkage] is the compiler
    with [always_preserving] in place of [preserving]. *)
val compile_without_preserving
  :  Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [ex_5_37 ()] compares both compilers on the combination
    [f (g 1) 2] (its stack operations) and on the factorial (its size,
    and the pushes and depth of [factorial 5]). *)
val ex_5_37 : unit -> (string list, Sicp_common.Eval_error.t) result
