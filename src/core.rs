use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

pub trait Key {
    fn send(&self);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ordering {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
}

pub struct Flask<K> {
    timing: Option<Duration>,
    ordering: Ordering,
    after: Option<Ordering>,
    sleep_interval: (u64, u64),
    key: K,
    last_send: Instant,
}

pub struct Flasks<K>([Flask<K>; 5]);

impl<K: Key> Flask<K> {
    pub fn new(
        timing: Option<Duration>,
        ordering: Ordering,
        after: Option<Ordering>,
        sleep_interval: (u64, u64),
        key: K,
    ) -> Self {
        Self {
            timing: timing,
            ordering: ordering,
            after: after,
            sleep_interval: sleep_interval,
            key: key,
            last_send: Instant::now(),
        }
    }
}

impl<K: Key> Flasks<K> {
    pub fn new(flasks: [Flask<K>; 5]) -> Self {
        Self(flasks)
    }
    fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.0.shuffle(rng);
    }
    pub fn send(&mut self) {
        let mut rng = rand::thread_rng();
        self.shuffle(&mut rng);
        let mut completed: HashSet<Ordering> = HashSet::with_capacity(5);
        let mut now = Instant::now();
        loop {
            self.0.iter_mut().for_each(|flask| {
                if completed.contains(&flask.ordering) {
                    return;
                }
                if let Some(t) = flask.timing {
                    if now.duration_since(flask.last_send) < t {
                        completed.insert(flask.ordering);
                        return;
                    }
                }
                if let Some(after) = flask.after {
                    if !completed.contains(&after) {
                        return;
                    }
                }
                std::thread::sleep(Duration::from_millis(rng.gen_range(
                    flask.sleep_interval.0,
                    flask.sleep_interval.1,
                )));
                flask.key.send();
                now = Instant::now();
                flask.last_send = now;
                completed.insert(flask.ordering);
            });
            if completed.len() == 5 {
                break;
            }
        }
    }
}