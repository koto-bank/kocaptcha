use actix::prelude::*;

#[derive(Default, Copy, Clone, MessageResponse)]
pub struct Stats {
    pub generated_captchas: i64
}

#[derive(Default)]
pub struct StatsActor {
    pub stats: Stats
}

impl Actor for StatsActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct IncGeneratedCaptchas;

#[derive(Message)]
#[rtype(result = "Stats")]
pub struct GetStats;

impl Handler<IncGeneratedCaptchas> for StatsActor {
    type Result = ();

    fn handle(&mut self, _msg: IncGeneratedCaptchas, _ctx: &mut Context<Self>) {
        self.stats.generated_captchas += 1;
    }
}

impl Handler<GetStats> for StatsActor {
    type Result = Stats;

    fn handle(&mut self, _msg: GetStats, _ctx: &mut Context<Self>) -> Self::Result {
        self.stats
    }
}
