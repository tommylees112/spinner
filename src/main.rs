use spinners::{Spinner, Spinners};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create an atomic boolean wrapped in an Arc to manage the spinner's running state.
    // Arc (Atomic Reference Counting) allows safe sharing of the boolean across threads.
    let running = Arc::new(AtomicBool::new(true));

    // Clone the Arc to share the same atomic boolean with the spinner thread.
    let spinner_running = Arc::clone(&running);

    // Initialize a spinner with a "Processing..." message.
    // The spinner will display a rotating dots animation in the terminal.
    let mut sp = Spinner::new(Spinners::Dots, "Processing...".into());

    // Spawn a new thread to run the spinner animation.
    // The 'move' keyword transfers ownership of 'spinner_running' and 'sp' into the closure.
    let spinner_thread = thread::spawn(move || {
        // Continuously check if the spinner should keep running.
        // 'load' reads the current value of the atomic boolean with sequential consistency.
        while spinner_running.load(Ordering::SeqCst) {
            // Sleep for 100 milliseconds to control the spinner's update rate.
            thread::sleep(Duration::from_millis(100));
        }
        // Stop the spinner and update its message to "Stopped!" once the loop exits.
        sp.stop_with_message("Stopped!".into());
    });

    // Wait for the spinner thread to complete before exiting the main function.
    // 'join' ensures that the main thread waits for the spinner thread to finish.
    // If the spinner thread panics, 'expect' will print an error message.
    spinner_thread.join().expect("Spinner thread panicked");
}
