(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.27 *)

(** Exercise 3.27: memo-fib. This edition's memo table is a [Hashtbl]
    keyed by the integer argument, and the environment diagram of the
    book becomes a trace: the counters answer, in numbers, the
    statement's question of how many steps the computation takes.
    [memo_fib_steps 25] computes each [fib k] once -- the [computes]
    counter grows linearly -- while the un-memoized [fib] of
    @ref{1.2.2} makes a call count that grows exponentially. *)

let memoize f =
  let table : (int, int) Hashtbl.t = Hashtbl.create 16 in
  let hits = ref 0 in
  let computes = ref 0 in
  let memo x =
    match Hashtbl.find_opt table x with
    | Some result ->
      incr hits;
      result
    | None ->
      incr computes;
      let result = f x in
      Hashtbl.replace table x result;
      result
  in
  memo, hits, computes
;;

(* The exponential fib of @ref{1.2.2}, with its call counter. *)
let fib_calls = ref 0

let rec fib n =
  incr fib_calls;
  if n = 0 then 0 else if n = 1 then 1 else fib (n - 1) + fib (n - 2)
;;

(* The statement's memo-fib: the recursive calls go through the memo
   wrapper, so each fib value is computed once. *)
let memo_fib_steps n =
  let table : (int, int) Hashtbl.t = Hashtbl.create 16 in
  let hits = ref 0 in
  let computes = ref 0 in
  let rec memo_fib k =
    match Hashtbl.find_opt table k with
    | Some result ->
      incr hits;
      result
    | None ->
      incr computes;
      let result = if k <= 1 then k else memo_fib (k - 1) + memo_fib (k - 2) in
      Hashtbl.replace table k result;
      result
  in
  let result = memo_fib n in
  result, !hits, !computes
;;

let ex_3_27 () =
  let memo_25, hits_25, computes_25 = memo_fib_steps 25 in
  fib_calls := 0;
  let plain_25 = fib 25 in
  memo_25, plain_25, computes_25, hits_25, !fib_calls, computes_25 < !fib_calls
;;
