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

(** [make_monitored f] is a monitored version [mf] of [f]: [mf
    How_many_calls] is the number of times [mf] has been called on an
    [Input]; [mf Reset_count] resets that counter to zero; [mf (Input
    x)] increments the counter and answers [Value (f x)]. *)
val make_monitored : ('a -> 'b) -> 'a message -> 'b response

(** [ex_3_02 ()] is the pair of answers the statement's calls to
    [(s 100)] and [(s 'how-many-calls?)] produce, in order. *)
val ex_3_02 : unit -> float response * float response
