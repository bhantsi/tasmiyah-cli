//! tasmiyah-cli — a tiny terminal greeting that prints Bismillah.
//!
//! This is the Step 2 scaffold: a minimal entry point that just prints the
//! Arabic Basmala. We'll grow this into a styled, cross-platform CLI in the
//! following steps.

fn main() {
    // The Basmala — "In the name of Allah, the Most Gracious, the Most Merciful"
    const BISMILLAH: &str = "بِسْمِ ٱللَّٰهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ";

    println!("{}", BISMILLAH);
}
