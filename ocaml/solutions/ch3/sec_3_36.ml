(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.36 *)

(** Exercise 3.36, replaced for this edition: the book asks for the
    environment diagram in which (for-each-except setter
    inform-about-value constraints) runs inside set-value!. This
    edition traces the call graph those closures make: setting a wakes
    every constraint attached to a except the setter, each wake-up is
    one dispatch of the constraint's [me] record, and each dispatch
    runs the constraint's own process_new_value closure. A logging
    constraint records the dispatches. *)

open Sicp_ch3.Sec_3_3
open Constraints
module K = Constraints

let ex_3_36 () =
  let a = K.make_connector () in
  let b = K.make_connector () in
  let calls = ref [] in
  let note s = calls := s :: !calls in
  (* a constraint on a, logging its dispatches: the book's adder box *)
  let spy =
    let process_new_value () = note "process_new_value" in
    let process_forget_value () = note "process_forget_value" in
    { K.me =
        (function
          | K.I_have_a_value ->
            note "me: I-have-a-value";
            process_new_value ()
          | K.I_lost_my_value ->
            note "me: I-lost-my-value";
            process_forget_value ())
    }
  in
  a.connect spy;
  ignore b;
  calls := [];
  a.set_value 10 K.User;
  let trace_set = List.rev !calls in
  (* the user retracts; the same closures run the forget path *)
  calls := [];
  a.forget_value K.User;
  let trace_forget = List.rev !calls in
  trace_set, trace_forget
;;
