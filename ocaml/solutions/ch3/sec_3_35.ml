(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.35 *)

(** Exercise 3.35: the squarer as a primitive constraint, filling the
    outline of the statement: the missing alternatives of
    [process_new_value], the body of [process_forget_value], the body
    of the [me] dispatch, and the connections. *)

open Sicp_ch3.Sec_3_3
open Constraints
module K = Constraints

let isqrt v =
  let r = Float.to_int (Float.sqrt (Float.of_int v)) in
  (* correct any rounding at the boundary *)
  if (r + 1) * (r + 1) <= v then r + 1 else if r * r > v then r - 1 else r
;;

let squarer a b =
  let rec me =
    { K.me =
        (function
          | K.I_have_a_value -> process_new_value ()
          | K.I_lost_my_value -> process_forget_value ())
    }
  and process_new_value () =
    if b.has_value ()
    then (
      let v = b.get_value () in
      if v < 0
      then invalid_arg (Printf.sprintf "square less than 0: SQUARER %d" v)
      else a.set_value (isqrt v) (K.Of_constraint me))
    else if a.has_value ()
    then (
      let v = a.get_value () in
      b.set_value (v * v) (K.Of_constraint me))
  and process_forget_value () =
    a.forget_value (K.Of_constraint me);
    b.forget_value (K.Of_constraint me);
    process_new_value ()
  in
  a.connect me;
  b.connect me;
  me
;;

let ex_3_35 () =
  let a = K.make_connector () in
  let b = K.make_connector () in
  ignore (squarer a b);
  b.set_value 49 K.User;
  let a_from_b = a.get_value () in
  b.forget_value K.User;
  a.set_value 6 K.User;
  let b_from_a = b.get_value () in
  a_from_b, b_from_a
;;

let negative_raises () =
  let a = K.make_connector () in
  let b = K.make_connector () in
  ignore (squarer a b);
  match b.set_value (-4) K.User with
  | () -> false
  | exception Invalid_argument _ -> true
;;
