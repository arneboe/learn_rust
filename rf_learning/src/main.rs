extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Open01;
use rand::distributions::Distribution;
use rand::distributions::Normal;

const NUM_ACTIONS: usize = 6;

#[derive(Debug)]
struct State {
    q: [f32; NUM_ACTIONS],   //estimation of the reward function
    n: [usize; NUM_ACTIONS], //number of times each action has been tried
}

impl State {
    fn empty() -> State {
        State {
            q: [0.0; NUM_ACTIONS],
            n: [0; NUM_ACTIONS],
        }
    }
}

/// A simple simulation of a k-arm bandit.
/// return value indicates how much money you win when pulling the i'th arm.
fn bandit(arm: usize) -> f32 {
    //TODO Use Optinals for error case!
    //     or maybe exceptions? are there exceptions in rust?

    let normal2 = Normal::new(2.0, 3.0);
    let normal4 = Normal::new(4.0, 3.0);
    let normal1 = Normal::new(1.0, 3.0);

    match arm {
        0 => normal4.sample(&mut rand::thread_rng()) as f32,
        1 => normal2.sample(&mut rand::thread_rng()) as f32,
        2 => normal1.sample(&mut rand::thread_rng()) as f32,
        3 => 0.0,
        4 => 0.0,
        5 => 0.0,
        _ => {
            println!("ERROR: illegal arm value");
            42.0
        }
    }
}

fn next_action(explore: bool, s: &State) -> usize {
    if explore {
        //exploration means taking a random action
        return thread_rng().gen_range(0, NUM_ACTIONS);
    } else {
        //exploit
        //explotation means taking the greedy action, i.e. the action that promises the best reward according to our current believe
        let mut max_reward = 0.0;
        let mut best_i = 0;
        for i in 0..NUM_ACTIONS - 1 {
            let reward = s.q[i];
            if reward > max_reward {
                max_reward = reward;
                best_i = i; //FIXME bad style?
            }
        }
        return best_i;
    }
}

fn main() {
    let mut s: State = State::empty();

    for i in 1..20000 {
        //decide whether to explore or exploit
        let explore = match rand::thread_rng().gen_range(0, 100) {
            0...10 => true,
            _ => false,
        };

        //decide on an action
        let a = next_action(explore, &s) as usize; //FIXME remvoe as
                                                   // println!("-----");
                                                   // println!("Action: {}", a);

        s.n[a] += 1;
        //try the action
        let reward = bandit(a);
        // println!("Reward: {}", reward);
        // println!("s.q[a] before: {}", s.q[a]);
        // println!("calc: {}", (1.0 / s.n[a] as f32) * (reward - s.q[a]));
        // println!("(1.0 / s.n[a] as f32): {}", (1.0 / s.n[a] as f32));
        // println!("(reward - s.q[a]): {}", (reward - s.q[a]));
        s.q[a] += (1.0 / s.n[a] as f32) * (reward - s.q[a]);
        println!("{:?}", s);
    }
}
