(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.46: the analysis of exercise 5.45 for the tree-recursive
    Fibonacci procedure.

    The pushes of all three versions grow with the call tree, so the
    ratios stay close to constants as [n] grows, while the depths grow
    linearly.  The compiled code pushes less than the machine of Figure
    5.12 here: each non-leaf call saves [continue] for the return
    linkage, [env] to find [n] for the right sum, and the left sum in
    [arg1], though at most two are held at once (the left call runs
    under [continue] and [env], the right one under [continue] and
    [arg1]), where the machine saves [continue] and [n] for the first
    recursive call and [continue] and [val] for the second.  The machine
    keeps the shallower stack. *)

(** [fib] is the tree-recursive Fibonacci definition. *)
val fib : string

(** [fib_machine] is the Fibonacci machine of Figure 5.12. *)
val fib_machine : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** [ex_5_46 ()] is the comparison at [n] = 5, 6, and 7. *)
val ex_5_46 : unit -> (string list, Sicp_common.Eval_error.t) result
