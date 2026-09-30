(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.19 *)

module Ast = Sicp_common.Ast
module S = Sicp_ch4.Sec_4_1

let binding_name (b : Ast.binding) = b.name

let sequential e =
  match Ast.view e with
  | Ast.Let (true, bindings, body) ->
    let at = Ast.at e in
    List.fold_right (fun b rest -> Ast.let_ ~at false [ b ] rest) bindings body
  | _ -> e
;;

(* The names of [names] that [e] reads while it runs: a reference inside
   a [fun] waits for a call and does not count.  Shadowing is ignored,
   so the answer may name more than [e] reads, never fewer. *)
let rec immediate_uses names e =
  match Ast.view e with
  | Ast.Var name when List.mem name names -> [ name ]
  | Ast.Fun _ -> []
  | _ -> List.concat_map (immediate_uses names) (Sec_4_2.subexpressions e)
;;

let order bindings =
  let names = List.filter_map binding_name bindings in
  let uses (b : Ast.binding) = immediate_uses names b.rhs in
  let rec place placed pending =
    match pending with
    | [] -> Some (List.rev placed)
    | _ ->
      let defined = List.filter_map binding_name placed in
      let ready, waiting =
        List.partition
          (fun b ->
             List.for_all
               (fun n -> List.mem n defined || binding_name b = Some n)
               (uses b))
          pending
      in
      (match ready with
       | [] -> None
       | _ -> place (List.rev_append ready placed) waiting)
  in
  place [] bindings
;;

let simultaneous e =
  match Ast.view e with
  | Ast.Let (true, bindings, body) ->
    let ordered =
      match order bindings with
      | Some ordered -> ordered
      | None -> bindings
    in
    Sec_4_16.scan_out_let_rec (Ast.let_ ~at:(Ast.at e) true ordered body)
  | _ -> e
;;

let evaluator transform =
  let rec self e env = Sec_4_16.scanning transform ~self e env in
  self
;;

let ben = evaluator sequential
let alyssa = Sec_4_16.eval
let eva = evaluator simultaneous

let program =
  "let a = 1 in let f x = let rec b = (a, x) and a = 5 in match b with (p, q) -> a + p + \
   q in f 10"
;;

let ex_4_19 () =
  [ "Ben: " ^ Sec_4_1.run_source ben program
  ; "Alyssa: " ^ Sec_4_1.run_source alyssa program
  ; "Eva: " ^ Sec_4_1.run_source eva program
  ; "OCaml: " ^ Sec_4_1.run_source S.eval_expr program
  ]
;;
