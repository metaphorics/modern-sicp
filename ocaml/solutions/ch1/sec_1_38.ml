(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.38 *)

(** Exercise 1.38: [euler_d]'s sequence 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8,
    ... puts a 2, 4, 6, 8, ... every third term, at [i] where [i + 1]
    is a multiple of 3. [cont_frac] is exercise 1.37's, folded locally
    since exercises and solutions stay self-contained per file. *)

let euler_d i = if (i + 1) mod 3 = 0 then 2.0 *. float_of_int ((i + 1) / 3) else 1.0

let cont_frac n d k =
  let rec go i result = if i = 0 then result else go (i - 1) (n i /. (d i +. result)) in
  go k 0.0
;;

let e_approx k = 2.0 +. cont_frac (fun _ -> 1.0) euler_d k
let ex_1_38 () = e_approx 20
