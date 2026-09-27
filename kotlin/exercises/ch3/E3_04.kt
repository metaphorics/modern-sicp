// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

// AccountError and Account are exercise 3.3's public declarations, reused here from the same package.

/**
 * Exercise 3.4: Modify the `makeAccount` function of Exercise 3.3 by
 * adding another local state variable so that, if an account is accessed
 * more than seven consecutive times with an incorrect password, it
 * raises `AccountError.CallTheCops` instead of `WrongPassword`. A correct
 * password resets the consecutive-wrong-password count to zero.
 *
 * The scaffold always raises `CallTheCops`, ignoring the balance and
 * every password given.
 */
public fun makeAccountWithLockout(
    balance: Long,
    correctPassword: String,
): Account = throw PendingSolution()
