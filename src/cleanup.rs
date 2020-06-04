use std::time::{SystemTime, Duration};
use std::collections::HashMap;

use actix::prelude::*;

#[derive(Default)]
pub struct CleanupActor {
    entries: HashMap<String, SystemTime>
}

impl Actor for CleanupActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let self_addr = ctx.address();

        self_addr.do_send(RunCleanup);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Remove the captcha directory if there are still any there
        let _ = std::fs::remove_dir_all("./captchas");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddEntry(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct RunCleanup;

impl Handler<AddEntry> for CleanupActor {
    type Result = ();

    fn handle(&mut self, msg: AddEntry, _ctx: &mut Context<Self>) {
        self.entries.insert(msg.0, SystemTime::now());
    }
}

static KEEP_SECS: u64 = 5;

impl Handler<RunCleanup> for CleanupActor {
    type Result = ();

    fn handle(&mut self, _msg: RunCleanup, ctx: &mut Context<Self>) {
        let now = SystemTime::now();

        self.entries.retain(|name, time| {
            let from_now = now.duration_since(*time).unwrap();

            if from_now.as_secs() >= KEEP_SECS {
                // Remove the file
                let file_path = format!("./captchas/{}.png", &name);
                let _ = std::fs::remove_file(file_path);

                false
            } else {
                true
            }
        });

        ctx.wait(actix::clock::delay_for(Duration::new(KEEP_SECS, 0)).into_actor(self));

        ctx.address().do_send(RunCleanup);
    }
}
