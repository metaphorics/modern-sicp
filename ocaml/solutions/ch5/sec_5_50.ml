(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let guest_factorial =
  "ELetRec (\"mul\", \"a\",\n\
  \  ELambda (\"b\", EIf (EEqual (EVar \"b\", EInt 0), EInt 0,\n\
  \    EAdd (EVar \"a\", EApply (EApply (EVar \"mul\", EVar \"a\"), ESub (EVar \"b\", \
   EInt 1))))),\n\
  \  ELetRec (\"factorial\", \"n\",\n\
  \    EIf (EEqual (EVar \"n\", EInt 0), EInt 1,\n\
  \      EApply (EApply (EVar \"mul\", EVar \"n\"),\n\
  \        EApply (EVar \"factorial\", ESub (EVar \"n\", EInt 1)))),\n\
  \    EApply (EVar \"factorial\", EInt 5)))"
;;

let guest_counter =
  "ELet (\"cell\", ERef (EInt 0),\n\
  \  ELet (\"tick\", ELambda (\"u\", EAssign (EVar \"cell\", EAdd (EDeref (EVar \
   \"cell\"), EInt 1))),\n\
  \    ESequence (EApply (EVar \"tick\", EUnit),\n\
  \      ESequence (EApply (EVar \"tick\", EUnit),\n\
  \        ESequence (EApply (EVar \"tick\", EUnit), EDeref (EVar \"cell\"))))))"
;;

let subset_factorial =
  "let rec mul a b = if b = 0 then 0 else a + mul a (b - 1)\n\
   let rec factorial n = if n = 0 then 1 else mul n (factorial (n - 1))\n\
   let () = print_endline (string_of_int (factorial 5))\n"
;;

let output run program =
  let out = Buffer.create 16 in
  let* _ = run ~emit:(Buffer.add_string out) program in
  Ok (String.trim (Buffer.contents out))
;;

let compiled_steps program =
  let out = Buffer.create 16 in
  let* _, steps = C.run_stats ~emit:(Buffer.add_string out) program in
  Ok (String.trim (Buffer.contents out), steps)
;;

let interpreted_steps program =
  let out = Buffer.create 16 in
  let* ev =
    W.make_evaluator ~controller:W.base_controller ~emit:(Buffer.add_string out) ()
  in
  let* _ = W.run_program ev program in
  Ok (String.trim (Buffer.contents out), M.executed (W.machine ev))
;;

let agree what expected actual =
  if String.trim actual = expected
  then Ok ()
  else
    Error
      (Sicp_common.Eval_error.User_error
         (Printf.sprintf "%s answers %S, expected %S" what actual expected))
;;

let ex_5_50 () =
  let metacircular guest =
    Sec_5_33.program ~filename:"ex_5_50.ml" (Sicp_ch5.Metacircular.with_guest guest)
  in
  let* factorial_unit = metacircular guest_factorial in
  let* counter_unit = metacircular guest_counter in
  let* subset = Sec_5_33.program ~filename:"ex_5_50_subset.ml" subset_factorial in
  let* compiled_counter = output C.run counter_unit in
  let* direct = output Sicp_ch4.Sec_4_1.run factorial_unit in
  let* interpreted = output W.run factorial_unit in
  let* level0_out, level0 = compiled_steps subset in
  let* level1_out, level1 = interpreted_steps subset in
  let* level2_out, level2 = compiled_steps factorial_unit in
  let* () = agree "explicit-control" level2_out interpreted in
  let* () = agree "direct" level2_out direct in
  let* () = agree "level 0" level2_out level0_out in
  let* () = agree "level 1" level2_out level1_out in
  let* native_oracle =
    Sec_5_51.ocaml_native_run (Sicp_ch5.Metacircular.with_guest guest_factorial)
  in
  let* counter_oracle =
    Sec_5_51.ocaml_native_run (Sicp_ch5.Metacircular.with_guest guest_counter)
  in
  let* () = agree "native oracle" level2_out native_oracle in
  let* () = agree "native counter" compiled_counter counter_oracle in
  Ok
    [ Printf.sprintf
        "metacircular answers: compiled %s, explicit-control %s, direct %s; counter %s"
        level2_out
        interpreted
        direct
        compiled_counter
    ; Printf.sprintf "native oracle agrees: %s" level2_out
    ; Printf.sprintf "native counter agrees: %s" compiled_counter
    ; Printf.sprintf
        "level 0 (compiled factorial): %s in %d machine steps"
        level0_out
        level0
    ; Printf.sprintf
        "level 1 (factorial on the explicit-control evaluator): %s in %d machine steps"
        level1_out
        level1
    ; Printf.sprintf
        "level 2 (factorial on the compiled metacircular evaluator): %s in %d machine \
         steps"
        level2_out
        level2
    ; Printf.sprintf
        "interpretation price: level 1 is %d times level 0, level 2 is %d times level 0"
        (level1 / level0)
        (level2 / level0)
    ]
;;
