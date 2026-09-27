(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.53 *)

(** Exercise 2.53 (this edition's replacement): predict what OCaml
    prints for seven expressions over the section's symbol data. The
    expressions below are the ones the exercise statement lists; the
    printed forms are computed from the values, so the prediction the
    statement asks for is checked against the real data, not against
    hand-copied text. *)

type symbol = Sym of string

let rec memq item = function
  | [] -> None
  | first :: rest -> if first = item then Some (first :: rest) else memq item rest
;;

(* The seven expressions, in the statement's order. *)
let e1 = [ Sym "a"; Sym "b"; Sym "c" ]
let e2 = [ [ Sym "george" ] ]
let nested = [ [ Sym "x1"; Sym "x2" ]; [ Sym "y1"; Sym "y2" ] ]
let e3 = List.tl nested
let e4 = List.hd (List.tl nested)
let e5 = List.hd [ Sym "a"; Sym "short"; Sym "list" ] = Sym "a"
let e6 = memq (Sym "red") [ Sym "shoes"; Sym "blue"; Sym "socks" ]
let e7 = memq (Sym "red") [ Sym "red"; Sym "shoes"; Sym "blue"; Sym "socks" ]

let show_symbols l =
  "[" ^ String.concat "; " (List.map (fun (Sym s) -> "Sym \"" ^ s ^ "\"") l) ^ "]"
;;

let show_symbol_lists l = "[" ^ String.concat "; " (List.map show_symbols l) ^ "]"

let show_option_symbols = function
  | None -> "None"
  | Some l -> "Some " ^ show_symbols l
;;

(** [ex_2_53 ()] is the seven printed values, in order. *)
let ex_2_53 () =
  [ show_symbols e1
  ; show_symbol_lists e2
  ; show_symbol_lists e3
  ; show_symbols e4
  ; string_of_bool e5
  ; show_option_symbols e6
  ; show_option_symbols e7
  ]
;;
