(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.65: Cy D. Fect's [wheel] query. The rule says [?person] is
    a wheel when someone supervises a middle-manager who in turn
    supervises someone. Under this edition's chronological data base the
    scan of the first conjunct's assertions delivers [Ben]'s routes
    first, so the engine answers Ben once and then Warbucks four times --
    the book's listing shows the same multiset in a different row order
    (Warbucks first). The four-fold Warbucks duplication is the point:
    the query counts derivation routes, not people. Warbucks is reached
    through four distinct (middle-manager, person) pairs -- Ben's three
    supervisees (Alyssa P Hacker, Cy D Fect, Lem E Tweakit) and Scrooge
    Eben's one (Robert Cratchet); Aull DeWitt, Warbucks's third
    middle-manager, supervises nobody. Ben is reached once, through
    Alyssa's supervisee Louis Reasoner. The demo pins the five rows and
    the route counts it computes from the data base. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show_result = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

(* The [wheel] rule's two conjuncts scan [supervisor] assertions only. *)
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

let wheel_rule =
  "(assert! (rule (wheel ?person)\n\
  \  (and (supervisor ?middle-manager ?person)\n\
  \       (supervisor ?x ?middle-manager))))"
;;

let assert_all env texts = List.iter (fun text -> ignore (Eval.run env text)) texts

(* [count env text] is the number of answers of a finite query. *)
let count env text =
  match Eval.query env text with
  | Ok answers -> List.length answers
  | Error _ -> 0
;;

let ex_4_65 () =
  let env = Eval.the_query_system () in
  assert_all env supervisor_assertions;
  assert_all env [ wheel_rule ];
  let wheels = Eval.query env "(wheel ?who)" in
  (* Routes to Warbucks: his middle-managers are Ben, Scrooge, and Aull;
     their supervisee counts are 3, 1, and 0. *)
  let ben_supervisees = count env "(supervisor ?x (Bitdiddle Ben))" in
  let scrooge_supervisees = count env "(supervisor ?x (Scrooge Eben))" in
  let aull_supervisees = count env "(supervisor ?x (Aull DeWitt))" in
  let warbucks_routes = ben_supervisees + scrooge_supervisees + aull_supervisees in
  (* Routes to Ben: his middle-managers are Alyssa, Cy, and Lem; only
     Alyssa supervises anyone (Louis). *)
  let alyssa_supervisees = count env "(supervisor ?x (Hacker Alyssa P))" in
  let cy_supervisees = count env "(supervisor ?x (Fect Cy D))" in
  let lem_supervisees = count env "(supervisor ?x (Tweakit Lem E))" in
  let ben_routes = alyssa_supervisees + cy_supervisees + lem_supervisees in
  show_result wheels
  @ [ Printf.sprintf
        "Warbucks appears %d times: %d (middle-manager Ben) + %d (middle-manager \
         Scrooge) + %d (middle-manager Aull) routes"
        warbucks_routes
        ben_supervisees
        scrooge_supervisees
        aull_supervisees
    ; Printf.sprintf
        "Ben appears %d time: middle-managers Alyssa, Cy, Lem with %d, 0, 0 supervisees"
        ben_routes
        alyssa_supervisees
    ]
;;
