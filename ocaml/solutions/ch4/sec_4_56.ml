(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.56: the three compound queries -- Ben's supervisees with
   their addresses, the people paid less than Ben with both salaries,
   and the people whose supervisor works outside the computer division
   with the supervisor's name and job.  Each is an [And] of simple
   queries with the filters the book prescribes: a [Holds] comparison
   (the book's [lisp-value]) fed by Ben's salary as an inner query, and
   a [Not] on the supervisor's division.  The dotted pattern
   [[computer | ?type]] is what makes the [Not] cover three-element
   computer jobs such as Reasoner's traineeship. *)

open Sec_4_55.Kit

let ex_4_56 () =
  transcript
    (session microshaft)
    [ Q.And
        [ p [ at "supervisor"; v "person"; person "Bitdiddle Ben" ]
        ; p [ at "address"; v "person"; v "where" ]
        ]
    ; Q.And
        [ p [ at "salary"; v "person"; v "amount" ]
        ; p [ at "salary"; person "Bitdiddle Ben"; v "ben-amount" ]
        ; Q.Holds ("<", [ v "amount"; v "ben-amount" ])
        ]
    ; Q.And
        [ p [ at "supervisor"; v "person"; v "supervisor" ]
        ; p [ at "job"; v "supervisor"; v "job" ]
        ; Q.Not (p [ at "job"; v "supervisor"; Q.dotted [ at "computer" ] (v "type") ])
        ]
    ]
;;
