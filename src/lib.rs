//! A macro for defining `#[cfg]` if/else blocks.
//!
//! This macro is similar to the popular [cfg-if](https://docs.rs/cfg-if/latest/cfg_if/) crate,
//! with minor differences in accepted syntax and allows usage in item or expression context.
//!
//! This macro is also useable in `#[no_std]` environments.
//!
//! # Examples
//!
//! ## Item Context
//!
//! ```
//! cifg::cifg! {
//!     if cfg(debug_assertions) {
//!         fn foo() { /* debug functionality */ }
//!     }
//!     else if cfg(windows) {
//!         fn foo() { /* windows release functionality */ }
//!     }
//!     else {
//!         fn foo() { /* non-windows release functionality */ }
//!     }
//! }
//!
//! # fn main() { foo(); }
//! ```
//!
//! ## Expression Context
//!
//! ```
//! fn foo() -> u32 {
//!     let bar = 100 + cifg::cifg! {
//!         if cfg(debug_assertions) {
//!             // debug functionality
//!             10
//!         }
//!         else if cfg(windows) {
//!             // windows release functionality
//!             11
//!         }
//!         else {
//!             // non-windows release functionality
//!             12
//!         }
//!     };
//!
//!     bar % 3
//! }
//!
//! # fn main() { foo(); }
//! ```

#![no_std]

/// A macro for defining `#[cfg]` if/else blocks; usable in item or expression context.
///
/// This macro accepts input with the following syntax:
/// * starts with an `if` statement
/// * zero or more `else/if` statements
/// * zero or one `else` statement
/// * conditions of `cfg(/* config option */)`
///
/// # Example
///
/// ```
/// cifg::cifg! {
///     if cfg(debug_assertions) {
///         // debug functionality
///     }
///     else if cfg(windows) {
///         // windows release functionality
///     }
///     else {
///         // non-windows release functionality
///     }
/// }
/// ```
///
/// # Expansion Diagram
///
/// ![cifg railroad diagram][diagram]
///
#[cfg_attr(
    not(feature = "docs"),
    doc = "[diagram]: https://docs.rs/cifg/latest/cifg/macro.cifg.html#expansion-diagram"
)]
#[cfg_attr(feature = "docs", cifg_diag_attr::gen_rr_diag("diagram"))]
#[macro_export]
macro_rules! cifg {
    // Main entry; create a cfg'd block for the initial condition.
    // Any remaining tokens are passed further on.
    (
        if cfg($positive:meta) {
            $($tokens:tt)*
        }
        $($remaining:tt)*
    ) => {
        #[cfg($positive)]
        $crate::cifg! {
            @IDENTITY [$($tokens)*]
        }
        $crate::cifg! {
            @AGGREGATE_NEGATED_METAS
            [$positive]
            $($remaining)*
        }
    };

    // No remaining conditions; no output;
    (@AGGREGATE_NEGATED_METAS [$($negative:meta),*]) => {};

    // Aggregate cfg items that must be negated.
    // Done before expanding the remaining tokens, since nested repetitions don't work here.
    (
        @AGGREGATE_NEGATED_METAS
        [$($negative:meta),*]
        $($remaining:tt)*
    ) => {
        $crate::cifg! {
            @PROCESS_REMAINING_TOKENS
            [not(any($($negative),*))]
            [$($negative),*]
            $($remaining)*
        }
    };

    // No remaining conditions; no output;
    (@PROCESS_REMAINING_TOKENS [$not_any:meta] [$($negative:meta),*]) => {};

    // Process else/if; recurse remaining tokens.
    (
        @PROCESS_REMAINING_TOKENS
        [$not_any:meta]
        [$($negative:meta),*]
        else if cfg($positive:meta) {
            $($tokens:tt)*
        }
        $($remaining:tt)*
    ) => {
        #[cfg(all($positive, $not_any))]
        $crate::cifg! {
            @IDENTITY $($tokens)*
        }
        $crate::cifg! {
            @AGGREGATE_NEGATED_METAS
            [$positive, $($negative),*]
            $($remaining)*
        }
    };

    // Process else; end invocation.
    (
        @PROCESS_REMAINING_TOKENS
        [$not_any:meta]
        [$($negative:meta),*]
        else {
            $($tokens:tt)*
        }
    ) => {
        #[cfg($not_any)]
        $crate::cifg! {
            @IDENTITY $($tokens)*
        }
    };

    // Output any tokens given; allows tokens of various meta-types to be given to the macro.
    (@IDENTITY [$($tokens:tt)*]) => { $($tokens)* };
}

#[cfg(test)]
mod tests {
    use super::*;
}
