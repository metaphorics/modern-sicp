(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.61: the book's [next-to] rules and the two queries of the
   statement.  The engine tries the candidate rules in insertion order,
   so the base case answers first; rule 2's body then re-enters the same
   rule pair at the list tail, and the recursive descent reports each
   deeper adjacency on the way out -- the pairs of the whole list before
   the pairs of its tail, innermost last.  The displayed answers
   instantiate the queried pattern, so every answer shows its adjacency
   in the original list. *)

open Sec_4_55.Kit

let next_to x y list = l [ x; at "next-to"; y; at "in"; list ]

let rules =
  [ next_to (v "x") (v "y") (Q.dotted [ v "x"; v "y" ] (v "u")), Q.Always_true
  ; ( next_to (v "x") (v "y") (Q.dotted [ v "v" ] (v "z"))
    , Q.Pattern (next_to (v "x") (v "y") (v "z")) )
  ]
;;

let ex_4_61 () =
  transcript
    (session ~rules [])
    [ Q.Pattern (next_to (v "x") (v "y") (l [ n 1; l [ n 2; n 3 ]; n 4 ]))
    ; Q.Pattern (next_to (v "x") (n 1) (l [ n 2; n 1; n 3; n 1 ]))
    ]
  @ [ "note: rules are tried in insertion order; rule 2 recurses on the list tail, so \
       the base case of the whole list answers first and the innermost adjacency last"
    ]
;;
