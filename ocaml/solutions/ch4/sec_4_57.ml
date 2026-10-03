(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.57: the [can-replace] rule -- person 1 can replace person
   2 when their jobs coincide or person 1's job can do person 2's job,
   and they are not the same person -- plus the two queries it exists
   for: everyone who can replace Cy D. Fect, and every replacement pair
   where the replaced person is paid more than the replacement, with
   both salaries.  The rule needs the section's [same] rule for the
   identity exclusion.  A person's own job never propagates
   transitively: [can-do-job] is a direct edge, so Ben replaces Hacker,
   Fect and Tweakit but not Reasoner's traineeship.  With [?person-2]
   bound by the query, the [Or]'s same-job disjunct streams before the
   [can-do-job] disjunct under the evaluator's interleaving, which is
   why Hacker's replacement of Fect precedes Ben's. *)

open Sec_4_55.Kit

let rules =
  [ l [ at "same"; v "x"; v "x" ], Q.Always_true
  ; ( l [ at "can-replace"; v "person-1"; v "person-2" ]
    , Q.And
        [ p [ at "job"; v "person-1"; v "job-1" ]
        ; Q.Or
            [ p [ at "job"; v "person-2"; v "job-1" ]
            ; Q.And
                [ p [ at "can-do-job"; v "job-1"; v "job-2" ]
                ; p [ at "job"; v "person-2"; v "job-2" ]
                ]
            ]
        ; Q.Not (p [ at "same"; v "person-1"; v "person-2" ])
        ] )
  ]
;;

let ex_4_57 () =
  transcript
    (session ~rules microshaft)
    [ p [ at "can-replace"; v "x"; person "Fect Cy D" ]
    ; Q.And
        [ p [ at "can-replace"; v "person-1"; v "person-2" ]
        ; p [ at "salary"; v "person-1"; v "salary-1" ]
        ; p [ at "salary"; v "person-2"; v "salary-2" ]
        ; Q.Holds ("<", [ v "salary-1"; v "salary-2" ])
        ]
    ]
;;
