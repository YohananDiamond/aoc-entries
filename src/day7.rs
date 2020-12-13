mod aoc;
mod math;

use std::collections::HashMap;

type BagRuleMap = HashMap<String, Vec<BagAmount>>;

#[derive(Debug, Clone)]
struct BagAmount {
    amount: usize,
    color: String,
}

fn main() {
    aoc::start("day7_example.txt", part1, part2);
    aoc::start("day7.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let rules = get_bag_rules(input)?;
    let colors = rules.iter().map(|(color, _)| color);

    fn contains_shiny(color: &str, map: &BagRuleMap) -> bool {
        let rules = map.get(color).unwrap();

        for rule in rules {
            if rule.color == "shiny gold" || contains_shiny(&rule.color, map) {
                return true;
            }
        }

        false
    }

    Ok(format!(
        "{}",
        rules
            .iter()
            .map(|(color, _)| color)
            .filter(|color| contains_shiny(color, &rules))
            .count()
    ))
}

fn part2(input: &str) -> Result<String, String> {
    let rules = get_bag_rules(input)?;
    let colors = rules.iter().map(|(color, _)| color);

    fn bags_inside(color: &str, map: &BagRuleMap) -> usize {
        let mut total = 0usize;
        let rules = map.get(color).unwrap();

        for rule in rules {
            total += rule.amount + rule.amount * bags_inside(&rule.color, map);
        }

        total
    }

    Ok(format!("{}", bags_inside("shiny gold", &rules)))
}

fn get_bag_rules(input: &str) -> Result<BagRuleMap, String> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| {
            fn forward_expect<'a>(
                input: Option<&'a str>,
                expectation: Option<&str>,
            ) -> Result<Option<&'a str>, String> {
                if input == expectation {
                    Ok(input)
                } else {
                    Err(format!("Expected {:?}, found {:?}", expectation, input))
                }
            }

            let mut words = line.split(" ");

            let color = format!(
                "{} {}",
                words
                    .next()
                    .ok_or_else(|| format!("Failed to get first color word"))?,
                words
                    .next()
                    .ok_or_else(|| format!("Failed to get second second color word"))?,
            );

            forward_expect(words.next(), Some("bags"))?;
            forward_expect(words.next(), Some("contain"))?;

            match words.clone().next() {
                Some("no") => {
                    words.next().unwrap(); // since we cloned the iterator, this `next()` call will return another Some("no")
                    forward_expect(words.next(), Some("other"))?;
                    forward_expect(words.next(), Some("bags."))?;
                    forward_expect(words.next(), None)?;

                    Ok((color, vec![]))
                }
                Some(_) => {
                    let mut bags = Vec::new();

                    let result = loop {
                        let possible_num = words
                            .next()
                            .ok_or_else(|| format!("Failed to get next number"))?;

                        if let Ok(amount) = possible_num.parse::<usize>() {
                            let color = format!(
                                "{} {}",
                                words
                                    .next()
                                    .ok_or_else(|| format!("Failed to get first color word"))?,
                                words
                                    .next()
                                    .ok_or_else(|| format!("Failed to get second color word"))?,
                            );

                            match words.next() {
                                Some("bag,") | Some("bags,") => {
                                    bags.push(BagAmount { amount, color })
                                }
                                Some("bag.") | Some("bags.") => {
                                    bags.push(BagAmount { amount, color });
                                    break bags;
                                }
                                Some(other) => {
                                    return Err(format!(
                                        "Unknown token {:?} (expected {:?})",
                                        other, r"bag(s?)[,.]",
                                    ))
                                }
                                None => {
                                    return Err(format!(
                                        "Premature end of input (expected {:?})",
                                        r"bag(s?)[,.]",
                                    ))
                                }
                            }
                        } else {
                            return Err(format!(
                                "Invalid input {:?} (expected either {:?} or a positive integer)",
                                possible_num, "no"
                            ));
                        };
                    };

                    Ok((color, result))
                }
                None => {
                    return Err(format!(
                        "Premature end of input (expected either {:?} or a positive integer)",
                        "no"
                    ))
                }
            }
        })
        .collect()
}
