(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.19 *)

(** Exercise 3.19: constant-space cycle detection, tortoise and hare. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [contains_cycle_constant x] is whether the cdr chain from [x]
    loops, decided with two moving pointers and no auxiliary storage. *)
val contains_cycle_constant : mobj -> bool

(** [ex_3_19 ()] is [(the answer for the plain list (a b c), for the
    (a b c) ring of exercise 3.13, for the one-pair self-cycle, and for
    a list whose cdr chain shares a suffix but never loops)]. *)
val ex_3_19 : unit -> bool * bool * bool * bool
