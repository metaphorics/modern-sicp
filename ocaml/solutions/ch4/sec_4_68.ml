(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.68: rules for [reverse] over the [append-to-form] rules of
   4.4.1.  The base rule covers the empty list; the recursive rule says
   the reverse of [[?u | ?v]] is the append of the reverse of [?v] with
   the one-element list [[?u]].  A forward query grounds the recursion,
   so it answers finitely.  The backward query [[reverse, ?x, [1, 2, 3]]]
   leaves both ends of the recursive subquery [[reverse, ?v, ?w]]
   unbound: the engine wades through the infinite stream of candidate
   pairs, the [append-to-form] conjunct filters them against the ground
   target, and the one true answer arrives first -- after which forcing
   further answers never returns.  The backward direction is therefore
   read one answer deep, never forcing the stream behind it. *)

open Sec_4_55.Kit

let append_to_form x y z = l [ at "append-to-form"; x; y; z ]
let reverse x y = l [ at "reverse"; x; y ]

let rules =
  [ append_to_form Q.Nil (v "y") (v "y"), Q.Always_true
  ; ( append_to_form (Q.dotted [ v "u" ] (v "v")) (v "y") (Q.dotted [ v "u" ] (v "z"))
    , Q.Pattern (append_to_form (v "v") (v "y") (v "z")) )
  ; reverse Q.Nil Q.Nil, Q.Always_true
  ; ( reverse (Q.dotted [ v "u" ] (v "v")) (v "z")
    , Q.And
        [ Q.Pattern (reverse (v "v") (v "w"))
        ; Q.Pattern (append_to_form (v "w") (l [ v "u" ]) (v "z"))
        ] )
  ]
;;

let ex_4_68 () =
  let s = session ~rules [] in
  let show label answers = label ^ " => " ^ String.concat "; " answers in
  let forward3 = Q.Pattern (reverse (l [ n 1; n 2; n 3 ]) (v "x")) in
  let forward4 = Q.Pattern (reverse (atoms [ "a"; "b"; "c"; "d" ]) (v "x")) in
  let backward = Q.Pattern (reverse (v "x") (l [ n 1; n 2; n 3 ])) in
  [ show (Q.render_query forward3) (answers_all s forward3)
  ; show (Q.render_query forward4) (answers_all s forward4)
  ; show (Q.render_query backward ^ " first answer") (answers_upto 1 s backward)
  ; "backward: forcing a second answer does not return; with both ends of [reverse, ?v, \
     ?w] unbound the engine generates infinitely many candidate pairs and only [reverse, \
     [3, 2, 1], [1, 2, 3]] survives the append-to-form filter"
  ]
;;
