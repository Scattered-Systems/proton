use gloo::console::log;
use gloo::worker::Spawnable;
use proton::ops::multiply::Multiplier;

fn main() {
    #[cfg(feature = "full")]
    console_error_panic_hook::set_once();

    let bridge = Multiplier::spawner()
        .callback(move |((a, b), result)| {
            log!(format!("{} * {} = {}", a, b, result));
        })
        .spawn("./worker.js");
    let bridge = Box::leak(Box::new(bridge));

    bridge.send((2, 5));
    bridge.send((3, 3));
    bridge.send((50, 5));
}