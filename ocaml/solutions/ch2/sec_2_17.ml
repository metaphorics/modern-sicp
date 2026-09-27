(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.17: last_pair of a nonempty list. *)

let rec ex_2_17 items =
  match items with
  | [] -> invalid_arg "ex_2_17: empty list"
  | [ _ ] -> items
  | _ :: rest -> ex_2_17 rest
;;
