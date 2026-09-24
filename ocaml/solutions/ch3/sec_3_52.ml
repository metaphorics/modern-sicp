(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.52 *)

(** Exercise 3.52: [accum] adds each mapped element into a shared
    [sum], and the transcript of sums reveals the interleaving of
    laziness and assignment. The map class is [A]: Scheme's [set!] sum
    becomes one [int ref], and the "would the answers differ without
    memoization" question is answered by running the same transcript
    over plain thunks, not only argued. *)

open Sicp_ch3.Sec_3_5

(* The non-memoized stream of the exercise's last question: a plain
   thunk for the tail, re-evaluated on every access. *)
type 'a thunk_stream =
  | TCons of 'a * (unit -> 'a thunk_stream)
  | TEmpty

let rec thunk_map f = function
  | TCons (head, tail) -> TCons (f head, fun () -> thunk_map f (tail ()))
  | TEmpty -> TEmpty
;;

let rec thunk_filter pred = function
  | TCons (head, tail) when pred head ->
    TCons (head, fun () -> thunk_filter pred (tail ()))
  | TCons (_, tail) -> thunk_filter pred (tail ())
  | TEmpty -> TEmpty
;;

let rec thunk_ref s n =
  match s with
  | TCons (head, _) when n = 0 -> head
  | TCons (_, tail) -> thunk_ref (tail ()) (n - 1)
  | TEmpty -> invalid_arg "thunk_ref: the empty stream"
;;

let rec thunk_take n s =
  if n <= 0
  then []
  else (
    match s with
    | TCons (head, tail) -> head :: thunk_take (n - 1) (tail ())
    | TEmpty -> [])
;;

let ex_3_52 () =
  let sum = ref 0 in
  let accum x =
    sum := !sum + x;
    !sum
  in
  let seq = Streams.stream_map accum (Streams.stream_enumerate_interval 1 20) in
  let sum_after_seq = !sum in
  let y = Streams.stream_filter (fun x -> x mod 2 = 0) seq in
  let sum_after_y = !sum in
  let z = Streams.stream_filter (fun x -> x mod 5 = 0) seq in
  let sum_after_z = !sum in
  let y7 = Streams.stream_ref y 7 in
  let sum_after_ref = !sum in
  let z_displayed = Streams.stream_take 10 z in
  let sum_after_display = !sum in
  ignore y7;
  (* The same transcript over plain thunks. *)
  let sum = ref 0 in
  let accum x =
    sum := !sum + x;
    !sum
  in
  let rec thunk_enumerate low high =
    if low > high then TEmpty else TCons (low, fun () -> thunk_enumerate (low + 1) high)
  in
  let seq = thunk_map accum (thunk_enumerate 1 20) in
  let plain_sum_after_seq = !sum in
  let y = thunk_filter (fun x -> x mod 2 = 0) seq in
  let plain_sum_after_y = !sum in
  let z = thunk_filter (fun x -> x mod 5 = 0) seq in
  let plain_sum_after_z = !sum in
  let plain_y7 = thunk_ref y 7 in
  let plain_sum_after_ref = !sum in
  let plain_z_displayed = thunk_take 10 z in
  let plain_sum_after_display = !sum in
  ignore plain_y7;
  ( ( sum_after_seq
    , sum_after_y
    , sum_after_z
    , y7
    , sum_after_ref
    , z_displayed
    , sum_after_display )
  , ( plain_sum_after_seq
    , plain_sum_after_y
    , plain_sum_after_z
    , plain_y7
    , plain_sum_after_ref
    , plain_z_displayed
    , plain_sum_after_display ) )
;;
