(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.30 *)

(** Exercise 3.30: the ripple-carry adder of @ref{Figure 3.27}: n
    full-adders in a row, each stage's carry-out feeding the next
    stage's carry-in, the last stage's carry-out the adder's carry
    wire. [ex_3_30] adds a few fixed operands; [ripple_value] turns the
    sum wires into an integer for the property tests of the test
    suite. *)

open Sicp_ch3.Sec_3_3
open Circuit
module C = Circuit

let ripple_carry_adder sim a_list b_list s_list c =
  let rec loop a bs ss carry_in =
    match a, bs, ss with
    | [ a ], [ b ], [ s ] -> C.full_adder sim a b carry_in s c
    | a :: a_tail, b :: b_tail, s :: s_tail ->
      let carry_out = C.make_wire () in
      C.full_adder sim a b carry_in s carry_out;
      loop a_tail b_tail s_tail carry_out
    | _ -> invalid_arg "ripple-carry-adder: lists of unequal length"
  in
  (* the first stage's carry-in is a wire at signal 0 *)
  loop a_list b_list s_list (C.make_wire ())
;;

let bits_of value n =
  let rec go v i acc = if i = n then acc else go (v / 2) (i + 1) ((v mod 2) :: acc) in
  go value 0 []
;;

let value_of bits = List.fold_left (fun acc b -> (2 * acc) + b) 0 bits

let add sim a_value b_value width =
  (* bits_of/value_of are most-significant-bit first for readable
     printing; the adder chain itself is wired least-significant bit
     first, since the carry starts at the low end. *)
  let a_bits_lsb = List.rev (bits_of a_value width) in
  let b_bits_lsb = List.rev (bits_of b_value width) in
  let as_ = List.map (fun _ -> C.make_wire ()) a_bits_lsb in
  let bs = List.map (fun _ -> C.make_wire ()) b_bits_lsb in
  let ss = List.map (fun _ -> C.make_wire ()) as_ in
  let c = C.make_wire () in
  ripple_carry_adder sim as_ bs ss c;
  List.iter2 (fun w v -> w.set_signal v) as_ a_bits_lsb;
  List.iter2 (fun w v -> w.set_signal v) bs b_bits_lsb;
  C.propagate sim;
  let s_value = value_of (List.rev (List.map (fun w -> w.get_signal ()) ss)) in
  let carry = c.get_signal () in
  s_value, carry
;;

let ex_3_30 () =
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  (* 5 + 3 = 8, 7 + 7 = 14, 15 + 1 = 16 with the carry out *)
  let s1, c1 = add sim 5 3 4 in
  let s2, c2 = add sim 7 7 4 in
  let s3, c3 = add sim 15 1 4 in
  (s1, c1), (s2, c2), (s3, c3)
;;
