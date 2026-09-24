(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.30 *)

(** Exercise 3.30: the ripple-carry adder for n-bit numbers. *)

(** [ripple_carry_adder sim a_list b_list s_list c] strings [n]
    full-adders together: [c] is the final carry wire, [s_list] the sum
    wires, most significant bit first. *)

(** [bits_of value width] is the bit list, most significant first;
    [value_of bits] is the inverse. *)

(** [add sim a_value b_value width] drives the bit patterns onto the
    input wires, propagates, and answers [(the sum value, the carry)]
    -- the harness the fixed examples and the property tests share. *)

(** [ex_3_30 ()] is [((5 + 3 on four bits), (7 + 7 on four bits), (15 +
    1 on four bits))], each answer the sum value with its carry. *)
let ripple_carry_adder = raise Sicp_common.Pending.Pending_solution

let bits_of = raise Sicp_common.Pending.Pending_solution
let value_of = raise Sicp_common.Pending.Pending_solution
let add = raise Sicp_common.Pending.Pending_solution
let ex_3_30 = raise Sicp_common.Pending.Pending_solution
