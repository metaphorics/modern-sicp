(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.58: the [big-shot] rule -- a person is a big shot in a
   division when they work in it and their supervisor, if any, works in
   a different one.  The job pattern [[?division | ?rest]] binds the
   division once and the dotted patterns compare divisions
   structurally, so the rule needs no extra data; Warbucks, who has no
   supervisor assertion at all, qualifies through the rule's [Or] of
   bossless and boss-in-another-division.  The [Or]'s first disjunct,
   the bossless case, answers for Warbucks alone, so he lands first; the
   second disjunct then answers Ben in computer (his boss Warbucks is
   administration) and Scrooge in accounting, in job-assertion order. *)

open Sec_4_55.Kit

let rules =
  [ ( l [ at "big-shot"; v "person"; v "division" ]
    , Q.And
        [ p [ at "job"; v "person"; Q.dotted [ v "division" ] (v "rest") ]
        ; Q.Or
            [ Q.Not (p [ at "supervisor"; v "person"; v "boss" ])
            ; Q.And
                [ p [ at "supervisor"; v "person"; v "boss" ]
                ; Q.Not (p [ at "job"; v "boss"; Q.dotted [ v "division" ] (v "rest-2") ])
                ]
            ]
        ] )
  ]
;;

let ex_4_58 () =
  transcript (session ~rules microshaft) [ p [ at "big-shot"; v "person"; v "division" ] ]
;;
