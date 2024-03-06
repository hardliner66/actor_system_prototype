mod value;

use value::Value;

mod actor;
use actor::ActorSystem;

fn main() {
    let mut sys = ActorSystem::new();
    let ping_actor = sys.spawn(|_actor, ctx, sender, msg| match msg {
        Value::U128(u) => {
            if let Some(sender) = sender {
                ctx.send(&sender, Value::U128(u + 1));
            }
        }
        _ => {}
    });
    let pong_actor = sys.spawn(|_actor, ctx, sender, msg| match msg {
        Value::U128(u) => {
            if let Some(sender) = sender {
                ctx.send(&sender, Value::U128(u + 1));
            }
        }
        _ => {}
    });
    sys.send(Some(pong_actor), &ping_actor, Value::U128(1));
    let runtime = std::time::Duration::from_secs(10);
    while sys.run(runtime) {}
    println!("Final State: {:#?}", sys);
}
