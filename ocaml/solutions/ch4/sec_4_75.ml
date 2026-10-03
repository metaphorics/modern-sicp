(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.75: the [unique] special form.  [uniquely_asserted] is the
   book's handler: for each frame it runs [qeval] on the [unique]
   query's contents, keeps the frame exactly when the extension stream
   has precisely one element, and emits that one extension -- the
   [Not]-style filter Alyssa describes.  The registration is the book's
   own data-directed step, [(put 'unique 'qeval uniquely-asserted)]: the
   engine's [put] installs the handler in the session, and a query
   reaches it as [Form ("unique", [contents])].

   The demonstration pins the book's three behaviors -- the one computer
   wizard prints one row, the two computer programmers print nothing,
   and the [And] of a job scan with [unique] lists every singly-filled
   job with its filler, in the chronological data base's scan order --
   plus the test the exercise asks for, the people who supervise
   precisely one person. *)

open Sec_4_55.Kit

let uniquely_asserted : Q.handler =
  fun s contents frames ->
  match contents with
  | [ query ] ->
    Q.stream_flatmap
      (fun frame ->
         match Q.qeval s query (Q.singleton_stream frame) with
         | Streams.Cons (_, rest) as extensions when Streams.stream_null (Lazy.force rest)
           -> extensions
         | Streams.Empty | Streams.Cons _ -> Streams.the_empty_stream)
      frames
  | _ ->
    raise
      (Q.Query_error
         (Sicp_common.Eval_error.Arity_mismatch
            { expected = 1; given = List.length contents }))
;;

let unique q = Q.Form ("unique", [ q ])

let ex_4_75 () =
  let s = session microshaft in
  Q.put s "unique" uniquely_asserted;
  let answers q = answers_all s q in
  let wizard = answers (unique (p [ at "job"; v "x"; atoms [ "computer"; "wizard" ] ])) in
  let programmer =
    answers (unique (p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ]))
  in
  let singly_filled =
    answers
      (Q.And [ p [ at "job"; v "x"; v "j" ]; unique (p [ at "job"; v "anyone"; v "j" ]) ])
  in
  let supervises_one =
    answers
      (Q.And
         [ p [ at "supervisor"; v "anyone"; v "person" ]
         ; unique (p [ at "supervisor"; v "subordinate"; v "person" ])
         ])
  in
  [ "unique_wizard" ]
  @ wizard
  @ [ "unique_wizard_answers=" ^ string_of_int (List.length wizard); "unique_programmer" ]
  @ programmer
  @ [ "unique_programmer_answers=" ^ string_of_int (List.length programmer)
    ; "singly_filled_jobs"
    ]
  @ singly_filled
  @ [ "singly_filled_jobs_answers=" ^ string_of_int (List.length singly_filled)
    ; "supervises_precisely_one_person"
    ]
  @ supervises_one
  @ [ "supervises_precisely_one_person_answers="
      ^ string_of_int (List.length supervises_one)
    ]
;;
