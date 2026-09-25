(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.64: Louis Reasoner reordered the [outranked-by] rule. The
    recursive clause reads [(and (outranked-by ?middle-manager ?boss)
    (supervisor ?staff-person ?middle-manager))] -- the recursion now runs
    first, so every rule application re-enters [outranked-by] with both
    variables still unbound. Each level's first disjunct re-enumerates the
    whole [supervisor] data base and spawns the next level before the
    trailing [supervisor] test can prune anything, and for the anchored
    query that test can never pass at all: it asks for [(supervisor
    (Bitdiddle Ben) ?middle-manager)], so it passes only frames whose
    middle-manager value is Warbucks Oliver, while frame staff slots are
    filled exclusively from the data base's staff-side names -- Warbucks
    appears only in boss slots. The filter therefore consumes the endless
    recursion forever without ever emitting a second answer.

    The demo pins the one answer that arrives before the loop --
    [(outranked-by (Bitdiddle Ben) (Warbucks Oliver))], delivered by the
    rule's first disjunct. This edition's [stream_take] forces one tail
    cell past the element it collects, and with Louis's conjunct order
    that tail is already the endless recursion, so the pre-loop prefix is
    captured from the driver's answer stream head ([run] plus
    [stream_car]) rather than [query_upto]. Non-termination is documented
    with a timeout-free bounded probe: [query_upto 2] of the recursion
    with both variables unbound returns at once -- its second answer
    already needs one full extra deduction level (Louis Reasoner via
    Alyssa P Hacker) -- while every further answer forces another level,
    each re-enumerating the data base; deeper takes never return in
    bounded time. The book's original conjunct order answers the same
    query completely, so the swap changed nothing about the answers --
    only about termination. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The [supervisor] assertions are the only facts [outranked-by] needs. *)
let supervisor_assertions =
  [ "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ]
;;

let louis_rule =
  "(assert! (rule (outranked-by ?staff-person ?boss)\n\
  \  (or (supervisor ?staff-person ?boss)\n\
  \      (and (outranked-by ?middle-manager\n\
  \                           ?boss)\n\
  \           (supervisor ?staff-person\n\
  \                       ?middle-manager)))))"
;;

let book_rule =
  "(assert! (rule (outranked-by ?staff-person ?boss)\n\
  \  (or (supervisor ?staff-person ?boss)\n\
  \      (and (supervisor ?staff-person\n\
  \                       ?middle-manager)\n\
  \           (outranked-by ?middle-manager\n\
  \                         ?boss)))))"
;;

let assert_all env texts = List.iter (fun text -> ignore (Eval.run env text)) texts

(* The answer stream's head: the one answer the driver prints before the
   system goes into the loop. *)
let first_answer env text =
  match Eval.run env text with
  | Ok (Eval.Answers s) -> Value.to_string (Eval.Streams.stream_car s)
  | Ok Asserted -> "asserted"
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let ex_4_64 () =
  let louis = Eval.the_query_system () in
  assert_all louis supervisor_assertions;
  assert_all louis [ louis_rule ];
  let before_loop = first_answer louis "(outranked-by (Bitdiddle Ben) ?who)" in
  (* Bounded probe of the recursion running on its own: the second answer
     is already the recursion one level deep. *)
  let probe =
    match Eval.query_upto 2 louis "(outranked-by ?staff ?boss)" with
    | Ok answers -> List.map Value.to_string answers
    | Error _ -> []
  in
  (* The book's original rule answers the same query to completion. *)
  let book = Eval.the_query_system () in
  assert_all book supervisor_assertions;
  assert_all book [ book_rule ];
  let complete =
    match Eval.query book "(outranked-by (Bitdiddle Ben) ?who)" with
    | Ok answers -> List.map Value.to_string answers
    | Error _ -> []
  in
  [ before_loop
  ; "forcing past this one answer diverges: the recursion runs before the "
    ^ "supervisor test, re-enumerates every level forever, and the test can "
    ^ "never pass (Warbucks appears only in boss slots)"
  ; Printf.sprintf
      "bounded probe take(2) of the unanchored recursion: %d answers, %s then %s -- the \
       second already one deduction level deep; deeper takes never return in bounded \
       time"
      (List.length probe)
      (match probe with
       | [ first; _ ] -> first
       | _ -> "?")
      (match probe with
       | [ _; second ] -> second
       | _ -> "?")
  ]
  @ ("the book's conjunct order answers the same query completely: " :: complete)
;;
