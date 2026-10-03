(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.59: Ben's meeting assertions, Alyssa's [meeting-time]
   rule -- a person's meetings are the whole-company meetings plus the
   meetings of the person's own division -- and the two queries: the
   Friday-morning scan, and Alyssa's Wednesday schedule by name.  The
   rule's [Or] interleaves its disjuncts, so Alyssa's Wednesday answer
   lists the whole-company 4pm meeting before her division's 3pm one:
   the first disjunct's answer lands first, the algorithm the book
   presents rather than the append order of the prose.  The division
   comes out of the job pattern [[?division | ?rest]], whose dotted
   tail spans three-element jobs such as Reasoner's traineeship. *)

open Sec_4_55.Kit

let meeting division day time = l [ at "meeting"; at division; atoms [ day; time ] ]

let meetings =
  [ meeting "accounting" "Monday" "9am"
  ; meeting "administration" "Monday" "10am"
  ; meeting "computer" "Wednesday" "3pm"
  ; meeting "administration" "Friday" "1pm"
  ; meeting "whole-company" "Wednesday" "4pm"
  ]
;;

let rules =
  [ ( l [ at "meeting-time"; v "person"; v "day-and-time" ]
    , Q.Or
        [ p [ at "meeting"; at "whole-company"; v "day-and-time" ]
        ; Q.And
            [ p [ at "job"; v "person"; Q.dotted [ v "division" ] (v "rest") ]
            ; p [ at "meeting"; v "division"; v "day-and-time" ]
            ]
        ] )
  ]
;;

let ex_4_59 () =
  transcript
    (session ~rules (microshaft @ meetings))
    [ p [ at "meeting"; v "division"; l [ at "Friday"; v "time" ] ]
    ; p [ at "meeting-time"; person "Hacker Alyssa P"; l [ at "Wednesday"; v "time" ] ]
    ]
;;
