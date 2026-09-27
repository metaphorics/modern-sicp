(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program tan-cf in SICP section 1.3
   exercise 1.39 *)

(** Exercise 1.39: Lambert's formula
    [tan x = x / (1 - x^2 / (3 - x^2 / (5 - ...)))] has [N_1 = x],
    every later [N_i = x^2], and [D_i = 2i - 1]; the fraction
    subtracts rather than adds at each level, so [go] uses [-.]
    where exercise 1.37's [cont_frac] used [+.]. *)

let tan_cf x k =
  let n i = if i = 1 then x else x *. x in
  let d i = (2.0 *. float_of_int i) -. 1.0 in
  let rec go i result = if i = 0 then result else go (i - 1) (n i /. (d i -. result)) in
  go k 0.0
;;

let ex_1_39 () = tan_cf 0.1 10, tan_cf 1.0 20
