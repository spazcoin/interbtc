//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-04 (Y/M/D)
//! HOSTNAME: `sander-dell`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//!
//! SHORT-NAME: `extrinsic`, LONG-NAME: `ExtrinsicBase`, RUNTIME: `interBTC`
//! WARMUPS: `100`, REPEAT: `10`
//! WEIGHT-PATH: `parachain/runtime/testnet-interlay/src/weights/`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1.0`, WEIGHT-ADD: `0`

// Executed Command:
//   target/release/interbtc-parachain
//   benchmark
//   overhead
//   --chain
//   interlay-testnet-bench
//   --execution=wasm
//   --wasm-execution=compiled
//   --warmup
//   100
//   --repeat
//   10
//   --weight-path
//   parachain/runtime/testnet-interlay/src/weights/

use sp_core::parameter_types;
use sp_weights::{constants::WEIGHT_REF_TIME_PER_NANOS, Weight};

parameter_types! {
    /// Time to execute a NO-OP extrinsic, for example `System::remark`.
    /// Calculated by multiplying the *Average* with `1.0` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 104_164, 106_174
    ///   Average:  105_094
    ///   Median:   104_968
    ///   Std-Dev:  582.44
    ///
    /// Percentiles nanoseconds:
    ///   99th: 106_174
    ///   95th: 106_174
    ///   75th: 105_539
    pub const ExtrinsicBaseWeight: Weight =
        Weight::from_ref_time(WEIGHT_REF_TIME_PER_NANOS.saturating_mul(105_094));
}

#[cfg(test)]
mod test_weights {
    use sp_weights::constants;

    /// Checks that the weight exists and is sane.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn sane() {
        let w = super::ExtrinsicBaseWeight::get();

        // At least 10 µs.
        assert!(
            w.ref_time() >= 10u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
            "Weight should be at least 10 µs."
        );
        // At most 1 ms.
        assert!(
            w.ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
            "Weight should be at most 1 ms."
        );
    }
}