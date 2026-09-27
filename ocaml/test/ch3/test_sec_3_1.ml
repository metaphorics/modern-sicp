(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 3.1. [sicp_ch3_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_3_1 = Sicp_ch3_solutions.Sec_3_1
module Sec_3_2 = Sicp_ch3_solutions.Sec_3_2
module Sec_3_3 = Sicp_ch3_solutions.Sec_3_3
module Sec_3_4 = Sicp_ch3_solutions.Sec_3_4
module Sec_3_5 = Sicp_ch3_solutions.Sec_3_5
module Sec_3_6 = Sicp_ch3_solutions.Sec_3_6
module Sec_3_7 = Sicp_ch3_solutions.Sec_3_7
module Sec_3_8 = Sicp_ch3_solutions.Sec_3_8

let float3 = Alcotest.float 1e-3

let ex_3_01_accumulator () =
  Alcotest.(check (pair int int)) "running sum" (15, 25) (Sec_3_1.ex_3_01 ())
;;

let ex_3_02_monitored () =
  let first, second = Sec_3_2.ex_3_02 () in
  (match first with
   | Sec_3_2.Value v -> Alcotest.(check float3) "sqrt 100" 10.0 v
   | _ -> Alcotest.fail "expected Value, got a different response");
  match second with
  | Sec_3_2.Count n -> Alcotest.(check int) "one call so far" 1 n
  | _ -> Alcotest.fail "expected Count, got a different response"
;;

let ex_3_03_password_account () =
  let first, second = Sec_3_3.ex_3_03 () in
  Alcotest.(check bool)
    "correct password withdraws"
    true
    (match first with
     | Sec_3_3.Balance_response (Sec_3_3.Balance 60) -> true
     | _ -> false);
  Alcotest.(check bool)
    "wrong password refuses"
    true
    (match second with
     | Sec_3_3.Incorrect_password -> true
     | _ -> false)
;;

let ex_3_04_lockout () =
  let responses = Sec_3_4.ex_3_04 () in
  let expected =
    List.init 8 (fun i ->
      if i < 7 then Sec_3_4.Incorrect_password else Sec_3_4.Police_called)
  in
  Alcotest.(check bool) "seven refusals then the cops" true (responses = expected)
;;

let ex_3_05_monte_carlo_pi () =
  let estimate = Sec_3_5.ex_3_05 () in
  Alcotest.(check bool)
    "estimate is within 0.2 of pi"
    true
    (Float.abs (estimate -. Float.pi) < 0.2)
;;

let ex_3_06_reset_reproduces () =
  let first, third = Sec_3_6.ex_3_06 () in
  Alcotest.(check int) "reset to the original seed reproduces the draw" first third
;;

let ex_3_07_joint_account () =
  let withdrawal, peters_view = Sec_3_7.ex_3_07 () in
  Alcotest.(check bool)
    "Paul's withdrawal through the joint account"
    true
    (match withdrawal with
     | Sec_3_7.Balance_response (Sec_3_7.Balance 60) -> true
     | _ -> false);
  Alcotest.(check bool)
    "Peter observes the same balance"
    true
    (match peters_view with
     | Sec_3_7.Deposit_response 60 -> true
     | _ -> false)
;;

let ex_3_07a_read_only_capability () =
  Alcotest.(check (pair int int)) "balance before and after a deposit" (100, 150)
  @@ Sec_3_7.ex_3_07a ()
;;

let ex_3_08_evaluation_order () =
  let result = Sec_3_8.ex_3_08 () in
  Alcotest.(check bool) "one operand contributes 0, the other its own argument" true
  @@ (result = 0 || result = 1)
;;

let () =
  Alcotest.run
    "sicp_ch3 solutions, section 3.1"
    [ "3.1 accumulator", [ Alcotest.test_case "running sum" `Quick ex_3_01_accumulator ]
    ; ( "3.2 monitored procedure"
      , [ Alcotest.test_case "counts and resets" `Quick ex_3_02_monitored ] )
    ; ( "3.3 password-protected account"
      , [ Alcotest.test_case "gates on the password" `Quick ex_3_03_password_account ] )
    ; ( "3.4 lockout after seven bad passwords"
      , [ Alcotest.test_case "calls the cops on the eighth" `Quick ex_3_04_lockout ] )
    ; ( "3.5 Monte Carlo integration"
      , [ Alcotest.test_case "estimates pi" `Quick ex_3_05_monte_carlo_pi ] )
    ; ( "3.6 rand with generate and reset"
      , [ Alcotest.test_case
            "reset reproduces the sequence"
            `Quick
            ex_3_06_reset_reproduces
        ] )
    ; ( "3.7 make-joint"
      , [ Alcotest.test_case "shares one account" `Quick ex_3_07_joint_account ] )
    ; ( "3.7a read-only account capability"
      , [ Alcotest.test_case
            "observes live state through a narrower type"
            `Quick
            ex_3_07a_read_only_capability
        ] )
    ; ( "3.8 operand evaluation order"
      , [ Alcotest.test_case "one order or the other" `Quick ex_3_08_evaluation_order ] )
    ]
;;
