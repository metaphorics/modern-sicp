(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.15 *)

module S = Sicp_ch4.Sec_4_1

let fuel_eval fuel : S.eval_t =
  let remaining = ref fuel in
  let rec eval e env =
    if !remaining <= 0
    then Error (Sicp_common.Eval_error.User_error "no answer within the step budget")
    else (
      decr remaining;
      S.open_eval ~self:eval e env)
  in
  eval
;;

let prefix oracle =
  "let halts = "
  ^ oracle
  ^ " in let rec run_forever u = run_forever u in let try_ p = if halts p p then \
     run_forever () else \"halted\" in "
;;

let diagonal oracle = prefix oracle ^ "try_ try_"
let claim_source oracle = prefix oracle ^ "halts try_ try_"
let claim_eval = fuel_eval 10_000
let oracles = [ "fun _ _ -> true"; "fun _ _ -> false" ]

let ex_4_15 () =
  List.map
    (fun oracle ->
       let claim = Sec_4_1.run_source claim_eval (claim_source oracle) in
       let observed = Sec_4_1.run_source (fuel_eval 10_000) (diagonal oracle) in
       "halts try_ try_ = " ^ claim ^ "; try_ try_ -> " ^ observed)
    oracles
;;
