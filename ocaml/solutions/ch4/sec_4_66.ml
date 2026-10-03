(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.66: Ben's accumulation scheme.  The designated variable is
   read out of every frame of the query's [qeval] frame stream and
   handed to the accumulation function -- here a sum over its [Num]
   bindings.  The scheme works on the book's programmer example (75000)
   and breaks exactly as Cy's [wheel] result predicts: the wheel query
   emits one frame per derivation route, so summing salaries over
   [And [wheel ?who; salary ?who ?amount]] counts Warbucks's 150000 four
   times (660000 instead of 210000).  The salvage is to accumulate over
   distinct answers only: each frame is instantiated to its full answer,
   duplicate instantiations are dropped before the variable is read,
   and the sum collapses to the true total. *)

open Sec_4_55.Kit

let wheel_rule =
  ( l [ at "wheel"; v "person" ]
  , Q.And
      [ p [ at "supervisor"; v "middle-manager"; v "person" ]
      ; p [ at "supervisor"; v "x"; v "middle-manager" ]
      ] )
;;

let frames s q =
  let collected = Dynarray.create () in
  Streams.stream_for_each
    (Dynarray.add_last collected)
    (Q.qeval s q (Q.singleton_stream []));
  Dynarray.to_list collected
;;

let amount_in variable frame =
  match Q.instantiate (v variable) frame (fun unbound -> Q.Var unbound) with
  | Q.Num k -> k
  | _ -> 0
;;

let sum variable fs = List.fold_left (fun acc f -> acc + amount_in variable f) 0 fs

(* Ben's scheme: the sum of the designated variable over every frame. *)
let sum_over s q variable =
  let fs = frames s q in
  sum variable fs, List.length fs
;;

(* The salvage: one frame per distinct full answer. *)
let sum_distinct s q variable =
  let seen = Hashtbl.create 8 in
  let distinct =
    List.filter
      (fun frame ->
         let key = Q.render_query (Q.instantiate_query q frame) in
         if Hashtbl.mem seen key
         then false
         else (
           Hashtbl.add seen key ();
           true))
      (frames s q)
  in
  sum variable distinct, List.length distinct
;;

let ex_4_66 () =
  let s = session ~rules:[ wheel_rule ] microshaft in
  let book_sum, n1 =
    sum_over
      s
      (Q.And
         [ p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ]
         ; p [ at "salary"; v "x"; v "amount" ]
         ])
      "amount"
  in
  let wheels =
    Q.And [ p [ at "wheel"; v "who" ]; p [ at "salary"; v "who"; v "amount" ] ]
  in
  let raw_sum, n2 = sum_over s wheels "amount" in
  let distinct_sum, n3 = sum_distinct s wheels "amount" in
  [ Printf.sprintf "sum over the book's query = %d (from %d frames)" book_sum n1
  ; Printf.sprintf
      "Ben's scheme on the wheel query = %d (from %d frames): Warbucks's 150000 counted \
       %d times, %d duplicate frames"
      raw_sum
      n2
      (n2 - n3 + 1)
      (n2 - n3)
  ; Printf.sprintf
      "salvage, distinct answers only = %d (from %d frames): the true payroll of the \
       wheels"
      distinct_sum
      n3
  ]
;;
