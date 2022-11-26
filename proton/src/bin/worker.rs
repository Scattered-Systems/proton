use gloo::worker::Registrable;
use proton::ops::multiply::Multiplier;

fn main() {
    #[cfg(feature = "full")]
    console_error_panic_hook::set_once();

    Multiplier::registrar().register();
}