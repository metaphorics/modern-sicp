(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.69: "greats" relationships over the Genesis data base of
   4.63.  A relationship is a list ending in the word [grandson]:
   [ends-in-grandson] recognizes exactly those lists, the bridge rule
   ties the one-element relationship [[grandson]] to the two-slot
   [grandson] relation, and the greats rule derives
   [[[great | ?rel], ?x, ?y]] by peeling one [great] off the
   relationship and one generation off the [son] chain.  Without the
   [ends-in-grandson] guard the recursion could bind the relationship to
   a dotted tail, so the guard is the rule's first conjunct.  The
   relationship sits in a pattern's head position, which the typed
   terms admit because a pattern is any term.  Queries with a ground
   relationship answer finitely; [[?relationship, Adam, Irad]] leaves
   [?rel] unbound, the guard then generates relationship lists of every
   depth, and the query is read one answer deep: the true answer is
   first, and the generation never runs out. *)

open Sec_4_55.Kit

let genesis = Sec_4_63.genesis

let rules =
  [ ( l [ at "son"; v "m"; v "s" ]
    , Q.And [ p [ at "wife"; v "m"; v "w" ]; p [ at "son"; v "w"; v "s" ] ] )
  ; ( l [ at "grandson"; v "g"; v "s" ]
    , Q.And [ p [ at "son"; v "g"; v "f" ]; p [ at "son"; v "f"; v "s" ] ] )
  ; l [ at "ends-in-grandson"; atoms [ "grandson" ] ], Q.Always_true
  ; ( l [ at "ends-in-grandson"; Q.dotted [ at "great" ] (v "rest") ]
    , p [ at "ends-in-grandson"; v "rest" ] )
  ; l [ atoms [ "grandson" ]; v "x"; v "y" ], p [ at "grandson"; v "x"; v "y" ]
  ; ( l [ Q.dotted [ at "great" ] (v "rel"); v "x"; v "y" ]
    , Q.And
        [ p [ at "ends-in-grandson"; v "rel" ]
        ; p [ v "rel"; v "x"; v "z" ]
        ; p [ at "son"; v "z"; v "y" ]
        ] )
  ]
;;

let ex_4_69 () =
  let s = session ~rules genesis in
  let labelled q answers =
    List.map (fun answer -> Q.render_query q ^ " => " ^ answer) answers
  in
  let greats = p [ atoms [ "great"; "grandson" ]; v "g"; v "ggs" ] in
  let irad = p [ v "relationship"; at "Adam"; at "Irad" ] in
  let fifth =
    p
      [ atoms [ "great"; "great"; "great"; "great"; "great"; "grandson" ]
      ; at "Adam"
      ; v "d"
      ]
  in
  labelled greats (answers_all s greats)
  @ List.map
      (fun answer -> Q.render_query irad ^ " first answer => " ^ answer)
      (answers_upto 1 s irad)
  @ [ "note: with ?relationship unbound the ends-in-grandson generator produces \
       relationship lists of every depth; the first answer is the true one and a full \
       forcing would not return"
    ]
  @ labelled fifth (answers_all s fifth)
;;
