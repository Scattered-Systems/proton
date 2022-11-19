use gloo::worker::Registrable;
use proton::multiplier::Multiplier;

fn main() {
    console_error_panic_hook::set_once();

    Multiplier::registrar().register();
}