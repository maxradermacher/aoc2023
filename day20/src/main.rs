use nom;
use std::collections::{HashMap, VecDeque};
use std::fs;

const FILE_PATH: &str = "input.txt";
const ITERATIONS: usize = 1000;

#[derive(Clone, Copy, Debug)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

fn parse(input: &str) -> nom::IResult<&str, (ModuleKind, &str, Vec<&str>)> {
    let (input, (kind, name, _, destinations)) = nom::sequence::tuple((
        nom::branch::alt((
            nom::bytes::complete::tag("&"),
            nom::bytes::complete::tag("%"),
            nom::bytes::complete::tag("broadcaster"),
        )),
        nom::character::complete::alpha0,
        nom::bytes::complete::tag(" -> "),
        nom::multi::separated_list1(
            nom::bytes::complete::tag(", "),
            nom::character::complete::alpha1,
        ),
    ))(input)?;
    let kind = match kind {
        "broadcaster" => ModuleKind::Broadcaster,
        "%" => ModuleKind::FlipFlop,
        "&" => ModuleKind::Conjunction,
        _ => unreachable!(),
    };
    Ok((input, (kind, name, destinations)))
}

trait Module: std::fmt::Debug {
    fn add_input(&mut self, name: &str);
    fn is_default(&self) -> bool;
    fn handle(&mut self, source: &str, pulse: Pulse) -> Option<Pulse>;
}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
}

impl FlipFlopModule {
    fn new() -> Self {
        FlipFlopModule { state: false }
    }
}

impl Module for FlipFlopModule {
    fn add_input(&mut self, _name: &str) {}

    fn is_default(&self) -> bool {
        !self.state
    }

    fn handle(&mut self, _source: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.state = !self.state;
                match self.state {
                    false => Some(Pulse::Low),
                    true => Some(Pulse::High),
                }
            }
        }
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    inputs: HashMap<String, Pulse>,
}

impl ConjunctionModule {
    fn new() -> Self {
        ConjunctionModule { inputs: HashMap::new() }
    }
}

impl Module for ConjunctionModule {
    fn add_input(&mut self, name: &str) {
        self.inputs.insert(name.to_string(), Pulse::Low);
    }

    fn is_default(&self) -> bool {
        self.inputs.iter().all(|(_, &pulse)| matches!(pulse, Pulse::Low))
    }

    fn handle(&mut self, source: &str, pulse: Pulse) -> Option<Pulse> {
        self.inputs.insert(source.to_string(), pulse);
        match self.inputs.iter().all(|(_, &pulse)| matches!(pulse, Pulse::High)) {
            false => Some(Pulse::High),
            true => Some(Pulse::Low),
        }
    }
}

#[derive(Debug)]
struct BroadcasterModule {}

impl Module for BroadcasterModule {
    fn add_input(&mut self, _name: &str) {
    }

    fn is_default(&self) -> bool {
        true
    }

    fn handle(&mut self, _source: &str, _pulse: Pulse) -> Option<Pulse> {
        Some(Pulse::Low)
    }
}

#[derive(Debug)]
struct NullModule {}

impl Module for NullModule {
    fn add_input(&mut self, _name: &str) {
    }

    fn is_default(&self) -> bool {
        true
    }

    fn handle(&mut self, _source: &str, _pulse: Pulse) -> Option<Pulse> {
        None
    }
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let mut module_destinations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut modules: HashMap<&str, Box<dyn Module>> = HashMap::new();
    for line in input.lines() {
        let (_, (kind, name, destinations)) = parse(line).unwrap();
        module_destinations.insert(name, destinations);
        modules.insert(name, match kind {
            ModuleKind::Broadcaster => Box::new(BroadcasterModule {}),
            ModuleKind::FlipFlop => Box::new(FlipFlopModule::new()),
            ModuleKind::Conjunction => Box::new(ConjunctionModule::new()),
        });
    }
    for (&name, destinations) in &module_destinations {
        for &destination in destinations {
            if let Some(module) = modules.get_mut(destination) {
                module.add_input(name);
            }
        }
    }
    let mut counts: Vec<(usize, usize)> = Vec::new();
    for _ in 0..1000 {
        let mut low_count: usize = 0;
        let mut high_count: usize = 0;
        let mut pulses: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        pulses.push_back(("", Pulse::Low, ""));
        while let Some((source, pulse, destination)) = pulses.pop_front() {
            match pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }
            let module = match modules.get_mut(destination) {
                None => continue,
                Some(module) => module,
            };
            match module.handle(source, pulse) {
                None => (),
                Some(pulse) => {
                    let source = destination;
                    for &destination in &module_destinations[source] {
                        pulses.push_back((source, pulse, destination));
                    }
                },
            }
        }
        counts.push((low_count, high_count));
        if modules.values().all(|module| module.is_default()) {
            break;
        }
    }
    let mut low_count_total = 0;
    let mut high_count_total = 0;
    let full_iterations = ITERATIONS / counts.len();
    for &(low_count, high_count) in &counts {
        low_count_total += low_count * full_iterations;
        high_count_total += high_count * full_iterations;
    }
    for &(low_count, high_count) in &counts[..(ITERATIONS % counts.len())] {
        low_count_total += low_count;
        high_count_total += high_count;
    }
    println!("{}", low_count_total * high_count_total);
}
