(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: a personnel data base of 4.4.1 built from typed
   terms, the section's simple, compound, and rule queries, and the
   append relation of 4.4.1, each transcript proved with [expect]. *)

module Replay = Sicp_ch1.Replay
module Q = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error

let a = Q.list
let atoms names = Q.list (List.map (fun n -> Q.Atom n) names)
let at n = Q.Atom n

let people =
  [ "Bitdiddle Ben", "wizard", 60000, None
  ; "Hacker Alyssa P", "programmer", 40000, Some "Bitdiddle Ben"
  ; "Fect Cy D", "programmer", 35000, Some "Bitdiddle Ben"
  ; "Tweakit Lem E", "technician", 25000, Some "Bitdiddle Ben"
  ]
;;

let person name = atoms (String.split_on_char ' ' name)

let data_base =
  List.concat_map
    (fun (name, role, salary, boss) ->
       [ Q.Assert (a [ at "job"; person name; atoms [ "computer"; role ] ])
       ; Q.Assert (a [ at "salary"; person name; Q.Num salary ])
       ]
       @ Option.to_list
           (Option.map
              (fun b -> Q.Assert (a [ at "supervisor"; person name; person b ]))
              boss))
    people
;;

let session commands =
  let out = Buffer.create 256 in
  match Q.run ~emit:(Buffer.add_string out) (Q.new_session ()) commands with
  | Ok () -> Buffer.contents out
  | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e
;;

let () =
  Replay.expect
    (session
       (data_base
        @ [ Q.Query
              (Q.Pattern (a [ at "job"; Q.var "x"; atoms [ "computer"; "programmer" ] ]))
          ; Q.Query
              (Q.Pattern
                 (a [ at "job"; Q.var "x"; Q.dotted [ at "computer" ] (Q.var "type") ]))
          ; Q.Query
              (Q.And
                 [ Q.Pattern (a [ at "salary"; Q.var "p"; Q.var "amount" ])
                 ; Q.Holds (">", [ Q.var "amount"; Q.Num 30000 ])
                 ])
          ; Q.Rule
              ( a [ at "supervises-programmer"; Q.var "boss" ]
              , Q.And
                  [ Q.Pattern (a [ at "supervisor"; Q.var "p"; Q.var "boss" ])
                  ; Q.Pattern
                      (a [ at "job"; Q.var "p"; atoms [ "computer"; "programmer" ] ])
                  ] )
          ; Q.Query (Q.Pattern (a [ at "supervises-programmer"; Q.var "who" ]))
          ; Q.Query
              (Q.Not
                 (Q.Pattern (a [ at "supervisor"; person "Bitdiddle Ben"; Q.var "b" ])))
          ]))
    "? [job, ?x, [computer, programmer]]\n\
     [job, [Hacker, Alyssa, P], [computer, programmer]]\n\
     [job, [Fect, Cy, D], [computer, programmer]]\n\
     ? [job, ?x, [computer | ?type]]\n\
     [job, [Bitdiddle, Ben], [computer, wizard]]\n\
     [job, [Hacker, Alyssa, P], [computer, programmer]]\n\
     [job, [Fect, Cy, D], [computer, programmer]]\n\
     [job, [Tweakit, Lem, E], [computer, technician]]\n\
     ? and([salary, ?p, ?amount], holds(>, ?amount, 30000))\n\
     and([salary, [Bitdiddle, Ben], 60000], holds(>, 60000, 30000))\n\
     and([salary, [Hacker, Alyssa, P], 40000], holds(>, 40000, 30000))\n\
     and([salary, [Fect, Cy, D], 35000], holds(>, 35000, 30000))\n\
     ? [supervises-programmer, ?who]\n\
     [supervises-programmer, [Bitdiddle, Ben]]\n\
     [supervises-programmer, [Bitdiddle, Ben]]\n\
     ? not([supervisor, [Bitdiddle, Ben], ?b])\n\
     not([supervisor, [Bitdiddle, Ben], ?b])\n";
  (* 4.4.1: append as two rules, run forwards and backwards. *)
  Replay.expect
    (session
       [ Q.Rule (a [ at "append-to-form"; Q.Nil; Q.var "y"; Q.var "y" ], Q.Always_true)
       ; Q.Rule
           ( a
               [ at "append-to-form"
               ; Q.dotted [ Q.var "u" ] (Q.var "v")
               ; Q.var "y"
               ; Q.dotted [ Q.var "u" ] (Q.var "z")
               ]
           , Q.Pattern (a [ at "append-to-form"; Q.var "v"; Q.var "y"; Q.var "z" ]) )
       ; Q.Query
           (Q.Pattern
              (a [ at "append-to-form"; Q.var "x"; Q.var "y"; atoms [ "a"; "b" ] ]))
       ])
    "? [append-to-form, ?x, ?y, [a, b]]\n\
     [append-to-form, [], [a, b], [a, b]]\n\
     [append-to-form, [a], [b], [a, b]]\n\
     [append-to-form, [a, b], [], [a, b]]\n";
  (* The occurs check: [?x] cannot unify with a term containing itself. *)
  Replay.expect
    (session
       [ Q.Rule (a [ at "same"; Q.var "v"; Q.var "v" ], Q.Always_true)
       ; Q.Query (Q.Pattern (a [ at "same"; Q.var "x"; a [ at "f"; Q.var "x" ] ]))
       ])
    "? [same, ?x, [f, ?x]]\n"
;;
