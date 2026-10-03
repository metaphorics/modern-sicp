(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.60: Alyssa's [lives-near] pair query.  The book's rule
   matches each pair of neighbors in both orders -- the two [address]
   conjuncts are independent, so binding [?person-1] to Alyssa and
   [?person-2] to Cy is one solution and the swapped binding is another
   -- and the chronological data base lists both, eight answers for four
   pairs.  The demonstration runs the ride-share query, the full pair
   query, and a deduplicated variant [lives-near-unique] that keeps one
   order per pair.

   The classic dedup orders each pair by name, but a [Holds] comparison
   reads numbers and strings, and a name here is a list of atoms.  The
   edition therefore orders each pair by salary through
   [Holds ("<", [?salary-1; ?salary-2])]; the strict [<] also excludes
   the self-pair the book's [Not (same ...)] clause removed.  Two
   same-town neighbors with equal salaries would still need a name
   comparison. *)

open Sec_4_55.Kit

let addresses_and_salaries =
  List.filter
    (function
      | Q.Pair (Q.Atom ("address" | "salary"), _) -> true
      | _ -> false)
    microshaft
;;

let town_of who rest = p [ at "address"; v who; Q.dotted [ v "town" ] (v rest) ]

let rules =
  [ ( l [ at "lives-near"; v "person-1"; v "person-2" ]
    , Q.And
        [ town_of "person-1" "rest-1"
        ; town_of "person-2" "rest-2"
        ; Q.Not (p [ at "same"; v "person-1"; v "person-2" ])
        ] )
  ; l [ at "same"; v "x"; v "x" ], Q.Always_true
  ; ( l [ at "lives-near-unique"; v "person-1"; v "person-2" ]
    , Q.And
        [ town_of "person-1" "rest-1"
        ; town_of "person-2" "rest-2"
        ; p [ at "salary"; v "person-1"; v "salary-1" ]
        ; p [ at "salary"; v "person-2"; v "salary-2" ]
        ; Q.Holds ("<", [ v "salary-1"; v "salary-2" ])
        ] )
  ]
;;

let ex_4_60 () =
  let s = session ~rules addresses_and_salaries in
  let ride =
    transcript s [ p [ at "lives-near"; v "person"; person "Hacker Alyssa P" ] ]
  in
  let pairs = transcript s [ p [ at "lives-near"; v "person-1"; v "person-2" ] ] in
  let unique =
    transcript s [ p [ at "lives-near-unique"; v "person-1"; v "person-2" ] ]
  in
  ride
  @ [ "note: every pair appears twice, once per binding order of the two address \
       conjuncts"
    ]
  @ pairs
  @ [ "note: one order per pair, chosen by holds(<, ?salary-1, ?salary-2)" ]
  @ unique
;;
