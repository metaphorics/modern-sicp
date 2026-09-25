(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.75: the [unique] special form.  [uniquely_asserted] is
    the book's handler: for each frame it runs [qeval] on the [unique]
    query's contents, keeps the frame exactly when the extension stream
    has precisely one element, and emits that one extension -- the
    [not]-style filter Alyssa describes.  The registration is the book's
    own data-directed step, [(put 'unique 'qeval uniquely-asserted)],
    done at module load; the substrate's dispatch table is code that
    survives [the_query_system]'s reset, so every fresh session in this
    module sees the handler.

    The demonstration pins the book's three behaviors -- the one
    computer wizard prints one row, the two computer programmers print
    nothing, and the [and] of a job scan with [unique] lists every
    singly-filled job with its filler, in the chronological data base's
    scan order -- plus the test the exercise asks for, the people who
    supervise precisely one person. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The Microshaft personnel data base of 4.4.1, in the book's order. *)
let assertions =
  [ "(assert! (address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))"
  ; "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (address (Fect Cy D) (Cambridge (Ames Street) 3)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (address (Tweakit Lem E) (Boston (Bay State Road) 22)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (address (Warbucks Oliver) (Swellesley (Top Heap Road))))"
  ; "(assert! (job (Warbucks Oliver) (administration big wheel)))"
  ; "(assert! (salary (Warbucks Oliver) 150000))"
  ; "(assert! (address (Scrooge Eben) (Weston (Shady Lane) 10)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (address (Cratchet Robert) (Allston (N Harvard Street) 16)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (address (Aull DeWitt) (Slumerville (Onion Square) 5)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (can-do-job (computer wizard) (computer programmer)))"
  ; "(assert! (can-do-job (computer wizard) (computer technician)))"
  ; "(assert! (can-do-job (computer programmer) (computer programmer trainee)))"
  ; "(assert! (can-do-job (administration secretary) (administration big wheel)))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | _ -> failwith "load: expected an assertion")
    assertions
;;

(* [(put 'unique 'qeval uniquely-asserted)]: the handler keeps only the
   frames whose extension stream for the [unique] contents has exactly
   one element, and emits that extension. *)
let uniquely_asserted env contents frames =
  Eval.stream_flatmap
    (fun frame ->
       let extensions =
         Eval.qeval env (Eval.first_operand contents) (Eval.singleton_stream frame)
       in
       if
         (not (Streams.stream_null extensions))
         && Streams.stream_null (Streams.stream_cdr extensions)
       then extensions
       else Streams.the_empty_stream)
    frames
;;

let () = Eval.put "unique" "qeval" uniquely_asserted

let answer_lines env query =
  match Eval.query env query with
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

let ex_4_75 () =
  let env = Eval.the_query_system () in
  load env;
  let wizard = answer_lines env "(unique (job ?x (computer wizard)))" in
  let programmer = answer_lines env "(unique (job ?x (computer programmer)))" in
  let singly_filled = answer_lines env "(and (job ?x ?j) (unique (job ?anyone ?j)))" in
  let supervises_one =
    answer_lines
      env
      "(and (supervisor ?anyone ?person) (unique (supervisor ?subordinate ?person)))"
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
