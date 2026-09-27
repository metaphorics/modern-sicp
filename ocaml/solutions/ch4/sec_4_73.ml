(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.73: why [flatten-stream] delays the rest of the stream of
    streams.  The exercise's proposed undelayed version computes
    [flatten-stream] of the input's [stream-cdr] as an ordinary argument,
    so it must unroll the whole input stream of streams before the first
    output element exists.  On a finite input that is only wasted work
    (the pinned finite case agrees with the delayed engine element for
    element); on the infinite frame streams that recursive rules
    produce, the undelayed flatten diverges before answering anything,
    while the delayed engine serves answer after answer and demands the
    input only in proportion.

    The layer demonstration makes the demand observable: the input
    stream's second tail raises a local sentinel standing in for the
    divergent work (a hanging query cannot be pinned, so the raise is
    the trace of exactly the computation that would loop).  The delayed
    [flatten_stream] serves its first element without touching that
    tail; the undelayed flatten raises it before any element exists.
    The real-engine demonstration runs the delayed system on an [and]
    whose first conjunct is the infinite swap-rule stream of 4.72 and
    whose second is a simple query, so the flatmap's input stream of
    streams is genuinely infinite; the delayed engine answers, and the
    undelayed twin of this very query is the case that would hang. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The book's 3.5.3 [interleave]: no delay in the combination. *)
let rec interleave_undelayed s1 s2 =
  if Streams.stream_null s1
  then s2
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      interleave_undelayed (Streams.stream_cdr s1) s2)
;;

(* The exercise's proposed [flatten-stream]: the rest of the input is
   computed eagerly, before the head stream is even looked at. *)
let rec flatten_undelayed stream =
  if Streams.stream_null stream
  then Streams.the_empty_stream
  else (
    let rest = flatten_undelayed (Streams.stream_cdr stream) in
    interleave_undelayed (Streams.stream_car stream) rest)
;;

(* A stream of one-element streams. *)
let singletons items =
  List.fold_right
    (fun n acc -> Streams.cons_stream (Eval.singleton_stream n) (fun () -> acc))
    items
    Streams.the_empty_stream
;;

(* The stand-in for the work an undelayed flatten demands before its
   first answer: in the real system that demand is the rest of the frame
   stream, which recursive rules make infinite, so the query hangs
   before answering.  The sentinel raise marks exactly where the
   undelayed version pays that demand; the delayed version never does. *)
exception Divergent_rest

let sentinel_input =
  Streams.cons_stream (Eval.singleton_stream 1) (fun () -> raise Divergent_rest)
;;

let delayed_sentinel_first = [ Streams.stream_car (Eval.flatten_stream sentinel_input) ]

let undelayed_diverged =
  match flatten_undelayed sentinel_input with
  | _ -> false
  | exception Divergent_rest -> true
;;

let delayed_finite =
  Streams.stream_take 4 (Eval.flatten_stream (singletons [ 1; 2; 3; 4 ]))
;;

let undelayed_finite =
  Streams.stream_take 4 (flatten_undelayed (singletons [ 1; 2; 3; 4 ]))
;;

(* The delayed engine on a flatmap whose input frame stream is infinite:
   the first conjunct answers forever (the swap rule), so the second
   conjunct's [stream-flatmap] faces an infinite stream of streams. *)
let assertions =
  [ "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (loves (Minnie Mouse) (Mickey Mouse)))"
  ; "(assert! (rule (loves ?x ?y) (loves ?y ?x)))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | _ -> failwith "load: expected an assertion")
    assertions
;;

let and_query = "(and (loves ?a ?b) (job ?who (computer programmer)))"

let ex_4_73 () =
  let env = Eval.the_query_system () in
  load env;
  let delayed_and_first4 =
    match Eval.query_upto 4 env and_query with
    | Ok answers -> List.map Value.to_string answers
    | Error e -> [ "Error: " ^ Eval_error.to_string e ]
  in
  let ints items = String.concat " " (List.map string_of_int items) in
  [ "delayed_and_first4" ]
  @ delayed_and_first4
  @ [ "delayed_finite_first4=" ^ ints delayed_finite
    ; "undelayed_finite_first4=" ^ ints undelayed_finite
    ; "finite_orders_agree=" ^ string_of_bool (delayed_finite = undelayed_finite)
    ; "delayed_sentinel_first=" ^ ints delayed_sentinel_first
    ; "undelayed_diverged_before_first_answer=" ^ string_of_bool undelayed_diverged
    ]
;;
