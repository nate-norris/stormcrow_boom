use std::pin::Pin;
use std::future::Future;
use tokio::time::{sleep, Duration, Sleep};

use super::models::EventRx;
use crate::lib_sensor::EdgeDetection;

pub async fn sensor_consume_task<F, Fut>(mut rx: EventRx, mut on_trigger: F) 
    where F: FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static {

    let mut timer: Option<Pin<Box<Sleep>>> = None;
    let wait = Duration::from_secs(1);

    loop {
        tokio::select! {
            // perform match for EdgeDetection types
            Some(event) = rx.recv() => {
                match event {
                    EdgeDetection::Triggered => {
                        println!("trigger received");
                        timer = Some(Box::pin(sleep(wait)));
                    }
                    EdgeDetection::Error(_msg) => {
                        eprintln!("Sensor error")
                    }
                }
            }

            // verify timer completes a run then complete trigger action
            _ = async {
                match &mut timer {
                    // println!("timer reset");
                    Some(t) => {
                        println!("timer reinitialized");
                        t.as_mut().await;
                    },
                    None => std::future::pending::<()>().await,
                }
            } => {
                println!("timer finalized after 1 sec");
                on_trigger().await;
                timer = None;
            }
        }
    }
}