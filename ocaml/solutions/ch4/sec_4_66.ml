(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.66: Ben's accumulation scheme. The designated variable is
    read out of every frame of the query pattern's [qeval] frame stream
    and handed to the accumulation function -- here a [sum] over
    [Value.Int] bindings. The scheme works on the book's programmer
    example (75000) and breaks exactly as Cy's [wheel] result predicts:
    the wheel query emits one frame per derivation route, so summing
    salaries over [(and (wheel ?who) (salary ?who ?amount))] counts
    Warbucks's 150000 four times (660000 instead of 210000). The salvage
    is to accumulate over distinct answers only: each frame is
    instantiated to its full answer, duplicate instantiations are
    dropped before the variable is read, and the sum collapses to the
    true total. *)

module Eval = Sicp_ch4.Sec_4_4
module Value = Sicp_common.Value

(* The facts the two demo queries scan. *)
let microshaft_core =
  [ "(assert! (address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))"
  ; "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (address (Fect Cy D) (Cambridge (Ames Street) 3)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (address (Tweakit Lem E) (Boston (Bay State Road) 22)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (address (Warbucks Oliver) (Swellesley (Top Heap Road))))"
  ; "(assert! (job (Warbucks Oliver) (administration big wheel)))"
  ; "(assert! (salary (Warbucks Oliver) 150000))"
  ; "(assert! (address (Scrooge Eben) (Weston (Shady Lane) 10)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (address (Cratchet Robert) (Allston (N Harvard Street) 16)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (address (Aull DeWitt) (Slumerville (Onion Square) 5)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ]
;;

let wheel_rule =
  "(assert! (rule (wheel ?person)\n\
  \  (and (supervisor ?middle-manager ?person)\n\
  \       (supervisor ?x ?middle-manager))))"
;;

let assert_all env texts = List.iter (fun text -> ignore (Eval.run env text)) texts

let internal_var text =
  match Eval.read_query text with
  | Ok raw -> Eval.query_syntax_process raw
  | Error _ -> Value.symbol "?"
;;

(* [answer_key query frame] is the frame's full answer, the instantiation
    the driver would print; duplicate keys are duplicate answers. *)
let answer_key query frame =
  Value.to_string
    (Eval.instantiate query frame (fun v _ -> Eval.contract_question_mark v))
;;

(* [frames env text] is the whole frame stream of a finite query, as a
    list, in the stream's order. *)
let frames env text =
  match Eval.read_query text with
  | Error _ -> []
  | Ok raw ->
    let q = Eval.query_syntax_process raw in
    let s = Eval.qeval env q (Eval.singleton_stream Eval.the_empty_frame) in
    let rec walk s acc =
      if Eval.Streams.stream_null s
      then List.rev acc
      else walk (Eval.Streams.stream_cdr s) (Eval.Streams.stream_car s :: acc)
    in
    walk s []
;;

(* [sum_over env query_text var_text] is Ben's scheme: the sum of the
    designated variable over every frame the query pattern produces. *)
let sum_over env query_text var_text =
  let v = internal_var var_text in
  let fs = frames env query_text in
  let total =
    List.fold_left
      (fun acc f ->
         match Eval.binding_in_frame v f with
         | Some (_, value) ->
           (match Value.view value with
            | Value.Int n -> acc + n
            | _ -> acc)
         | None -> acc)
      0
      fs
  in
  List.length fs, total
;;

(* [sum_distinct env query_text var_text] is the salvage: drop every
    frame whose full answer was already seen, then run the same
    accumulation over the distinct frames. *)
let sum_distinct env query_text var_text =
  let q =
    match Eval.read_query query_text with
    | Ok raw -> Eval.query_syntax_process raw
    | Error _ -> Value.nil
  in
  let v = internal_var var_text in
  let fs = frames env query_text in
  let rec keep seen = function
    | [] -> []
    | f :: rest ->
      let key = answer_key q f in
      if List.exists (String.equal key) seen
      then keep seen rest
      else f :: keep (key :: seen) rest
  in
  let distinct = keep [] fs in
  let total =
    List.fold_left
      (fun acc f ->
         match Eval.binding_in_frame v f with
         | Some (_, value) ->
           (match Value.view value with
            | Value.Int n -> acc + n
            | _ -> acc)
         | None -> acc)
      0
      distinct
  in
  List.length distinct, total
;;

let ex_4_66 () =
  let env = Eval.the_query_system () in
  assert_all env microshaft_core;
  assert_all env [ wheel_rule ];
  let n1, book_sum =
    sum_over env "(and (job ?x (computer programmer)) (salary ?x ?amount))" "?amount"
  in
  let n2, raw_sum = sum_over env "(and (wheel ?who) (salary ?who ?amount))" "?amount" in
  let n3, distinct_sum =
    sum_distinct env "(and (wheel ?who) (salary ?who ?amount))" "?amount"
  in
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
