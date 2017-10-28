Version 0.1.2
=============

    * Remove unnecessary outer wrapper macro, fixes issue where wrapped_vec!
      macro would fail when imported from extern crate

Version 0.1.1
=============

    * Add all recommended metadata to Cargo.toml
    * Remove example module in lib.rs

Version 0.1.0
=============

    * Proof of concept macro that generates a wrapped Vec type and associated
      `Iter`, `FromIter` & `Extend` impls, plus a small number of helper
      methods