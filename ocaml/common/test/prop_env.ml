(* SPDX-License-Identifier: GPL-3.0-only *)

module E = Sicp_common.Eval_error
module Env = Sicp_common.Env
module V = Sicp_common.Value
open QCheck2

type op =
  | Define of string * int
  | Set of string * int
  | Find of string

let keys = [ "a"; "b"; "c"; "d" ]

let op_gen =
  let open Gen in
  let key = oneof_list keys in
  oneof
    [ map2 (fun k v -> Define (k, v)) key int
    ; map2 (fun k v -> Set (k, v)) key int
    ; map (fun k -> Find k) key
    ]
;;

let show_op = function
  | Define (k, v) -> Printf.sprintf "define %s %d" k v
  | Set (k, v) -> Printf.sprintf "set! %s %d" k v
  | Find k -> Printf.sprintf "find %s" k
;;

let show_ops ops = String.concat "; " (List.map show_op ops)

(* The model: a list of assoc-list frames, newest first, mirroring the
   documented Env semantics. *)

let model_find model k =
  let rec go = function
    | [] -> None
    | frame :: rest ->
      (match List.assoc_opt k frame with
       | Some v -> Some v
       | None -> go rest)
  in
  go model
;;

let model_define model k v =
  match model with
  | [] -> model
  | frame :: rest -> ((k, v) :: List.remove_assoc k frame) :: rest
;;

let rec model_set model k v =
  match model with
  | [] -> None
  | frame :: rest ->
    if List.mem_assoc k frame
    then Some (((k, v) :: List.remove_assoc k frame) :: rest)
    else Option.map (fun rest' -> frame :: rest') (model_set rest k v)
;;

let property ops =
  let env = Env.empty () in
  let rec go model = function
    | [] -> true
    | op :: rest ->
      (match op, model with
       | Define (k, v), model ->
         Env.define env k (V.int v);
         go (model_define model k v) rest
       | Set (k, v), model ->
         (match Env.set env k (V.int v), model_set model k v with
          | Ok (), Some model' -> go model' rest
          | Error E.(Unbound_variable _), None -> go model rest
          | _ -> false)
       | Find k, model ->
         let actual =
           Option.map
             (fun value ->
                match V.view value with
                | V.Int n -> n
                | _ -> max_int)
             (Env.find_binding env k)
         in
         if actual = model_find model k then go model rest else false)
  in
  go [ [] ] ops
;;

let env_matches_model =
  Test.make
    ~name:"Env matches a frame-list model"
    ~count:1000
    ~print:show_ops
    (Gen.sized (fun _ -> Gen.list op_gen))
    property
;;

let () = QCheck_base_runner.run_tests_main [ env_matches_model ]
