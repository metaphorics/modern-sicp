// SPDX-License-Identifier: GPL-3.0-only
export const fib = (n: number): number => (n < 2 ? n : fib(n - 1) + fib(n - 2));
export const fibStackPushes = (n: number): number => 56 * fib(n + 1) - 40;
export const fibRecurrenceConstant = 40;
export const ex_5_29 = fibStackPushes;
