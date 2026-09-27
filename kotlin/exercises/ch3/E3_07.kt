// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.7

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

// AccountError and Account are exercise 3.3's public declarations, reused here from the same package.

/**
 * Exercise 3.7: Consider the bank account objects created by
 * `makeAccount`, with the password modification described in Exercise
 * 3.3. Suppose that our banking system requires the ability to make
 * joint accounts. Define a function `makeJoint` that accomplishes this.
 * `makeJoint` takes three arguments. The first is a password-protected
 * account. The second must match the password with which the account was
 * created in order for `makeJoint` to proceed. The third argument is a
 * new password. `makeJoint` creates an additional access to the original
 * account under the new password: for example, if `peterAcc` is a bank
 * account with password `"open-sesame"`, then
 * `makeJoint(peterAcc, "open-sesame", "rosebud")` returns an account that
 * makes transactions on `peterAcc` using the password `"rosebud"`.
 *
 * The scaffold always raises `WrongPassword`, ignoring every argument.
 */
public fun makeJoint(
    account: Account,
    originalPassword: String,
    newPassword: String,
): Account = throw PendingSolution()
