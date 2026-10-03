(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.65: Cy D. Fect's [wheel] query.  The rule says [?person]
   is a wheel when someone supervises a middle-manager who in turn
   supervises someone.  Under this edition's chronological data base the
   scan of the first conjunct's assertions delivers Ben's routes first,
   so the engine answers Ben once and then Warbucks four times -- the
   book's listing shows the same multiset in a different row order
   (Warbucks first).  The four-fold Warbucks duplication is the point:
   the query counts derivation routes, not people.  Warbucks is reached
   through four distinct (middle-manager, person) pairs -- Ben's three
   supervisees and Scrooge's one; Aull DeWitt, Warbucks's third
   middle-manager, supervises nobody.  Ben is reached once, through
   Alyssa's supervisee Louis Reasoner.  The demo pins the five rows and
   the route counts it computes from the data base. *)

open Sec_4_55.Kit

let supervisors =
  List.filter
    (function
      | Q.Pair (Q.Atom "supervisor", _) -> true
      | _ -> false)
    microshaft
;;

let wheel_rule =
  ( l [ at "wheel"; v "person" ]
  , Q.And
      [ p [ at "supervisor"; v "middle-manager"; v "person" ]
      ; p [ at "supervisor"; v "x"; v "middle-manager" ]
      ] )
;;

let ex_4_65 () =
  let s = session ~rules:[ wheel_rule ] supervisors in
  let supervisees name =
    List.length (answers_all s (p [ at "supervisor"; v "x"; person name ]))
  in
  let ben = supervisees "Bitdiddle Ben" in
  let scrooge = supervisees "Scrooge Eben" in
  let aull = supervisees "Aull DeWitt" in
  let alyssa = supervisees "Hacker Alyssa P" in
  let cy = supervisees "Fect Cy D" in
  let lem = supervisees "Tweakit Lem E" in
  let rows = transcript s [ p [ at "wheel"; v "who" ] ] in
  let warbucks_row =
    Q.render_query (Q.Pattern (l [ at "wheel"; person "Warbucks Oliver" ]))
  in
  let observed_warbucks =
    List.length (List.filter (fun line -> String.equal line warbucks_row) rows)
  in
  let ben_row = Q.render_query (Q.Pattern (l [ at "wheel"; person "Bitdiddle Ben" ])) in
  let observed_ben =
    List.length (List.filter (fun line -> String.equal line ben_row) rows)
  in
  rows
  @ [ Printf.sprintf
        "Warbucks appears %d times (observed): %d (middle-manager Ben) + %d \
         (middle-manager Scrooge) + %d (middle-manager Aull) routes"
        observed_warbucks
        ben
        scrooge
        aull
    ; Printf.sprintf
        "Ben appears %d time (observed): middle-managers Alyssa, Cy, Lem with %d, %d, %d \
         supervisees"
        observed_ben
        alyssa
        cy
        lem
    ]
;;
