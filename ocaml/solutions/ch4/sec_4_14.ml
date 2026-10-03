(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.14 *)

let ( let* ) = Result.bind

module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

(* What host code can call: a primitive is a host function, but a
   guest closure is syntax and an environment, which only the evaluator
   can run.  Louis's primitives never receive the evaluator. *)
let host_procedure f =
  match Value.view f with
  | Value.Primitive p ->
    let no_evaluator _ _ =
      Error (Eval_error.Type_error "host code has no evaluator to run a guest procedure")
    in
    Ok (fun args -> p.prim_apply no_evaluator args)
  | _ ->
    Error
      (Eval_error.Type_error
         ("host code cannot call the guest "
          ^ Value.to_string f
          ^ "; only the evaluator can"))
;;

let rec host_list v =
  match Value.view v with
  | Value.Nil -> Ok []
  | Value.Cons (head, tail) ->
    let* tail = host_list tail in
    Ok (head :: tail)
  | _ -> Error (Eval_error.Type_error "not a list")
;;

let guest_list items = List.fold_right Value.cons items Value.nil

let two = function
  | [ a; b ] -> Ok (a, b)
  | args -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args })
;;

let louis_map =
  Value.primitive ~name:"List.map" ~arity:2 (fun _ args ->
    let* f, items = two args in
    let* f = host_procedure f in
    let* items = host_list items in
    let rec map acc = function
      | [] -> Ok (guest_list (List.rev acc))
      | item :: rest ->
        let* v = f [ item ] in
        map (v :: acc) rest
    in
    map [] items)
;;

let louis_sort =
  Value.primitive ~name:"List.sort" ~arity:2 (fun _ args ->
    let* compare, items = two args in
    let* compare = host_procedure compare in
    let* items = host_list items in
    let failure = ref None in
    let host_compare x y =
      match compare [ x; y ] with
      | Ok v ->
        (match Value.view v with
         | Value.Int n -> n
         | _ ->
           failure
           := Some (Eval_error.Type_error "List.sort: comparator did not answer an int");
           0)
      | Error e ->
        failure := Some e;
        0
    in
    let sorted = List.sort host_compare items in
    match !failure with
    | Some e -> Error e
    | None -> Ok (guest_list sorted))
;;

let with_primitive name primitive (eval : S.eval_t) : S.eval_t =
  fun e env -> eval e (Env.bind name primitive env)
;;

let eval = with_primitive "List.map" louis_map S.eval_expr

let eva_map =
  "let rec map f l = match l with [] -> [] | x :: rest -> f x :: map f rest in "
;;

let eva_sort =
  "let rec insert compare x l = match l with [] -> [ x ] | y :: rest -> if compare x y \
   <= 0 then x :: y :: rest else y :: insert compare x rest in let rec sort compare l = \
   match l with [] -> [] | x :: rest -> insert compare x (sort compare rest) in "
;;

let ex_4_14 () =
  let square_all = "List.map (fun x -> x * x) [ 1; 2; 3 ]" in
  [ Sec_4_1.run_source eval "List.map string_of_int [ 1; 2; 3 ]"
  ; Sec_4_1.run_source eval square_all
  ; Sec_4_1.run_source eval (eva_map ^ "map (fun x -> x * x) [ 1; 2; 3 ]")
  ; Sec_4_1.run_source S.eval_expr square_all
  ]
;;

let ex_4_14a () =
  let louis = with_primitive "List.sort" louis_sort S.eval_expr in
  let descending = "(fun a b -> b - a) [ 3; 1; 2 ]" in
  [ Sec_4_1.run_source louis ("List.sort " ^ descending)
  ; Sec_4_1.run_source louis (eva_sort ^ "sort " ^ descending)
  ; Sec_4_1.run_source S.eval_expr ("List.sort " ^ descending)
  ]
;;
