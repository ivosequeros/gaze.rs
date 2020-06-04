mod gaze;
mod actor;

use futures::future::{join_all};

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(100);
    /* Run actors: */
    for _ in 1..100 {
        handles.push(tokio::spawn(actor::run()));
    }

    join_all(handles).await;

    ()
}