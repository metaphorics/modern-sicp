(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.86 *)

module type Scalar = sig
  type t

  val add : t -> t -> t
  val mul : t -> t -> t
  val sine : t -> t
  val cosine : t -> t
  val sqrt : t -> t
  val atan2 : t -> t -> t
end

module Make_complex (S : Scalar) = struct
  type t = S.t * S.t

  let make_from_real_imag _a0 _a1 = raise Sicp_common.Pending.Pending_solution
  let real_part _a0 = raise Sicp_common.Pending.Pending_solution
  let imag_part _a0 = raise Sicp_common.Pending.Pending_solution
  let magnitude _a0 = raise Sicp_common.Pending.Pending_solution
  let angle _a0 = raise Sicp_common.Pending.Pending_solution
  let make_from_mag_ang _a0 _a1 = raise Sicp_common.Pending.Pending_solution
end

module Float_scalar = struct
  type t = float

  let add _a0 _a1 = raise Sicp_common.Pending.Pending_solution
  let mul _a0 _a1 = raise Sicp_common.Pending.Pending_solution
  let sine _a0 = raise Sicp_common.Pending.Pending_solution
  let cosine _a0 = raise Sicp_common.Pending.Pending_solution
  let sqrt _a0 = raise Sicp_common.Pending.Pending_solution
  let atan2 _a0 _a1 = raise Sicp_common.Pending.Pending_solution
end

module Complex_float = Make_complex (Float_scalar)

let ex_2_86 () = raise Sicp_common.Pending.Pending_solution
