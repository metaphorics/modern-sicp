(* SPDX-License-Identifier: GPL-3.0-only *)

module A = Sicp_common.Ast
module R = Sicp_common.Reader
module V = Sicp_common.Value
open QCheck2

let rec datum_to_value = function
  | A.DInt n -> V.int n
  | A.DFloat f -> V.float f
  | A.DBool b -> V.bool b
  | A.DString s -> V.string s
  | A.DSymbol s -> V.symbol s
  | A.DNil -> V.nil
  | A.DPair (a, d) -> V.pair (datum_to_value a) (datum_to_value d)
;;

let sexp_of_datum d = V.to_string (datum_to_value d)

let symbol_gen =
  let letter = Gen.oneof [ Gen.char_range 'a' 'z'; Gen.char_range 'A' 'Z' ] in
  let rest_char =
    Gen.oneof
      [ letter
      ; Gen.char_range '0' '9'
      ; Gen.oneof_array [| '-'; '?'; '!'; '*'; '+'; '/'; '<'; '>'; '='; '_'; '.' |]
      ]
  in
  Gen.map2
    (fun head rest ->
       String.concat "" (String.make 1 head :: List.map (String.make 1) rest))
    letter
    (Gen.list rest_char)
;;

let finite_float_gen = Gen.map (fun n -> float_of_int n *. 1.5) Gen.int

let rec datum_gen depth =
  let atom =
    Gen.oneof
      [ Gen.map (fun n -> A.DInt n) Gen.int
      ; Gen.map (fun f -> A.DFloat f) finite_float_gen
      ; Gen.map (fun b -> A.DBool b) Gen.bool
      ; Gen.map (fun s -> A.DString s) Gen.string
      ; Gen.map (fun s -> A.DSymbol s) symbol_gen
      ]
  in
  if depth <= 0
  then atom
  else
    Gen.oneof_weighted
      [ 6, atom
      ; ( 2
        , Gen.map2
            (fun a b -> A.DPair (a, b))
            (datum_gen (depth - 1))
            (datum_gen (depth - 1)) )
      ; ( 1
        , Gen.map
            (fun items -> List.fold_right (fun d acc -> A.DPair (d, acc)) items A.DNil)
            (Gen.list (datum_gen (depth - 1))) )
      ]
;;

let quoted_datum_round_trips =
  let gen = Gen.sized (fun n -> datum_gen (n mod 4)) in
  Test.make
    ~name:"Reader round-trips quoted data"
    ~count:100
    ~print:(fun d -> "'" ^ sexp_of_datum d)
    gen
    (fun d ->
       let input = "'" ^ sexp_of_datum d in
       match R.read input with
       | Ok e -> A.view e = A.Quote d
       | Error err -> Test.fail_reportf "rejected %S: %s" input (R.to_string err))
;;

let () = QCheck_base_runner.run_tests_main [ quoted_datum_round_trips ]
