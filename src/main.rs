use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;
#[macro_use] extern crate rocket;

struct HitCount {
    count: AtomicUsize
}

#[get("/")]
fn index(hit_count: &State<HitCount>) -> &'static str {
    hit_count.count.store(hit_count.count.load(Ordering::Relaxed)+1, Ordering::Relaxed);
    "Hello, world!"
}
#[get("/metrics")]
fn metrics(hit_count: &State<HitCount>) -> String{
    format!("# TYPE num_requests counter\nnum_requests{{method=\"post\"}} {}",hit_count.count.load(Ordering::Relaxed))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,metrics]).manage(HitCount { count: AtomicUsize::new(0) })
}