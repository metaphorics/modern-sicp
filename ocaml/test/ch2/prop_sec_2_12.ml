(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Addition 2.12a: the endpoint-based and center-width-based
   [add_interval] agree. The unit spot check in [test_sec_2_1.ml]
   covers the ordinary exercise 2.12 contract. *)

module Sec_2_12 = Sicp_ch2_solutions.Sec_2_12
open QCheck2

let bounded = Gen.float_range (-1000.0) 1000.0

let interval_gen =
  Gen.map2
    (fun a b -> Sec_2_12.make_interval (Float.min a b) (Float.max a b))
    bounded
    bounded
;;

let close a b = Float.abs (a -. b) < 1e-6

let endpoint_and_center_width_agree =
  Test.make
    ~name:"add_interval agrees with add_interval_by_center_width"
    ~count:200
    ~print:(fun (x, y) ->
      Printf.sprintf
        "([%g, %g], [%g, %g])"
        (Sec_2_12.lower_bound x)
        (Sec_2_12.upper_bound x)
        (Sec_2_12.lower_bound y)
        (Sec_2_12.upper_bound y))
    (Gen.pair interval_gen interval_gen)
    (fun (x, y) ->
       let via_endpoints = Sec_2_12.add_interval x y in
       let via_center_width = Sec_2_12.add_interval_by_center_width x y in
       close (Sec_2_12.lower_bound via_endpoints) (Sec_2_12.lower_bound via_center_width)
       && close
            (Sec_2_12.upper_bound via_endpoints)
            (Sec_2_12.upper_bound via_center_width))
;;

let () = QCheck_base_runner.run_tests_main [ endpoint_and_center_width_agree ]
