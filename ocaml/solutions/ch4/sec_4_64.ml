(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.64: Louis Reasoner reordered the [outranked-by] rule.
   The recursive clause reads
   [And [outranked-by ?middle-manager ?boss; supervisor ?staff-person
   ?middle-manager]] -- the recursion now runs first, so every rule
   application re-enters [outranked-by] with both variables still
   unbound.  Each level's first disjunct re-enumerates the whole
   [supervisor] data base and spawns the next level before the trailing
   [supervisor] test can prune anything, and for the anchored query that
   test can never pass at all: it asks for
   [[supervisor, [Bitdiddle, Ben], ?middle-manager]], so it passes only
   frames whose middle-manager value is Warbucks Oliver, while frame
   staff slots are filled exclusively from the data base's staff-side
   names -- Warbucks appears only in boss slots.  The filter therefore
   consumes the endless recursion forever without emitting a second
   answer.

   The demo reads exactly the one answer that arrives before the loop
   -- [[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]], delivered
   by the rule's first disjunct -- and never forces the stream behind
   it.  A bounded probe reads two answers of the recursion with both
   variables unbound: the second already needs one full extra deduction
   level, and every further answer forces another level, each
   re-enumerating the data base.  The book's original conjunct order
   answers the same query completely, so the swap changed nothing about
   the answers -- only about termination. *)

open Sec_4_55.Kit

let supervisors =
  List.filter
    (function
      | Q.Pair (Q.Atom "supervisor", _) -> true
      | _ -> false)
    microshaft
;;

let outranked_by staff boss = l [ at "outranked-by"; staff; boss ]

let louis_rule =
  ( outranked_by (v "staff-person") (v "boss")
  , Q.Or
      [ p [ at "supervisor"; v "staff-person"; v "boss" ]
      ; Q.And
          [ Q.Pattern (outranked_by (v "middle-manager") (v "boss"))
          ; p [ at "supervisor"; v "staff-person"; v "middle-manager" ]
          ]
      ] )
;;

let book_rule =
  ( outranked_by (v "staff-person") (v "boss")
  , Q.Or
      [ p [ at "supervisor"; v "staff-person"; v "boss" ]
      ; Q.And
          [ p [ at "supervisor"; v "staff-person"; v "middle-manager" ]
          ; Q.Pattern (outranked_by (v "middle-manager") (v "boss"))
          ]
      ] )
;;

let ex_4_64 () =
  let anchored = Q.Pattern (outranked_by (person "Bitdiddle Ben") (v "who")) in
  let louis = session ~rules:[ louis_rule ] supervisors in
  let before_loop = answers_upto 1 louis anchored in
  let probe = answers_upto 2 louis (Q.Pattern (outranked_by (v "staff") (v "boss"))) in
  let complete = answers_all (session ~rules:[ book_rule ] supervisors) anchored in
  before_loop
  @ [ "forcing past this one answer diverges: the recursion runs before the supervisor \
       test, re-enumerates every level forever, and the test can never pass (Warbucks \
       appears only in boss slots)"
    ; Printf.sprintf
        "bounded probe take(2) of the unanchored recursion: %d answers, %s -- the second \
         already one deduction level deep; deeper takes never return in bounded time"
        (List.length probe)
        (String.concat " then " probe)
    ; "the book's conjunct order answers the same query completely:"
    ]
  @ complete
;;
