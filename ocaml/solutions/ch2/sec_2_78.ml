(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.78 *)

type value =
  | Num of float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

let type_tag = function
  | Num _ -> "scheme-number"
  | Tagged t -> t.tag
;;

let contents_of = function
  | Num _ as v -> v
  | Tagged t -> t.contents
;;

let attach_tag tag contents =
  if String.equal tag "scheme-number"
  then (
    match contents with
    | Num _ -> contents
    | Tagged _ -> invalid_arg "attach_tag: scheme-number expects a bare Num contents")
  else Tagged { tag; contents }
;;

let ex_2_78 () =
  ( type_tag (Num 5.0)
  , (match contents_of (Num 5.0) with
     | Num n -> Float.equal n 5.0
     | Tagged _ -> false)
  , type_tag (attach_tag "rational" (Num 0.0)) )
;;
