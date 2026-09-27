(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.49 *)

(** Exercise 3.49: the deadlock-avoidance scheme of exercise 3.48
    needs to know in advance which resources a process will use. The
    exercise asks for a scenario where a process must acquire one
    resource before it can learn the next one, so no static ordering
    helps. *)

(** [routing_deadlock_reachable ()] builds two accounts, each naming
    the other as its overdraft-routing target, and drives two
    processes that first hold their own account, read the routing
    record, and only then reach for the target. It answers whether
    both first locks are held, whether both onward acquisitions fail,
    and whether the deadlock therefore arises although both processes
    would gladly order their acquisitions if they could know them in
    advance. *)
val routing_deadlock_reachable : unit -> bool * bool * bool

(** [ex_3_49 ()] is the conjunction: all three facts of the scenario
    hold. *)
val ex_3_49 : unit -> bool
