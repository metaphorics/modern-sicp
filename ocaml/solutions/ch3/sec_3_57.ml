(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.57 *)

(** Exercise 3.57: how many additions does the [fibs] definition
    perform, and how many without the memoized delay? The map class is
    [T]: the counts are measured, not argued, by running both stream
    implementations with a counting adder. *)

open Sicp_ch3.Sec_3_5

type thunk_stream = TCons of int * (unit -> thunk_stream)

(* The memoized fibs with a counting adder: every element past the
   first two costs exactly one addition, once. *)
let memoized_additions n =
  let adds = ref 0 in
  let counting_add a b =
    incr adds;
    a + b
  in
  let rec fibs =
    Streams.Cons
      ( 0
      , lazy
          (Streams.Cons
             (1, lazy (Infinite.stream_map2 counting_add (Streams.stream_cdr fibs) fibs)))
      )
  in
  (* stream_ref, not stream_take: take forces one tail past the last
     requested element, which would spend the count's n-2th addition on
     an element the count never asked for. *)
  ignore (Streams.stream_ref fibs (n - 1));
  !adds
;;

(* The same definition over plain thunks: every access re-runs the
   additions underneath it. *)
let thunk_additions n =
  let adds = ref 0 in
  let counting_add a b =
    incr adds;
    a + b
  in
  let rec t_add2 a b =
    TCons (counting_add (t_car a) (t_car b), fun () -> t_add2 (t_cdr a) (t_cdr b))
  and t_car = function
    | TCons (v, _) -> v
  and t_cdr = function
    | TCons (_, f) -> f ()
  and fibs = TCons (0, fun () -> TCons (1, fun () -> t_add2 (t_cdr fibs) fibs)) in
  let rec take m s = if m <= 0 then [] else t_car s :: take (m - 1) (t_cdr s) in
  ignore (take n fibs);
  !adds
;;

let ex_3_57 () =
  let n = 15 in
  let memo = memoized_additions n in
  let plain = thunk_additions n in
  n, memo, plain, memo = n - 2
;;
