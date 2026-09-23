(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.4 *)

(** [cons x y] is [fun m -> m x y]; [car z] applies [z] to a selector
    that returns its first argument, so substituting [cons x y] for
    [z] leaves [(fun m -> m x y) (fun p _q -> p)], which beta-reduces
    to [(fun p _q -> p) x y = x]. [cdr] is the mirror image, selecting
    the second argument instead. *)

let cons x y m = m x y
let car z = z (fun p _q -> p)
let cdr z = z (fun _p q -> q)
let ex_2_04 x y = car (cons x y) = x && cdr (cons x y) = y
