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

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Nincs karórád vagy mivan?")
        .as_secs();
    loop {
        // Set the activity
        drpc.set_activity(|act| {
            act
                // .state("Nehogy elhidd, hogy ez real. Még csak tesztfázis! 😉")
                // .details(state_message)
                .timestamps(|tim| tim.start(now))
                .assets(|ass| ass.large_image("sckk").large_text("SeeMTA v4 - SCKK"))
        })
        .expect("Failed to set activity");

        // Wait 10 seconds before exiting
    }
}
