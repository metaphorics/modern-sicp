(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.62: rules for [last-pair], the operation of exercise 2.17.
   The base case reads "the last pair of a one-element list is that
   list"; the step strips the first element and recurses.  The four
   statement queries behave differently, and the demonstration reports
   each as the engine actually answers it.

   [[last-pair, [3], ?x]] and [[last-pair, [1, 2, 3], ?x]] answer once
   and run dry: the step rule's body walks down to [[]] where no rule
   matches.  [[last-pair, [2, ?x], [3]]] also terminates with exactly
   the intended answer: re-applying the step rule to the bound
   one-element tail would need [[]] to unify with that list, so the
   recursive branch dies after the one answer.

   [[last-pair, ?x, [3]]] does not run dry: [?x] is free, so every
   application of the step rule rebinds its fresh tail variable and
   yields one more answer one level deeper, forever.  The demonstration
   reads exactly three answers, each with its renamed rule variables
   numbered by first occurrence ([?v.1], then [[?v.1, ?v.2]]). *)

open Sec_4_55.Kit

let last_pair list last = l [ at "last-pair"; list; last ]

let rules =
  [ last_pair (l [ v "x" ]) (l [ v "x" ]), Q.Always_true
  ; last_pair (Q.dotted [ v "v" ] (v "w")) (v "y"), Q.Pattern (last_pair (v "w") (v "y"))
  ]
;;

let ex_4_62 () =
  let s = session ~rules [] in
  let reverse = Q.Pattern (last_pair (v "x") (l [ n 3 ])) in
  transcript
    s
    [ Q.Pattern (last_pair (l [ n 3 ]) (v "x"))
    ; Q.Pattern (last_pair (l [ n 1; n 2; n 3 ]) (v "x"))
    ; Q.Pattern (last_pair (l [ n 2; v "x" ]) (l [ n 3 ]))
    ]
  @ [ "note: the third query terminates; the step rule's recursive branch dies against \
       the query's already-bound one-element tail"
    ; "? " ^ Q.render_query reverse
    ; "note: the full query never terminates; each step-rule application rebinds its \
       fresh tail variable and yields one more answer; first three answers:"
    ]
  @ answers_upto 3 s reverse
;;
