#![feature(option_unwrap_none)]

#[macro_use]
extern crate lazy_static;

mod aoc;
use aoc::parse_number;

use regex::Regex;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
    required_len: usize,
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Rule {
    name: Rc<String>,
    ranges: ((u32, u32), (u32, u32)),
}

fn main() {
    aoc::start_with_file("day16_example1.txt", part1, aoc::dummy_part);
    aoc::start_with_file("day16_example2.txt", aoc::dummy_part, part2);
    aoc::start_with_file("day16.txt", part1, part2);
}

fn part1(input_str: &str) -> Result<String, String> {
    let input = Input::new(input_str)?;

    Ok(format!("{}", input.invalid_tickets_sum()))
}

fn part2(input_str: &str) -> Result<String, String> {
    let input = Input::new(input_str)?.with_invalid_tickets_discarded();
    let ordered_fields = input.figure_out_field_meanings();

    Ok(format!(
        "{}",
        ordered_fields
            .iter()
            .enumerate()
            .fold(1usize, |prev, (i, name)| if name.starts_with("departure") {
                prev * input.my_ticket.values[i] as usize
            } else {
                prev
            })
    ))
}

impl Input {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut iter = input.split("\n");

        let mut rules = Vec::new();
        loop {
            match iter.next() {
                None => {
                    return Err(
                        r#"Premature end of input (expected either a rule or the "your ticket header")"#.into(),
                    )
                }
                Some("") => break, // go to the next part - it's probably the header
                Some(possible_rule) => rules.push(Rule::new(possible_rule)?),
            }
        }

        expect_match!(iter.next(), Some("your ticket:"))?;

        let my_ticket_str = expect_match!(iter.next(), Some(_))?.unwrap();
        let my_ticket = Ticket::new(my_ticket_str)?;
        let required_len = my_ticket.values.len();

        expect_match!(iter.next(), Some(""))?;
        expect_match!(iter.next(), Some("nearby tickets:"))?;

        let mut nearby_tickets = Vec::new();
        while let Some(possible_ticket) = iter.next() {
            if !possible_ticket.is_empty() {
                let ticket = Ticket::new(possible_ticket)?;

                if ticket.values.len() != required_len {
                    return Err(format!(
                        "Incompatible length for {:?}: found {}, expected {}",
                        ticket,
                        ticket.values.len(),
                        required_len
                    ));
                }

                nearby_tickets.push(ticket);
            }
        }

        Ok(Self {
            rules,
            my_ticket,
            nearby_tickets,
            required_len,
        })
    }

    pub fn with_invalid_tickets_discarded(mut self) -> Self {
        let Self {
            ref mut nearby_tickets,
            ref rules,
            ..
        } = self;

        nearby_tickets.retain(|ticket| {
            !ticket
                .values
                .iter()
                .cloned()
                .any(|value| !Self::valid_by_any_rule(rules, value))
        });

        self
    }

    pub fn invalid_tickets_sum(&self) -> u32 {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.values.iter().cloned())
            .filter(|&n| !Self::valid_by_any_rule(&self.rules, n))
            .sum()
    }

    pub fn valid_by_any_rule(rules: &[Rule], num: u32) -> bool {
        rules.iter().any(|rule| rule.is_valid(num))
    }

    pub fn figure_out_field_meanings(&self) -> Vec<Rc<String>> {
        let valid_rules_per_field_index: Vec<Vec<&str>> = (0..self.required_len)
            .map(|i| {
                let fields_iter = std::iter::once(&self.my_ticket)
                    .chain(&self.nearby_tickets)
                    .map(|ticket| ticket.values[i]);

                self.rules
                    .iter()
                    .filter(|rule| {
                        // all values of index i are valid with this rule
                        fields_iter.clone().all(|value| rule.is_valid(value))
                    })
                    .map(|rule| rule.name.as_str())
                    .collect()
            })
            .collect();

        // Up next: go through a vector of rules (`remaining_rules`) generated from the rules list and try to figure
        // which field index each rule is supposed to be (by storing them on a map), removing the ones that have found
        // an index already.
        let mut remaining_rules: Vec<&Rule> = self.rules.iter().collect();
        let mut map: HashMap<usize, Rc<String>> = HashMap::new();
        let mut threshold = 1; // the amount of ocurrences a remaining rule needs to be moved to the map (increases with time, until there are no remaining rules)

        while !remaining_rules.is_empty() {
            // we can't iterate directly through the remaining rules because borrow checker (it's right tho), so let's
            // use a classic index counter
            let mut remaining_idx = 0;

            while remaining_idx < remaining_rules.len() {
                // get the name of the current rule being analyzed
                let remaining_name = Rc::clone(&remaining_rules[remaining_idx].name);

                // get the field indexes where this rule name is found
                let indexes_found: Vec<usize> = valid_rules_per_field_index
                    .iter()
                    .enumerate()
                    .filter_map(|(i, rules_at_field)| {
                        if rules_at_field
                            .iter()
                            .any(|&rule_name| rule_name == remaining_name.as_str())
                        {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                // the amount of fields where this rule is at is the amount we need right now
                if indexes_found.len() == threshold {
                    'a: loop {
                        // go through each possible removal index and check if it already has a rule on the map
                        for idx in indexes_found {
                            use std::collections::hash_map::Entry;

                            if let Entry::Vacant(e) = map.entry(idx) {
                                remaining_rules.remove(remaining_idx);
                                e.insert(Rc::clone(&remaining_name));
                                break 'a;
                            }
                        }

                        // a little test guard
                        panic!("Oopsie woopsie");
                    }
                } else {
                    remaining_idx += 1;
                }
            }

            threshold += 1;
        }

        let mut map_collect: Vec<(usize, Rc<String>)> = map.into_iter().collect();
        map_collect.sort_by(|(i, _), (i2, _)| i.partial_cmp(i2).unwrap());
        map_collect.iter().cloned().map(|(_, rule)| rule).collect()
    }
}

impl Ticket {
    pub fn new(ticket_input: &str) -> Result<Self, String> {
        let values = ticket_input
            .split(",")
            .map(parse_number)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { values })
    }
}

impl Rule {
    pub fn new(input: &str) -> Result<Self, String> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        if let Some(caps) = REGEX.captures(input) {
            Ok(Self {
                name: Rc::new(caps[1].to_string()),
                ranges: (
                    (parse_number(&caps[2])?, parse_number(&caps[3])?),
                    (parse_number(&caps[4])?, parse_number(&caps[5])?),
                ),
            })
        } else {
            Err(format!("Invalid rule: {:?}", input))
        }
    }

    pub fn is_valid(&self, value: u32) -> bool {
        let ((r1a, r1b), (r2a, r2b)) = self.ranges;
        (r1a..=r1b).contains(&value) || (r2a..=r2b).contains(&value)
    }
}
