(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.63: the Genesis 4 genealogy data base and the two rules
    the statement phrases: a grandson is a son of a son, and the son of
    a man's wife is his son. The [grandson] body keeps the statement's
    order -- "S is the son of f" before "f is the son of G" -- and the
    son-of-wife rule is what extends [son] to the mother, so Lamech's
    sons and Methushael's grandsons come out through it. All three
    statement queries are finite. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let assertions =
  [ "(assert! (son Adam Cain))"
  ; "(assert! (son Cain Enoch))"
  ; "(assert! (son Enoch Irad))"
  ; "(assert! (son Irad Mehujael))"
  ; "(assert! (son Mehujael Methushael))"
  ; "(assert! (son Methushael Lamech))"
  ; "(assert! (wife Lamech Ada))"
  ; "(assert! (son Ada Jabal))"
  ; "(assert! (son Ada Jubal))"
  ; "(assert! (rule (grandson ?grandson ?grandfather)\n\
    \     (and (son ?father ?grandson)\n\
    \          (son ?grandfather ?father))))"
  ; "(assert! (rule (son ?m ?s)\n     (and (wife ?m ?w)\n          (son ?w ?s))))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    assertions
;;

let answers_or_fail = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> failwith ("query failed: " ^ Eval_error.to_string e)
;;

(** [ex_4_63 ()] pins the grandson of Cain, the sons of Lamech, and the
    grandsons of Methushael. *)
let ex_4_63 () =
  let env = Eval.the_query_system () in
  load env;
  let cain = answers_or_fail (Eval.query env "(grandson ?x Cain)") in
  let lamech = answers_or_fail (Eval.query env "(son Lamech ?x)") in
  let methushael = answers_or_fail (Eval.query env "(grandson ?x Methushael)") in
  [ "query: (grandson ?x Cain)" ]
  @ cain
  @ [ "query: (son Lamech ?x)" ]
  @ lamech
  @ [ "query: (grandson ?x Methushael)" ]
  @ methushael
;;
