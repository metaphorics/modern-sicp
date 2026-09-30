(* SPDX-License-Identifier: GPL-3.0-only *)

module Env = Sicp_common.Env
module V = Sicp_common.Value
open QCheck2

(* Environments are immutable chains: a lookup answers the newest
   binding of a name in scope, and extending never disturbs the
   environment it extends.  A generated program of nested scopes is
   compared with an association-list model. *)

type op =
  | Extend of (string * int) list
  | Find of string

let keys = [ "a"; "b"; "c"; "d" ]

let op_gen =
  let open Gen in
  let key = oneof_list keys in
  oneof
    [ map (fun bindings -> Extend bindings) (list_size (int_range 0 3) (pair key int))
    ; map (fun k -> Find k) key
    ]
;;

let show_op = function
  | Extend bindings ->
    "extend ["
    ^ String.concat "; " (List.map (fun (k, v) -> Printf.sprintf "%s=%d" k v) bindings)
    ^ "]"
  | Find k -> "find " ^ k
;;

let as_int v =
  match V.view v with
  | V.Int n -> n
  | _ -> max_int
;;

(* A frame binding one name twice keeps its first binding, as the
   environment's assoc search does. *)
let property ops =
  let rec go env model scopes = function
    | [] -> true
    | Extend bindings :: rest ->
      go
        (Env.extend (List.map (fun (k, v) -> k, V.int v) bindings) env)
        (bindings @ model)
        ((env, model) :: scopes)
        rest
    | Find k :: rest ->
      let actual = Option.map as_int (Env.find env k) in
      let outer_intact =
        List.for_all
          (fun (e, m) -> Option.map as_int (Env.find e k) = List.assoc_opt k m)
          scopes
      in
      actual = List.assoc_opt k model && outer_intact && go env model scopes rest
  in
  go Env.empty [] [] ops
;;

let env_matches_model =
  Test.make
    ~name:"Env lookup answers the newest binding and never disturbs outer scopes"
    ~count:1000
    ~print:(fun ops -> String.concat "; " (List.map show_op ops))
    (Gen.list op_gen)
    property
;;

let () = QCheck_base_runner.run_tests_main [ env_matches_model ]
