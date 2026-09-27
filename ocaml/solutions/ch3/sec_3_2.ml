(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.2 *)

(** Exercise 3.2: a monitored wrapper that counts and resets calls to
    an underlying procedure. Scheme's untyped [how-many-calls?] and
    [reset-count] symbols, mixed into [f]'s own argument domain,
    become a closed [message] variant; the three possible shapes of
    answer become a closed [response] variant. *)

type 'a message =
  | How_many_calls
  | Reset_count
  | Input of 'a

type 'a response =
  | Value of 'a
  | Count of int
  | Reset

let make_monitored f =
  let calls = ref 0 in
  fun message ->
    match message with
    | How_many_calls -> Count !calls
    | Reset_count ->
      calls := 0;
      Reset
    | Input x ->
      incr calls;
      Value (f x)
;;

let ex_3_02 () =
  let s = make_monitored sqrt in
  let first = s (Input 100.) in
  let second = s How_many_calls in
  first, second
;;
