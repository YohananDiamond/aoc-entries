mod aoc;
mod math;

use std::collections::HashSet;

fn main() {
    aoc::start("day6.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let mut questions_answered: usize = 0;

    for group in input.split("\n\n").filter(|s| !s.is_empty()) {
        let answers: HashSet<char> = group
            .chars()
            .filter(|&c| matches!(c, 'a'..='z'))
            .collect();

        questions_answered += answers.len();
    }

    Ok(format!("{}", questions_answered))
}

fn part2(input: &str) -> Result<String, String> {
    let mut questions_answered: usize = 0;

    for group in input.split("\n\n").filter(|s| !s.is_empty()) {
        let mut gp_answers: HashSet<char> = ('a'..='z').collect();


        for person in
        group.split("\n").filter(|s| !s.is_empty())
        {
            let p_answers: HashSet<char> = person
                .chars()
                .filter(|&c| matches!(c, 'a'..='z'))
                .collect();

            gp_answers = gp_answers
                .into_iter()
                .filter(|c| p_answers.contains(c))
                .collect();
        }

        // eprintln!("Questions answered: {}", gp_answers.iter().collect::<String>());
        questions_answered += gp_answers.len();
    }

    Ok(format!("{}", questions_answered))
}
