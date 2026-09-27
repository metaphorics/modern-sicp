(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.33 *)

(** Exercise 3.33: the average of two connectors as a constraint. *)

open Sicp_ch3.Sec_3_3

(** [averager a b c] constrains [c] to be the average of [a] and [b]:
    the sum feeds a multiplier whose other factor is the constant 2. *)
val averager
  :  Constraints.connector
  -> Constraints.connector
  -> Constraints.connector
  -> unit

(** [ex_3_33 ()] is [(the average of 6 and 14, the missing addend b
    when c = 12 and a = 4, the a then in force, the value of c at the
    end)], all through one network used in both directions. *)
val ex_3_33 : unit -> int * int * int * int
