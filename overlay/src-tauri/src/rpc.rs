use discord_rpc_client::{models::ActivityTimestamps, Client, Event};
use std::{
    env, thread,
    time::{self, SystemTime, UNIX_EPOCH},
};

pub fn main() {
    // Get our main status message
    let state_message = "Nincs szolgálatban";

    // Create the client
    let mut drpc = Client::new(1293942657625751563);

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    // or

    drpc.on_event(Event::Ready, |ctx| {
        println!("READY!");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| {
        act.state(state_message).timestamps(|tim| {
            tim.start(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Nincs karórád vagy mivan?")
                    .as_secs(),
            )
        })
    })
    .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    loop {
        thread::sleep(time::Duration::from_secs(10))
    }
}
