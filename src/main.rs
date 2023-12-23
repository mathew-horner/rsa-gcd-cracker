use crack::Challenge;

mod crack;
mod math;

fn main() {
    let challenges: Vec<_> = (1..=100).map(Challenge::read).collect();
    let mut solutions = Vec::new();
    for i in 0..challenges.len() {
        for j in i + 1..challenges.len() {
            if let Some(sols) = challenges[i].attempt(&challenges[j]) {
                solutions.push(sols.0);
                solutions.push(sols.1);
            }
        }
    }
    solutions.sort_by_key(|sol| sol.challenge);
    solutions
        .iter()
        .for_each(|sol| println!("{}: {}", sol.challenge, sol.decrypted_message));
}
