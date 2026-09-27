(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.77 *)

(** Exercise 3.77: [integral] in the integers-starting-from style,
    made to expect a delayed integrand so it can sit in feedback
    loops. The map class is [T]. *)

(** [integral delayed_integrand initial_value dt] conses the initial
    value and recurses on the forced integrand's tail; the first
    output element never touches the integrand, which is what lets a
    loop close through this procedure. An integrand that is the empty
    stream ends the output after the initial value. *)
val integral
  :  float Sicp_ch3.Sec_3_5.Streams.stream Lazy.t
  -> float
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [solve f y0 dt] generates the solution of dy/dt = f(y) by delaying
    dy in the definition of y. *)
val solve : (float -> float) -> float -> float -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_77 ()] is y(1) of dy/dt = y, y(0) = 1, at dt = 0.001
    (2.7169239322359..., the same e-approximation as 3.5.4's solve,
    now through the cons-stream-style integral) and the two elements
    an empty integrand yields ([5.0] alone). *)
val ex_3_77 : unit -> float * float list
