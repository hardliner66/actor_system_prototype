use std::time::{Duration, Instant};
use std::{cell::RefCell, collections::HashMap};

use crate::value::Value;
use uuid::Uuid;

type Sender = Option<ActorRef>;

#[derive(Debug)]
pub struct Actor {
    pub state: HashMap<String, Value>,
    behavior: fn(&mut Actor, ctx: &mut Context, sender: Sender, msg: Value),
    queue: Vec<(Sender, Value)>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct ActorRef(Uuid);

#[derive(Debug)]
pub struct ActorSystem {
    actors: HashMap<ActorRef, RefCell<Actor>>,
    start_time: Instant,
}

pub struct Context {
    queue: Vec<(ActorRef, Value)>,
}

impl Context {
    pub fn send(&mut self, receiver: &ActorRef, msg: Value) {
        self.queue.push((receiver.to_owned(), msg));
    }
}

impl ActorSystem {
    #[must_use]
    pub fn new() -> Self {
        ActorSystem {
            actors: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    #[must_use]
    pub fn spawn(&mut self, behavior: fn(&mut Actor, &mut Context, Sender, Value)) -> ActorRef {
        let actor_ref = ActorRef(Uuid::new_v4());
        self.actors.insert(
            actor_ref.clone(),
            RefCell::new(Actor {
                state: HashMap::new(),
                behavior,
                queue: Vec::new(),
            }),
        );
        actor_ref
    }

    pub fn send(&self, sender: Sender, receiver: &ActorRef, msg: Value) {
        let actor = self.actors.get(receiver).unwrap();
        actor.borrow_mut().queue.push((sender, msg));
    }

    pub fn run(&mut self, runtime: Duration) -> bool {
        for (actor_ref, actor) in self.actors.iter() {
            let mut actor = actor.borrow_mut();
            if let Some((sender, msg)) = actor.queue.pop() {
                let mut ctx = Context { queue: Vec::new() };
                (actor.behavior)(&mut actor, &mut ctx, sender, msg);
                for (rec, msg) in ctx.queue {
                    self.send(Some(actor_ref.to_owned()), &rec, msg);
                }
            }
        }
        let duration = self.start_time.elapsed();
        duration <= runtime
    }
}
