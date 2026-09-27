(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.72: why [disjoin] interleaves the disjunct streams rather
    than appending them.  The variant evaluator routes [and]/[or] through
    its own handlers and leaves every other form -- errors included --
    to the substrate's [qeval]; its [or] merges the disjuncts with
    [stream_append_delayed], the same explicit delay the book's
    [disjoin] uses, so the demonstration isolates this exercise's
    append-versus-interleave choice from 4.71's delay question.

    The demonstration query ors an infinite disjunct with a finite one:
    the swap rule over one [loves] assertion answers forever, while
    exactly three people supervise Ben.  The book's interleaved [or]
    serves both disjuncts -- all three supervisor rows land inside the
    first six answers, drawn in proportion.  The appended variant
    starves the second disjunct: no supervisor row appears at all,
    whatever the bound, because the first stream never runs out.  That
    is 3.5.3's lesson about [interleave] versus [append] verbatim. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The three supervisor-of-Ben assertions and the infinite-answer
   family: one [loves] assertion plus the swap rule, whose query answers
   forever, alternating the two matchings. *)
let assertions =
  [ "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
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

let contents_of query =
  match Value.view query with
  | Value.Pair (_, cdr) -> Eval.value_list cdr
  | _ -> Error (Eval_error.Invalid_form "a compound query must be a list")
;;

(* The variant evaluator: append instead of interleave in [or]. *)
let rec disjoin_append env disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Eval.stream_append_delayed (qeval_append env first frames) (fun () ->
      disjoin_append env rest frames)

and conjoin_series env conjuncts frames =
  match conjuncts with
  | [] -> frames
  | first :: rest -> conjoin_series env rest (qeval_append env first frames)

and qeval_append env query frames =
  match Value.view query with
  | Value.Pair (tag, _) ->
    let name = Value.to_string tag in
    if String.equal name "or" || String.equal name "and"
    then (
      match contents_of query with
      | Ok operands ->
        if String.equal name "or"
        then disjoin_append env operands frames
        else conjoin_series env operands frames
      | Error _ -> Eval.qeval env query frames)
    else Eval.qeval env query frames
  | _ -> Eval.qeval env query frames
;;

(* The second disjunct of an or-answer: [(or d1 d2)] is
    [Pair(or, Pair(d1, Pair(d2, nil)))], so [d2] is the car of the cdr's
    cdr cell. *)
let or_second_disjunct answer =
  match Value.view answer with
  | Value.Pair (_, rest) ->
    (match Value.view rest with
     | Value.Pair (_, second_cell) ->
       (match Value.view second_cell with
        | Value.Pair (second, _) -> Some second
        | _ -> None)
     | _ -> None)
  | _ -> None
;;

(* The supervisee slot of a [supervisor] disjunct. *)
let supervisor_who disjunct =
  match Value.view disjunct with
  | Value.Pair (_, args) ->
    (match Value.view args with
     | Value.Pair (who, _) -> Some who
     | _ -> None)
  | _ -> None
;;

(* A supervisor answer instantiates the second disjunct's supervisee
   slot; the [loves] answers leave it the unbound [?who]. *)
let is_supervisor_answer answer =
  match or_second_disjunct answer with
  | Some disjunct ->
    (match supervisor_who disjunct with
     | Some who -> not (Value.structural_equal who (Value.symbol "?who"))
     | None -> false)
  | None -> false
;;

let or_query = "(or (loves ?a ?b) (supervisor ?who (Bitdiddle Ben)))"

(* One measurement: the first [n] answers rendered, plus how many of
   them come from the finite second disjunct. *)
let interleave_upto n env =
  match Eval.query_upto n env or_query with
  | Ok answers ->
    ( List.map Value.to_string answers
    , List.length (List.filter is_supervisor_answer answers) )
  | Error e -> [ "Error: " ^ Eval_error.to_string e ], 0
;;

let append_upto n env =
  match Eval.read_query or_query with
  | Error message -> [ "Error: " ^ message ], 0
  | Ok raw ->
    let q = Eval.query_syntax_process raw in
    let frames = qeval_append env q (Eval.singleton_stream Eval.the_empty_frame) in
    let answers =
      Streams.stream_take
        n
        (Streams.stream_map
           (fun frame ->
              Eval.instantiate q frame (fun v _ -> Eval.contract_question_mark v))
           frames)
    in
    ( List.map Value.to_string answers
    , List.length (List.filter is_supervisor_answer answers) )
;;

let ex_4_72 () =
  let env = Eval.the_query_system () in
  load env;
  let interleave8, interleave8_supervisors = interleave_upto 8 env in
  let append8, append8_supervisors = append_upto 8 env in
  let _append20, append20_supervisors = append_upto 20 env in
  [ "interleave_first8" ]
  @ interleave8
  @ [ "interleave_supervisor_answers_in_first8=" ^ string_of_int interleave8_supervisors
    ; "append_first8"
    ]
  @ append8
  @ [ "append_supervisor_answers_in_first8=" ^ string_of_int append8_supervisors
    ; "append_supervisor_answers_in_first20=" ^ string_of_int append20_supervisors
    ]
;;
