# SPDX-License-Identifier: MIT
"""Split Texinfo with a reversible, byte-checked substitution manifest."""

import argparse
import base64
import csv
import re
import sys
from collections.abc import Callable, Sequence
from dataclasses import dataclass
from pathlib import Path

REWRITES: dict[str, str] = {
    (
        "\n"
        "\\[ % :1:\n"
        "\n"
        "\\left|{x}\\right| \\; = \\;\n"
        "  \\left\\{ \n"
        "    \\begin{array}{rll}\t \t \n"
        "       x & \\;\\text{if} & x \\gt 0, \\\\\n"
        "       0 & \\;\\text{if} & x  =  0, \\\\\n"
        "      -x & \\;\\text{if} & x \\lt 0. \n"
        "    \\end{array} \n"
        "  \\right. \n"
        "\\]\n"
        "\n"
    ): (
        "\\left|{x}\\right| \\; = \\;\n"
        "  \\cases{\t \t \n"
        "       x & \\hbox{if } $x > 0$, \\cr\n"
        "       0 & \\hbox{if } $x  =  0$, \\cr\n"
        "      -x & \\hbox{if } $x < 0$. \n"
        "    }"
    ),
    (
        "\n"
        "\\[ % :7:\n"
        "  \n"
        "\\text{Fib}(n) \\; = \\;\n"
        "  \\left\\{ \n"
        "    \\begin{array}{ll}\t \t \n"
        "      0 & \\;\\text{if} \\;\\; n = 0, \\\\\n"
        "      1 & \\;\\text{if} \\;\\; n = 1, \\\\\n"
        "      \\text{Fib}(n-1) + \\text{Fib}(n-2) & \\;\\text{otherwise}. \n"
        "    \\end{array} \n"
        "  \\right. \n"
        "\\]\n"
        "\n"
    ): (
        "\\hbox{Fib}(n) \\; = \\;\n"
        "  \\cases{\t \t \n"
        "      0 & \\hbox{if } $n = 0$, \\cr\n"
        "      1 & \\hbox{if } $n = 1$, \\cr\n"
        "      $\\hbox{Fib}(n-1) + \\hbox{Fib}(n-2)$ & \\hbox{otherwise}. \n"
        "    }"
    ),
    (
        "\n"
        "\\[ % :10:\n"
        " \n"
        "\\begin{array}{l}\n"
        "  a \\;\\leftarrow\\; a + b, \\\\ \n"
        "  b \\;\\leftarrow\\; a. \n"
        "\\end{array}\n"
        "\\]\n"
        "\n"
    ): ("\\matrix{\n  a \\;\\leftarrow\\; a + b, \\cr \n  b \\;\\leftarrow\\; a. \n}"),
    (
        "\n"
        "\\[ % :13:\n"
        " \n"
        "\\begin{array}{l}\n"
        "  b^n \\,=\\, b\\cdot b^{n-1}, \\\\ \n"
        "  b^0 \\,=\\, 1, \n"
        "\\end{array}\n"
        "\\]\n"
        "\n"
    ): ("\\matrix{\n  b^n \\,=\\, b\\cdot b^{n-1}, \\cr \n  b^0 \\,=\\, 1, \n}"),
    (
        "\n"
        "\\[ % :15:\n"
        " \n"
        "\\begin{array}{l}\n"
        "  b^2 \\,=\\, b\\cdot b, \\\\ \n"
        "  b^4 \\,=\\, b^2\\cdot b^2, \\\\\n"
        "  b^8 \\,=\\, b^4\\cdot b^4.\n"
        "\\end{array}\n"
        "\\]\n"
        "\n"
    ): (
        "\\matrix{\n"
        "  b^2 \\,=\\, b\\cdot b, \\cr \n"
        "  b^4 \\,=\\, b^2\\cdot b^2, \\cr\n"
        "  b^8 \\,=\\, b^4\\cdot b^4.\n"
        "}"
    ),
    (
        "\n"
        "\\[ % :16:\n"
        " \n"
        "\\begin{array}{ll}\n"
        "  b^n \\,=\\, (b^{n / 2})^2   & \\text{if} \\; n \\; \\text{is even}, \\\\\n"
        "  b^n \\,=\\, b\\cdot b^{n-1}  & \\text{if} \\; n \\; \\text{is odd}.\n"
        "\\end{array}\n"
        "\\]\n"
        "\n"
    ): (
        "\\matrix{\n"
        "  b^n \\,=\\, (b^{n / 2})^2   & \\hbox{if} \\; n \\; \\hbox{is even}, \\cr\n"
        "  b^n \\,=\\, b\\cdot b^{n-1}  & \\hbox{if} \\; n \\; \\hbox{is odd}.\n"
        "}"
    ),
    (
        "\n"
        "\\[ % :23:\n"
        " \n"
        "\\begin{eqnarray}\n"
        "  a                     &=&   {1 + xy,}  \\\\\n"
        "  \\hphantom{(x,y)} b    &=&   {1 - y,}   \\\\\n"
        "  {f(x,y)}              &=&   {xa^2} + {yb} + {ab.}\n"
        "\\end{eqnarray}\n"
        "\\]\n"
        "\n"
    ): (
        "\\eqalign{\n"
        "  a                     &=   {1 + xy,}  \\cr\n"
        "  \\hphantom{(x,y)} b    &=   {1 - y,}   \\cr\n"
        "  {f(x,y)}              &=   {xa^2} + {yb} + {ab.}\n"
        "}"
    ),
    (
        "\\[ % :30:\n"
        " \n"
        "\\begin{eqnarray}\n"
        "{n_1 \\over d_1} + {n_2 \\over d_2}       &=& {n_1 d_2 + n_2 d_1 \\over d_1 d_2}, \\\\\n"
        "{n_1 \\over d_1} - {n_2 \\over d_2} \t&=& {n_1 d_2 - n_2 d_1 \\over d_1 d_2}, \\\\\n"
        "{n_1 \\over d_1} \\times {n_2 \\over d_2} \t&=& {n_1 n_2 \\over d_1 d_2}, \\\\\n"
        "{n_1 \\,/\\, d_1} \\over {n_2 \\,/\\, d_2} \t&=& {n_1 d_2 \\over d_1 n_2}, \\\\\n"
        "{n_1 \\over d_1} \t\t\t&=& {n_2 \\over d_2} \\quad\n"
        "\t\t\t\t\t\t{\\rm\\ if\\ and\\ only\\ if\\quad} \n"
        "\t\t\t\t\t\tn_1 d_2 = n_2 d_1. \n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "{n_1 \\over d_1} + {n_2 \\over d_2}       &= {n_1 d_2 + n_2 d_1 \\over d_1 d_2}, \\cr\n"
        "{n_1 \\over d_1} - {n_2 \\over d_2} \t&= {n_1 d_2 - n_2 d_1 \\over d_1 d_2}, \\cr\n"
        "{n_1 \\over d_1} \\times {n_2 \\over d_2} \t&= {n_1 n_2 \\over d_1 d_2}, \\cr\n"
        "{n_1 \\,/\\, d_1} \\over {n_2 \\,/\\, d_2} \t&= {n_1 d_2 \\over d_1 n_2}, \\cr\n"
        "{n_1 \\over d_1} \t\t\t&= {n_2 \\over d_2} \\quad\n"
        "\t\t\t\t\t\t{\\rm\\ if\\ and\\ only\\ if\\quad} \n"
        "\t\t\t\t\t\tn_1 d_2 = n_2 d_1. \n"
        "}"
    ),
    (
        "\\[ % :39:\n"
        " \n"
        "\\begin{array}{c|ccccccc}\n"
        "i \t& 2 & 3 & 4 & 4 & 5 & 6 & 6 \\\\\n"
        "j \t& 1 & 2 & 1 & 3 & 2 & 1 & 5 \\\\\n"
        "\\hline\n"
        "i + j\t& 3 & 5 & 5 & 7 & 7 & 7 & 11 \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        "i \t& 2 & 3 & 4 & 4 & 5 & 6 & 6 \\cr\n"
        "j \t& 1 & 2 & 1 & 3 & 2 & 1 & 5 \\cr\n"
        "\\noalign{\\hrule}\n"
        "i + j\t& 3 & 5 & 5 & 7 & 7 & 7 & 11 \n"
        "}"
    ),
    (
        "\\[ % :41:\n"
        "  \n"
        "\\begin{eqnarray}\n"
        "\t(x_1, y_1) + (x_2, y_2)  &=&  (x_1 + x_2, y_1 + y_2), \\\\\n"
        "\t(x_1, y_1) - (x_2, y_2)  &=&  (x_1 - x_2, y_1 - y_2), \\\\\n"
        "\ts \\cdot (x, y) \t\t &=&  (sx, sy).\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "\t(x_1, y_1) + (x_2, y_2)  &=  (x_1 + x_2, y_1 + y_2), \\cr\n"
        "\t(x_1, y_1) - (x_2, y_2)  &=  (x_1 - x_2, y_1 - y_2), \\cr\n"
        "\ts \\cdot (x, y) \t\t &=  (sx, sy).\n"
        "}"
    ),
    (
        "\\[ % :43:\n"
        "\\begin{eqnarray}\n"
        "{dx \\over dx}         &=&   1, \\\\\n"
        "{d(u + v) \\over dx}   &=&   {du \\over dx} + {dv \\over dx}, \\\\\n"
        "{d(uv) \\over dx}      &=&   u \\kern0.1em {dv \\over dx}"
        " + v \\kern0.1em {du \\over dx}.\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "{dx \\over dx}         &=   1, \\cr\n"
        "{d(u + v) \\over dx}   &=   {du \\over dx} + {dv \\over dx}, \\cr\n"
        "{d(uv) \\over dx}      &=   u \\kern0.1em {dv \\over dx} + v \\kern0.1em {du \\over dx}.\n"
        "}"
    ),
    (
        "\\[ % :46:\n"
        "\\begin{eqnarray}\n"
        "\\text{Real-part} (z_1 + z_2) \t    &=& \\text{Real-part} (z_1) + \\\\\n"
        "                                    & & \\text{Real-part} (z_2),  \\\\\n"
        "\\text{Imaginary-part} (z_1 + z_2)   &=& \\text{Imaginary-part} (z_1) + \\\\\n"
        "                                    & & \\text{Imaginary-part} (z_2).\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "\\hbox{Real-part} (z_1 + z_2) \t    &= \\hbox{Real-part} (z_1) + \\cr\n"
        "                                    & \\hbox{Real-part} (z_2),  \\cr\n"
        "\\hbox{Imaginary-part} (z_1 + z_2)   &= \\hbox{Imaginary-part} (z_1) + \\cr\n"
        "                                    & \\hbox{Imaginary-part} (z_2).\n"
        "}"
    ),
    (
        "\\[ % :47:\n"
        "\\begin{eqnarray}\n"
        "  \\text{Magnitude} (z_1 \\cdot z_2)  &=& \n"
        "    \\text{Magnitude} (z_1) \\cdot \\text{Magnitude} (z_2), \\\\\n"
        "  \\text{Angle} (z_1 \\cdot z_2)      &=& \n"
        "    \\text{Angle} (z_1) + \\text{Angle} (z_2).\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  \\hbox{Magnitude} (z_1 \\cdot z_2)  &= \n"
        "    \\hbox{Magnitude} (z_1) \\cdot \\hbox{Magnitude} (z_2), \\cr\n"
        "  \\hbox{Angle} (z_1 \\cdot z_2)      &= \n"
        "    \\hbox{Angle} (z_1) + \\hbox{Angle} (z_2).\n"
        "}"
    ),
    (
        "\\[ % :48:\n"
        "\\begin{eqnarray}\n"
        "  x &=& r \\cos A, \\\\\n"
        "  y &=& r \\sin A, \\\\\n"
        "  r &=& \\sqrt{x^2 + y^2,} \\\\\n"
        "  A &=& \\arctan(y, x),\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  x &= r \\cos A, \\cr\n"
        "  y &= r \\sin A, \\cr\n"
        "  r &= \\sqrt{x^2 + y^2,} \\cr\n"
        "  A &= \\arctan(y, x),\n"
        "}"
    ),
    (
        "\\[ % :60:\n"
        "\\begin{array}{rl}\n"
        "  P_1:  &   x^2 - 2x + 1, \\\\\n"
        "  P_2:  &   11x^2 + 7,    \\\\\n"
        "  P_3:  &   13x + 5.\n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        "  P_1:  &   x^2 - 2x + 1, \\cr\n"
        "  P_2:  &   11x^2 + 7,    \\cr\n"
        "  P_3:  &   13x + 5.\n"
        "}"
    ),
    (
        "\\[ % :63:\n"
        "\\begin{eqnarray}\n"
        "  e^x \t  &=& 1 + x + \\frac{1}{2} x^2  + \\frac{1}{3 \\cdot 2} x^3 "
        " + \\frac{1}{4 \\cdot 3 \\cdot 2} x^4  + \\dots, \\\\\n"
        "  \\cos x  &=& 1 - \\frac{1}{2} x^2 "
        " + \\frac{1}{4 \\cdot 3 \\cdot 2} x^4  - \\dots, \\\\\n"
        "  \\sin x  &=& x - \\frac{1}{3 \\cdot 2} x^3 "
        " + \\frac{1}{5 \\cdot 4 \\cdot 3 \\cdot 2} x^5  - \\dots\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  e^x \t  &= 1 + x + {1 \\over 2} x^2  + {1 \\over 3 \\cdot 2} x^3 "
        " + {1 \\over 4 \\cdot 3 \\cdot 2} x^4  + \\dots, \\cr\n"
        "  \\cos x  &= 1 - {1 \\over 2} x^2  + {1 \\over 4 \\cdot 3 \\cdot 2} x^4  - \\dots, \\cr\n"
        "  \\sin x  &= x - {1 \\over 3 \\cdot 2} x^3 "
        " + {1 \\over 5 \\cdot 4 \\cdot 3 \\cdot 2} x^5  - \\dots\n"
        "}"
    ),
    (
        "\\[ % :65:\n"
        "\\begin{eqnarray}\n"
        "  S \\cdot X           &=&   1, \\\\\n"
        "  (1 + S_R) \\cdot X   &=&   1, \\\\\n"
        "  X + S_R \\cdot X     &=&   1, \\\\\n"
        "  X                   &=&   1 - S_R \\cdot X.\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  S \\cdot X           &=   1, \\cr\n"
        "  (1 + S_R) \\cdot X   &=   1, \\cr\n"
        "  X + S_R \\cdot X     &=   1, \\cr\n"
        "  X                   &=   1 - S_R \\cdot X.\n"
        "}"
    ),
    (
        "\\[ % :68:\n"
        "\n"
        "\\begin{array}{cccccc}\n"
        " s_{00} \t&   s_{01}  \t&   s_{02}  \t&   s_{03}  \t&   s_{04}  \t&   \\dots  \\\\\n"
        "\t\t&   s_{10}  \t&   s_{11}  \t&   s_{12}  \t&   s_{13}  \t&   \\dots  \\\\\n"
        "\t\t& \t\t&   s_{20}  \t&   s_{21}  \t&   s_{22}  \t&   \\dots  \\\\\n"
        "\t\t& \t\t& \t\t&   \\dots  \t& \t\t&   \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        " s_{00} \t&   s_{01}  \t&   s_{02}  \t&   s_{03}  \t&   s_{04}  \t&   \\dots  \\cr\n"
        "\t\t&   s_{10}  \t&   s_{11}  \t&   s_{12}  \t&   s_{13}  \t&   \\dots  \\cr\n"
        "\t\t& \t\t&   s_{20}  \t&   s_{21}  \t&   s_{22}  \t&   \\dots  \\cr\n"
        "\t\t& \t\t& \t\t&   \\dots  \t& \t\t&   \n"
        "}"
    ),
    (
        "\\[ % :70:\n"
        "\n"
        "\\begin{array}{cccc}\n"
        "\t (S_0, T_0)  &  (S_0, T_1)  &  (S_0, T_2)  &  \\dots  \\\\\n"
        "\t (S_1, T_0)  &  (S_1, T_1)  &  (S_1, T_2)  &  \\dots  \\\\\n"
        "\t (S_2, T_0)  &  (S_2, T_1)  &  (S_2, T_2)  &  \\dots  \\\\\n"
        "\t \\dots  & & & \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        "\t (S_0, T_0)  &  (S_0, T_1)  &  (S_0, T_2)  &  \\dots  \\cr\n"
        "\t (S_1, T_0)  &  (S_1, T_1)  &  (S_1, T_2)  &  \\dots  \\cr\n"
        "\t (S_2, T_0)  &  (S_2, T_1)  &  (S_2, T_2)  &  \\dots  \\cr\n"
        "\t \\dots  & & & \n"
        "}"
    ),
    (
        "\\[ % :71:\n"
        "\n"
        "\\begin{array}{cccc}\n"
        " (S_0, T_0)  \t&  (S_0, T_1)  \t&  (S_0, T_2)  \t&  \\dots  \\\\\n"
        "\t\t&  (S_1, T_1)  \t&  (S_1, T_2)  \t&  \\dots  \\\\\n"
        "\t\t& \t\t&  (S_2, T_2)  \t&  \\dots  \\\\\n"
        "\t\t& \t\t& \t\t&  \\dots \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        " (S_0, T_0)  \t&  (S_0, T_1)  \t&  (S_0, T_2)  \t&  \\dots  \\cr\n"
        "\t\t&  (S_1, T_1)  \t&  (S_1, T_2)  \t&  \\dots  \\cr\n"
        "\t\t& \t\t&  (S_2, T_2)  \t&  \\dots  \\cr\n"
        "\t\t& \t\t& \t\t&  \\dots \n"
        "}"
    ),
    (
        "\\[ % :72:\n"
        "\n"
        "\\begin{array}{c|ccc}\n"
        " (S_0, T_0)  \t&  (S_0, T_1)  \t&  (S_0, T_2)  \t&  \\dots  \\\\\n"
        "\\hline\n"
        "\t\t&  (S_1, T_1)  \t&  (S_1, T_2)  \t&  \\dots  \\\\\n"
        "\t\t& \t\t&  (S_2, T_2)  \t&  \\dots  \\\\\n"
        "\t\t& \t\t& \t\t&  \\dots  \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        " (S_0, T_0)  \t&  (S_0, T_1)  \t&  (S_0, T_2)  \t&  \\dots  \\cr\n"
        "\\noalign{\\hrule}\n"
        "\t\t&  (S_1, T_1)  \t&  (S_1, T_2)  \t&  \\dots  \\cr\n"
        "\t\t& \t\t&  (S_2, T_2)  \t&  \\dots  \\cr\n"
        "\t\t& \t\t& \t\t&  \\dots  \n"
        "}"
    ),
    (
        "\\[ % :75:\n"
        "\\begin{eqnarray}\n"
        "  v_R \t&=&   i_R R, \\\\\n"
        "  v_L \t&=&   L\\,{di_L \\over dt}, \\\\\n"
        "  i_C \t&=&   C\\,{dv_C \\over dt},\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  v_R \t&=   i_R R, \\cr\n"
        "  v_L \t&=   L\\,{di_L \\over dt}, \\cr\n"
        "  i_C \t&=   C\\,{dv_C \\over dt},\n"
        "}"
    ),
    (
        "\\[ % :76:\n"
        "\\begin{eqnarray}\n"
        "  i_R \t&=&   i_L = -i_C, \\\\\n"
        "  v_C \t&=&   v_L + v_R.\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): ("\\eqalign{\n  i_R \t&=   i_L = -i_C, \\cr\n  v_C \t&=   v_L + v_R.\n}"),
    (
        "\\[ % :77:\n"
        "\\begin{eqnarray}\n"
        "  {dv_C \\over dt}   &=&   -{i_L \\over C}\\,, \\\\\n"
        "  {di_L \\over dt}   &=&    {1 \\over L}\\, v_C - {R \\over L}\\, i_L.\n"
        "\\end{eqnarray}\n"
        "\\]\n"
    ): (
        "\\eqalign{\n"
        "  {dv_C \\over dt}   &=   -{i_L \\over C}\\,, \\cr\n"
        "  {di_L \\over dt}   &=    {1 \\over L}\\, v_C - {R \\over L}\\, i_L.\n"
        "}"
    ),
    (
        "\\[ % :82:\n"
        "\n"
        "\\begin{array}{l|l|l}\n"
        "                \t& \\text{Maximum} \t& \\text{Number of} \t\\\\\n"
        "                \t& \\text{depth} \t        & \\text{pushes} \t\\\\\n"
        "\\hline\n"
        "\\text{Recursive} \t&\t                &  \\\\\n"
        "\\text{factorial} \t&  \t                &  \\\\\n"
        "\\hline\n"
        "\\text{Iterative} \t&               \t&  \\\\\n"
        "\\text{factorial} \t&  \t                & \n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        "                \t& \\hbox{Maximum} \t& \\hbox{Number of} \t\\cr\n"
        "                \t& \\hbox{depth} \t        & \\hbox{pushes} \t\\cr\n"
        "\\noalign{\\hrule}\n"
        "\\hbox{Recursive} \t&\t                &  \\cr\n"
        "\\hbox{factorial} \t&  \t                &  \\cr\n"
        "\\noalign{\\hrule}\n"
        "\\hbox{Iterative} \t&               \t&  \\cr\n"
        "\\hbox{factorial} \t&  \t                & \n"
        "}"
    ),
    (
        "\\[ % :83:\n"
        "\n"
        "\\begin{array}{l|l|l|l}\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "\\text{(save}                            &\n"
        "\\text{(save}                            &\n"
        "\\text{(save} \\kern1ex ⟨\\kern0.1em{reg_2}⟩\\text{)}    \\\\\n"
        "\n"
        "⟨\\kern0.1em{seq_2}⟩  \t\t        &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_1}⟩\\text{)}    &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_2}⟩\\text{)}    &\n"
        "\\text{(save} \\kern1ex ⟨\\kern0.1em{reg_1}⟩\\text{)}    \\\\\n"
        "\n"
        "                                        &\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "⟨\\kern0.1em{seq_1}⟩                                  \\\\\n"
        "\n"
        "                                        &\n"
        "\\text{(restore}                         &\n"
        "\\text{(restore}                         &\n"
        "\\text{(restore} \\kern1ex ⟨\\kern0.1em{reg_1}⟩\\text{)} \\\\\n"
        "\n"
        "                                        &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_1}⟩\\text{)}    &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_2}⟩\\text{)}    &\n"
        "\\text{(restore} \\kern1ex ⟨\\kern0.1em{reg_2}⟩\\text{)} \\\\\n"
        "\n"
        "                                        &\n"
        "⟨\\kern0.1em{seq_2}⟩                     &\n"
        "⟨\\kern0.1em{seq_2}⟩                     &\n"
        "⟨\\kern0.1em{seq_2}⟩\n"
        "\\end{array}\n"
        "\\]\n"
    ): (
        "\\matrix{\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "\\hbox{(save}                            &\n"
        "\\hbox{(save}                            &\n"
        "\\hbox{(save} \\kern1ex ⟨\\kern0.1em{reg_2}⟩\\hbox{)}    \\cr\n"
        "⟨\\kern0.1em{seq_2}⟩  \t\t        &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_1}⟩\\hbox{)}    &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_2}⟩\\hbox{)}    &\n"
        "\\hbox{(save} \\kern1ex ⟨\\kern0.1em{reg_1}⟩\\hbox{)}    \\cr\n"
        "                                        &\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "⟨\\kern0.1em{seq_1}⟩                     &\n"
        "⟨\\kern0.1em{seq_1}⟩                                  \\cr\n"
        "                                        &\n"
        "\\hbox{(restore}                         &\n"
        "\\hbox{(restore}                         &\n"
        "\\hbox{(restore} \\kern1ex ⟨\\kern0.1em{reg_1}⟩\\hbox{)} \\cr\n"
        "                                        &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_1}⟩\\hbox{)}    &\n"
        "\\kern1ex ⟨\\kern0.1em{reg_2}⟩\\hbox{)}    &\n"
        "\\hbox{(restore} \\kern1ex ⟨\\kern0.1em{reg_2}⟩\\hbox{)} \\cr\n"
        "                                        &\n"
        "⟨\\kern0.1em{seq_2}⟩                     &\n"
        "⟨\\kern0.1em{seq_2}⟩                     &\n"
        "⟨\\kern0.1em{seq_2}⟩\n"
        "}"
    ),
}


@dataclass(frozen=True, slots=True)
class Substitution:
    """One replacement, addressed in the input of its transformation pass."""

    offset: int
    old: str
    new: str


def replace_matches(
    text: str, pattern: str, transform: Callable[[re.Match[str]], str], edits: list[Substitution]
) -> str:
    """Record replacements in application order so reverse order restores bytes."""
    for found in reversed(list(re.finditer(pattern, text, re.MULTILINE | re.DOTALL))):
        replacement = transform(found)
        edits.append(Substitution(found.start(), found[0], replacement))
        text = text[: found.start()] + replacement + text[found.end() :]
    return text


def braced(text: str, start: int) -> tuple[str, int]:
    """Read a balanced Texinfo argument, including nested commands."""
    depth = 1
    cursor = start
    while cursor < len(text):
        if text[cursor : cursor + 2] in ("@@", "@{", "@}"):
            cursor += 2
            continue
        if text[cursor] == "{":
            depth += 1
        elif text[cursor] == "}":
            depth -= 1
            if depth == 0:
                return text[start:cursor], cursor + 1
        cursor += 1
    raise ValueError("Unclosed Texinfo argument")


def figure(match: re.Match[str]) -> str:
    """Expose the SVG in HTML and the PDF in TeX, with one shared caption."""
    text = match[0]
    image = re.search(r"@image\{([^}\n]+)\}", text)
    caption_start = text.find("@caption{")
    if image is None or caption_start < 0:
        raise ValueError("Figure float lacks an image or caption")
    caption, caption_end = braced(text, caption_start + len("@caption{"))
    path, width, height, _, _ = image[1].split(",")
    size = height or width
    alt = re.sub(r"@strong\{(Figure [^}]+)\}", r"\1", caption)
    alt = " ".join(alt.split()).replace(",", "@comma{}")
    html = f"@ifhtml\n@image{{{path},,{size},{alt},.std.svg}}\n@end ifhtml\n"
    pdf = f"@image{{{path},,{size},,.pdf}}"
    text = text[:caption_start] + text[caption_end:]
    text = text.replace(image[0], pdf, 1)
    text = text.replace("@iftex\n", html + "@iftex\n", 1)
    return text.replace("@end float", f"@caption{{{caption}}}\n@end float", 1)


def display_math(match: re.Match[str]) -> str:
    """Convert a display and move it before its optional Info fallback."""
    fallback = match[1] or ""
    body = match[2]
    if body in REWRITES:
        math = REWRITES[body]
    else:
        if r"\begin{" in body:
            raise ValueError("Display environment has no declared rewrite")
        math = re.sub(r"^\s*\\\[\s*(?:%[^\n]*)?\n?", "", body)
        math = re.sub(r"\\\]\s*$", "", math).strip()
    return f"@displaymath\n{math}\n@end displaymath\n{fallback}"


FRACTION = "\\frac{"


def fraction_span(text: str, start: int) -> tuple[str, str, int] | None:
    r"""Read the two balanced arguments of the `\\frac` beginning at `start`.

    TeX allows whitespace, including a line break, between the two arguments.
    """
    numerator, after = braced(text, start + len(FRACTION))
    while after < len(text) and text[after].isspace():
        after += 1
    if after >= len(text) or text[after] != "{":
        return None
    denominator, end = braced(text, after + 1)
    return numerator, denominator, end


def convert_fractions(text: str) -> str:
    r"""Rewrite every `\\frac{a}{b}`, however nested, into plain TeX `{a \\over b}`."""
    pieces: list[str] = []
    cursor = 0
    while (found := text.find(FRACTION, cursor)) >= 0:
        parsed = fraction_span(text, found)
        if parsed is None:
            pieces.append(text[cursor : found + len(FRACTION)])
            cursor = found + len(FRACTION)
            continue
        numerator, denominator, end = parsed
        pieces.append(text[cursor:found])
        pieces.append(f"{{{convert_fractions(numerator)} \\over {convert_fractions(denominator)}}}")
        cursor = end
    pieces.append(text[cursor:])
    return "".join(pieces)


def outermost_fractions(text: str) -> list[tuple[int, int]]:
    r"""Return the span of each `\\frac` that no other `\\frac` encloses."""
    spans: list[tuple[int, int]] = []
    cursor = 0
    while (found := text.find(FRACTION, cursor)) >= 0:
        parsed = fraction_span(text, found)
        if parsed is None:
            cursor = found + len(FRACTION)
            continue
        spans.append((found, parsed[2]))
        cursor = parsed[2]
    return spans


def replace_fractions(text: str, edits: list[Substitution]) -> str:
    """Convert fractions right to left, recording each outermost span once.

    Recording the outermost span keeps every `old` equal to the original bytes,
    which is what the inverse in `restore` replays.
    """
    for start, end in reversed(outermost_fractions(text)):
        old = text[start:end]
        new = convert_fractions(old)
        edits.append(Substitution(start, old, new))
        text = text[:start] + new + text[end:]
    return text


def transform(text: str, figures_rel: str) -> tuple[str, list[Substitution]]:
    """Apply only the five declared source transformations."""
    edits: list[Substitution] = []
    text = replace_matches(text, r"^@float[^\n]*\n.*?^@end float\n", figure, edits)
    text = replace_matches(
        text,
        r"(^@ifinfo\n(?:(?!^@end ifinfo).)*^@end ifinfo\n\s*)?^@tex\n(.*?)^@end tex\n",
        display_math,
        edits,
    )
    text = replace_matches(
        text,
        r"@image\{fig/",
        lambda _: f"@image{{{figures_rel.rstrip('/')}/",
        edits,
    )
    # Plain TeX defines neither macro; either one aborts the TeX math run, and
    # texi2any then reports success with every equation missing from the output.
    # Both appear only inside math in the source, never in `@example` or `@lisp`.
    text = replace_matches(text, r"\\text\{", lambda _: "\\hbox{", edits)
    return replace_fractions(text, edits), edits


def partition(source: bytes) -> list[tuple[str, str, int, int]]:
    """Return section slices in source order with byte offsets."""
    sections = list(re.finditer(rb"^@node[ \t]+([1-5]\.\d+),[^\n]*\n", source, re.MULTILINE))
    back = re.search(rb"^@node[ \t]+References(?:,|\n)", source, re.MULTILINE)
    if not sections or back is None:
        raise ValueError("Source needs numbered sections and a References node")
    starts = [("front/preface.texi", "front", 0)]
    starts.extend(
        (f"ch{m[1].decode().split('.')[0]}/{m[1].decode()}.texi", m[1].decode(), m.start())
        for m in sections
    )
    starts.append(("back/references.texi", "References", back.start()))
    ends = [start for _, _, start in starts[1:]] + [len(source)]
    return [(name, node, start, end) for (name, node, start), end in zip(starts, ends, strict=True)]


def encode(text: str) -> str:
    """Encode multiline substitution text as one TSV field."""
    return base64.b64encode(text.encode()).decode("ascii")


def split(source: bytes, out: Path, figures_rel: str) -> int:
    """Write parts and the inverse manifest, then verify the original bytes."""
    records: list[list[str]] = []
    for name, node, start, end in partition(source):
        text, edits = transform(source[start:end].decode(), figures_rel)
        path = out / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(text.encode())
        records.append(["part", name, node, str(start), str(end)])
        records.extend(["edit", str(e.offset), encode(e.old), encode(e.new)] for e in edits)
    with (out / "MANIFEST.txt").open("w", encoding="utf-8", newline="") as stream:
        csv.writer(stream, delimiter="\t", lineterminator="\n").writerows(records)
    compare(source, join(out))
    return len(partition(source))


def restore(text: str, edits: list[Substitution]) -> bytes:
    """Reject modified replacement regions instead of masking source damage."""
    for edit in reversed(edits):
        end = edit.offset + len(edit.new)
        if text[edit.offset : end] != edit.new:
            raise ValueError(f"Modified substitution at character offset {edit.offset}")
        text = text[: edit.offset] + edit.old + text[end:]
    return text.encode()


def join(tree: Path) -> bytes:
    """Restore the original source from the manifest and current part files."""
    chunks: list[bytes] = []
    text = ""
    edits: list[Substitution] = []
    active = False
    with (tree / "MANIFEST.txt").open(encoding="utf-8", newline="") as stream:
        for row in csv.reader(stream, delimiter="\t"):
            match row:
                case ["part", name, _, start, end]:
                    if active:
                        chunks.append(restore(text, edits))
                    path = (tree / name).resolve()
                    if not path.is_relative_to(tree.resolve()) or int(end) < int(start):
                        raise ValueError("Invalid manifest part")
                    text = path.read_bytes().decode()
                    edits = []
                    active = True
                case ["edit", offset, old, new] if active:
                    edits.append(
                        Substitution(
                            int(offset),
                            base64.b64decode(old, validate=True).decode(),
                            base64.b64decode(new, validate=True).decode(),
                        )
                    )
                case _:
                    raise ValueError("Invalid manifest record")
    if not active:
        raise ValueError("Empty manifest")
    chunks.append(restore(text, edits))
    return b"".join(chunks)


def compare(source: bytes, restored: bytes) -> None:
    """Report the first differing byte, including a truncated or extended source."""
    if source != restored:
        offset = next(
            (i for i, (a, b) in enumerate(zip(source, restored, strict=False)) if a != b),
            min(len(source), len(restored)),
        )
        raise ValueError(f"Join mismatch at byte offset {offset}")


def main(argv: Sequence[str] | None = None) -> int:
    """Split or verify without changing existing parts in check mode."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--src", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--figures-rel", default="../../text/original/figures")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    try:
        source = args.src.read_bytes()
        if args.check:
            compare(source, join(args.out))
            count = len(partition(source))
        else:
            count = split(source, args.out, args.figures_rel)
        print(f"parts={count} bytes={len(source)} identical=1")
    except (OSError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
