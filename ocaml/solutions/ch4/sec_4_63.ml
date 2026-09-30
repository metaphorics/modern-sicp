(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.63: the Genesis 4 genealogy data base and the two rules
   the statement phrases: a grandson is a son of a son, and the
   son-of-wife rule is what extends [son] to the mother, so Lamech's
   sons and Methushael's grandsons come out through it.  The [grandson]
   relation reads [[grandson, ?grandson, ?grandfather]], and its body
   keeps the statement's order -- "S is the son of F" before "F is the
   son of G".  All three statement queries are finite. *)

open Sec_4_55.Kit

let son father child = l [ at "son"; at father; at child ]

let genesis =
  [ son "Adam" "Cain"
  ; son "Cain" "Enoch"
  ; son "Enoch" "Irad"
  ; son "Irad" "Mehujael"
  ; son "Mehujael" "Methushael"
  ; son "Methushael" "Lamech"
  ; atoms [ "wife"; "Lamech"; "Ada" ]
  ; son "Ada" "Jabal"
  ; son "Ada" "Jubal"
  ]
;;

let rules =
  [ ( l [ at "grandson"; v "grandson"; v "grandfather" ]
    , Q.And
        [ p [ at "son"; v "father"; v "grandson" ]
        ; p [ at "son"; v "grandfather"; v "father" ]
        ] )
  ; ( l [ at "son"; v "m"; v "s" ]
    , Q.And [ p [ at "wife"; v "m"; v "w" ]; p [ at "son"; v "w"; v "s" ] ] )
  ]
;;

let ex_4_63 () =
  transcript
    (session ~rules genesis)
    [ p [ at "grandson"; v "x"; at "Cain" ]
    ; p [ at "son"; at "Lamech"; v "x" ]
    ; p [ at "grandson"; v "x"; at "Methushael" ]
    ]
;;
