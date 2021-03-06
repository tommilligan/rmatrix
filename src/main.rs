#[macro_use]
extern crate chan;
extern crate pancurses;
extern crate rmatrix;
extern crate chan_signal;

use pancurses::*;
use chan_signal::Signal;

use rmatrix::Matrix;
use rmatrix::config::Config;

fn main() {
    // Get command line args
    let mut config = Config::new();

    // Save the terminal state and start up ncurses
    let window = rmatrix::ncurses_init();

    // Register for UNIX signals
    let signal = chan_signal::notify(&[Signal::INT, Signal::WINCH]);

    // Create the board
    let mut matrix = Matrix::new();

    // Main event loop
    loop {
        // Check for SIGINT or SIGWINCH
        chan_select! {
            default => {},
            signal.recv() -> signal => {
                if let Some(signal) = signal {
                    match signal {
                        // Terminate ncurses properly on SIGINT
                        Signal::INT => rmatrix::finish(),
                        // Redraw the screen on SIGWINCH
                        Signal::WINCH => {
                            rmatrix::resize_window();
                            matrix = Matrix::new();
                        },
                        _ => {}
                    }
                }
            },
        }

        // Handle a keypress
        if let Some(keypress) = window.getch() {
            if let Input::Character(c) = keypress {
                config.handle_keypress(c)
            }
        }

        // Updaate and redraw the board
        matrix.arrange(&config);
        matrix.draw(&window, &config);
    }
}
