(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.14 *)

(** Exercise 3.14: [mystery] reverses a mutable list in place. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [mystery x] is the reversal of [x], built from [x]'s own pairs; [x]
    afterwards holds only its first element. *)
val mystery : mobj -> mobj

(** [ex_3_14 ()] is [(the printed v before, the printed v after the
    call, the printed w)] for [v] the list (a b c d). *)
val ex_3_14 : unit -> string * string * string
