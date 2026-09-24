(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.30 *)

(** Exercise 3.30: the ripple-carry adder for n-bit numbers. *)

open Sicp_ch3.Sec_3_3

(** [ripple_carry_adder sim a_list b_list s_list c] strings [n]
    full-adders together: [c] is the final carry wire, [s_list] the sum
    wires, most significant bit first. *)
val ripple_carry_adder
  :  Circuit.sim
  -> Circuit.wire list
  -> Circuit.wire list
  -> Circuit.wire list
  -> Circuit.wire
  -> unit

(** [bits_of value width] is the bit list, most significant first;
    [value_of bits] is the inverse. *)
val bits_of : int -> int -> int list

val value_of : int list -> int

(** [add sim a_value b_value width] drives the bit patterns onto the
    input wires, propagates, and answers [(the sum value, the carry)]
    -- the harness the fixed examples and the property tests share. *)
val add : Circuit.sim -> int -> int -> int -> int * int

(** [ex_3_30 ()] is [((5 + 3 on four bits), (7 + 7 on four bits), (15 +
    1 on four bits))], each answer the sum value with its carry. *)
val ex_3_30 : unit -> (int * int) * (int * int) * (int * int)
