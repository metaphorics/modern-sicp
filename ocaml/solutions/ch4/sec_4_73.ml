(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.73: why [flatten_stream] delays the rest of the stream of
   streams.  The exercise's proposed undelayed version computes
   [flatten_stream] of the input's tail as an ordinary argument, so it
   must unroll the whole input stream of streams before the first output
   element exists.  On a finite input that is only wasted work (the
   pinned finite case agrees with the delayed engine element for
   element); on the infinite frame streams that recursive rules produce,
   the undelayed flatten diverges before answering anything, while the
   delayed engine serves answer after answer and demands the input only
   in proportion.  The ladder demonstration makes the demand observable:
   the input stream's second tail raises a local sentinel standing in
   for the divergent work (a hanging query cannot be pinned, so the
   raise is the trace of exactly the computation that would loop).  The
   delayed flatten serves its first element without touching that tail;
   the undelayed flatten raises before any element exists.  The
   real-engine demonstration runs the delayed system on an [And] whose
   first conjunct is the infinite swap-rule stream of 4.72 and whose
   second is a simple query, so the flatmap's input stream of streams is
   genuinely infinite: the delayed engine answers, and the undelayed twin
   of this very query is the case that would hang. *)

open Sec_4_55.Kit

(* The book's flatten, reached through the engine's exported flatmap. *)
let flatten_stream streams = Q.stream_flatmap Fun.id streams

(* The book's 3.5.3 [interleave]: no delay in the combination. *)
let rec interleave_undelayed s1 s2 =
  match s1 with
  | Streams.Empty -> s2
  | Streams.Cons (head, tail) ->
    Streams.cons_stream head (fun () -> interleave_undelayed s2 (Lazy.force tail))
;;

(* The exercise's proposed [flatten-stream]: the rest of the input is
   computed eagerly, before the head stream is even looked at. *)
let rec flatten_undelayed = function
  | Streams.Empty -> Streams.the_empty_stream
  | Streams.Cons (head, tail) ->
    let rest = flatten_undelayed (Lazy.force tail) in
    interleave_undelayed head rest
;;

let singletons items =
  List.fold_right
    (fun k acc -> Streams.cons_stream (Q.singleton_stream k) (fun () -> acc))
    items
    Streams.the_empty_stream
;;

(* The stand-in for the work an undelayed flatten demands before its
   first answer. *)
exception Divergent_rest

let sentinel_input () =
  Streams.cons_stream (Q.singleton_stream 1) (fun () ->
    Streams.cons_stream (Q.singleton_stream 2) (fun () -> raise Divergent_rest))
;;

let assertions =
  [ l [ at "job"; person "Hacker Alyssa P"; atoms [ "computer"; "programmer" ] ]
  ; l [ at "job"; person "Fect Cy D"; atoms [ "computer"; "programmer" ] ]
  ; l [ at "loves"; person "Minnie Mouse"; person "Mickey Mouse" ]
  ]
;;

let rules = [ l [ at "loves"; v "x"; v "y" ], p [ at "loves"; v "y"; v "x" ] ]

let ex_4_73 () =
  let s = session ~rules assertions in
  let and_query =
    Q.And
      [ p [ at "loves"; v "a"; v "b" ]
      ; p [ at "job"; v "who"; atoms [ "computer"; "programmer" ] ]
      ]
  in
  let ints items = String.concat " " (List.map string_of_int items) in
  let delayed_sentinel_first = take 1 (flatten_stream (sentinel_input ())) in
  let undelayed_diverged =
    match flatten_undelayed (sentinel_input ()) with
    | _ -> false
    | exception Divergent_rest -> true
  in
  let delayed_finite = take 4 (flatten_stream (singletons [ 1; 2; 3; 4 ])) in
  let undelayed_finite = take 4 (flatten_undelayed (singletons [ 1; 2; 3; 4 ])) in
  [ "delayed_and_first4" ]
  @ answers_upto 4 s and_query
  @ [ "delayed_finite_first4=" ^ ints delayed_finite
    ; "undelayed_finite_first4=" ^ ints undelayed_finite
    ; "finite_orders_agree=" ^ string_of_bool (delayed_finite = undelayed_finite)
    ; "delayed_sentinel_first=" ^ ints delayed_sentinel_first
    ; "undelayed_diverged_before_first_answer=" ^ string_of_bool undelayed_diverged
    ]
;;
