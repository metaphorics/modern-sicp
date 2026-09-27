(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

let create_exn seed =
  match Sicp_common.Random.create seed with
  | Ok t -> t
  | Error Sicp_common.Error.Zero_seed -> Alcotest.fail "create rejected a nonzero seed"
;;

let seed_1_draws () =
  let t = create_exn 1L in
  Alcotest.(check (list int))
    "seed 1 draws from random 1000"
    [ 165; 517; 103; 413; 928 ]
    (List.init 5 (fun _ -> Sicp_common.Random.random t 1000))
;;

let rejects_zero_seed () =
  match Sicp_common.Random.create 0L with
  | Error Sicp_common.Error.Zero_seed -> ()
  | Ok _ -> Alcotest.fail "create accepted a zero seed"
;;

let same_seed_same_draws () =
  let a = create_exn 1L in
  let b = create_exn 1L in
  Alcotest.(check (list int))
    "two generators from one seed agree"
    (List.init 5 (fun _ -> Sicp_common.Random.random a 1000))
    (List.init 5 (fun _ -> Sicp_common.Random.random b 1000))
;;

let () =
  Alcotest.run
    "sicp_common.random"
    [ ( "random"
      , Alcotest.
          [ test_case "seed 1 draws" `Quick seed_1_draws
          ; test_case "rejects zero seed" `Quick rejects_zero_seed
          ; test_case "same seed same draws" `Quick same_seed_same_draws
          ] )
    ]
;;
