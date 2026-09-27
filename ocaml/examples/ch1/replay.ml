(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

let expect actual expected =
  print_endline actual;
  if not (String.equal actual expected)
  then (
    Printf.eprintf "replay mismatch: expected %S, got %S\n%!" expected actual;
    exit 1)
;;

let expect_float computed shown =
  print_endline shown;
  if not (Float.equal computed (float_of_string shown))
  then (
    Printf.eprintf "replay mismatch: %S does not render %0.17g\n%!" shown computed;
    exit 1)
;;
