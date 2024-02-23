use std::{collections::HashMap, ops::{Index, IndexMut}, num::Wrapping};

type Register = usize;
type Label = String;

struct State {
    registers: [Wrapping<u64>; 8],
    zero: bool,
    labels: HashMap<String, usize>,
}

impl State {
    fn new() -> State {
        State {
            registers: [Wrapping(0); 8],
            zero: false,
            labels: HashMap::new(),
        }
    }

    fn add_label(&mut self, label: String, index: usize) {
        self.labels.insert(label, index);
    }

    fn with_zero(&mut self, reg: &Register, value: Wrapping<u64>) {
        self[reg] = value;
        self.zero = value.0 == 0;
    }

    fn dump(&self) {
        println!("Zero: {}", self.zero);
        println!("                 unsigned                signed                 hex");
        for i in 0..8 {
            println!("R{i}:  {:width$}  {:width$}  0x{:016X}", self[&i], self[&i].0 as i64, self[&i], width = 20);
        }
    }
}

impl Index<&usize> for State {
    type Output = Wrapping<u64>;

    fn index(&self, index: &usize) -> &Self::Output {
        &self.registers[*index]
    }
}

impl IndexMut<&usize> for State {
    fn index_mut(&mut self, index: &usize) -> &mut Self::Output {
        &mut self.registers[*index]
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Debug,
    Zero {
        reg: Register,
    },
    Mov {
        to: Register,
        from: Register,
    },
    Add {
        to: Register,
        op1: Register,
        op2: Register,
    },
    Sub {
        to: Register,
        op1: Register,
        op2: Register,
    },
    Inc {
        reg: Register,
    },
    Dec {
        reg: Register,
    },
    And {
        to: Register,
        op1: Register,
        op2: Register,
    },
    Or {
        to: Register,
        op1: Register,
        op2: Register,
    },
    Xor {
        to: Register,
        op1: Register,
        op2: Register,
    },
    Not {
        reg: Register,
    },
    Shl {
        reg: Register,
        amount: u64,
    },
    Shr {
        reg: Register,
        amount: u64,
    },
    Jz {
        label: Label,
    },
    Jnz {
        label: Label,
    },
    J {
        label: Label,
    },
}

impl Instruction {
    fn apply(&self, state: &mut State, index: usize) -> usize  {
        match self {
            Instruction::Noop => (),
            Instruction::Debug => {
                println!("{} {}", ansi_term::Color::Yellow.paint("Debug:"), ansi_term::Color::Blue.paint(&format!("line {}", index + 1)));
                state.dump();
                let mut s = String::new();
                report_error_if_none(std::io::stdin().read_line(&mut s).ok(), "IO error. Did you close stdin?");
            }
            Instruction::Zero { reg } => state[reg] = Wrapping(0),
            Instruction::Mov { to, from } => state[to] = state[from],
            Instruction::Add { to, op1, op2 } => state.with_zero(to, state[op1] + state[op2]),
            Instruction::Sub { to, op1, op2 } => state.with_zero(to, state[op1] - state[op2]),
            Instruction::Inc { reg } => state.with_zero(reg, state[reg] + Wrapping(1)),
            Instruction::Dec { reg } => state.with_zero(reg, state[reg] - Wrapping(1)),
            Instruction::And { to, op1, op2 } => state.with_zero(to, state[op1] & state[op2]),
            Instruction::Or { to, op1, op2 } => state.with_zero(to, state[op1] | state[op2]),
            Instruction::Xor { to, op1, op2 } => state.with_zero(to, state[op1] ^ state[op2]),
            Instruction::Not { reg } => state.with_zero(reg, !state[reg]),
            Instruction::Shl { reg, amount } => state[reg] = Wrapping(state[reg].0 << amount),
            Instruction::Shr { reg, amount } => state[reg] = Wrapping(state[reg].0 >> amount),
            Instruction::Jz { label } => if state.zero { return *report_error_if_none(state.labels.get(label), &format!("unknown label `{}` on line {}", label, index + 1)) },
            Instruction::Jnz { label } => if !state.zero { return *report_error_if_none(state.labels.get(label), &format!("unknown label `{}` on line {}", label, index + 1)) },
            Instruction::J { label } => return *report_error_if_none(state.labels.get(label), &format!("unknown label `{}` on line {}", label, index + 1)),
        }
        index + 1
    }

    fn parse<'a>(state: &'a mut State) -> impl FnMut((usize, &str)) -> Instruction + 'a {
        |(index, src)| {
            let lowercase = src.to_lowercase();
            let code = lowercase.split_once("//").map(|(a, _)| a).unwrap_or(&lowercase);
            let code = lowercase.split_once(";").map(|(a, _)| a).unwrap_or(&code);
            let code = lowercase.split_once("#").map(|(a, _)| a).unwrap_or(&code);
            let mut split = code.split_whitespace();
            let first = split.next();
            let operands = split.collect::<String>();
            let mut operands = operands.split(",");
            match first {
                Some(first) => match first.trim() {
                    "zero" => Instruction::Zero { reg: read_reg(&mut operands, index) },
                    "debug" => Instruction::Debug,
                    "mov" => Instruction::Mov { to: read_reg(&mut operands, index), from: read_reg(&mut operands, index) },
                    "add" => Instruction::Add { to: read_reg(&mut operands, index), op1: read_reg(&mut operands, index), op2: read_reg(&mut operands, index) },
                    "sub" => Instruction::Sub { to: read_reg(&mut operands, index), op1: read_reg(&mut operands, index), op2: read_reg(&mut operands, index) },
                    "inc" => Instruction::Inc { reg: read_reg(&mut operands, index) },
                    "dec" => Instruction::Dec { reg: read_reg(&mut operands, index) },
                    "and" => Instruction::And { to: read_reg(&mut operands, index), op1: read_reg(&mut operands, index), op2: read_reg(&mut operands, index) },
                    "or" => Instruction::Or { to: read_reg(&mut operands, index), op1: read_reg(&mut operands, index), op2: read_reg(&mut operands, index) },
                    "xor" => Instruction::Xor { to: read_reg(&mut operands, index), op1: read_reg(&mut operands, index), op2: read_reg(&mut operands, index) },
                    "not" => Instruction::Not { reg: read_reg(&mut operands, index) },
                    "shl" => Instruction::Shl { reg: read_reg(&mut operands, index), amount: read_imm(&mut operands, index) },
                    "shr" => Instruction::Shr { reg: read_reg(&mut operands, index), amount: read_imm(&mut operands, index) },
                    "jz" => Instruction::Jz { label: read_label(&mut operands, index) },
                    "jnz" => Instruction::Jnz { label: read_label(&mut operands, index) },
                    "j" => Instruction::J { label: read_label(&mut operands, index) },
                    label => { state.add_label(report_error_if_none(label.split_once(":"), &format!("garbage instruction `{}`", label)).0.to_string(), index); Instruction::Noop },
                },
                None => Instruction::Noop,
            }
        }
    }
}

fn read_reg<'a>(operands: &mut impl Iterator<Item = &'a str>, index: usize) -> Register {
    match read_reg_(operands) {
        Some(reg) => {
            if reg >= 8 {
                report_error(&format!("r{} does not exist", reg));
            }
            reg
        },
        None => report_error(&format!("garbage following instruction on line {}", index + 1)),
    }
}

fn read_reg_<'a>(i: &mut impl Iterator<Item = &'a str>) -> Option<Register> {
    let n = i.next()?.trim();
    n.starts_with("r").then(|| {
        n[1..].parse().ok()
    }).flatten()
}

fn read_imm<'a>(operands: &mut impl Iterator<Item = &'a str>, index: usize) -> u64 {
    match read_imm_(operands) {
        Some(v) => v,
        None => report_error(&format!("garbage following instruction on line {}", index + 1)),
    }
}

fn read_imm_<'a>(i: &mut impl Iterator<Item = &'a str>) -> Option<u64> {
    let n = i.next()?.trim();
    n.parse().ok()
}

fn read_label<'a>(operands: &mut impl Iterator<Item = &'a str>, index: usize) -> String {
    match read_label_(operands) {
        Some(v) => v,
        None => report_error(&format!("garbage following instruction on line {}", index + 1)),
    }
}

fn read_label_<'a>(i: &mut impl Iterator<Item = &'a str>) -> Option<String> {
    let n = i.next()?.trim();
    Some(n.to_string())
}

fn get_source() -> Option<String> {
    std::fs::read_to_string(std::env::args().nth(1)?).ok()
}

fn interpret_arg(arg: String) -> Result<(usize, u64), String> {
    let (before, after) = arg.split_once('=').ok_or_else(|| arg.clone())?;
    let after = after.parse().or_else(|_| after.parse::<i64>().map(|i| i as u64)).map_err(|_| arg.clone())?;
    if !(before.starts_with("r") || before.starts_with("R")) {
        Err(arg)
    }
    else {
        let reg = before[1..].parse().map_err(|_| arg.clone())?;
        if reg >= 8 {
            report_error(&format!("r{} does not exist", reg));
        }
        Ok((reg, after))
    }
}

fn report_error_if_none<T>(opt: Option<T>, error: &str) -> T {
    match opt {
        Some(v) => v,
        None => report_error(error),
    }
}

fn report_error(error: &str) -> ! {
    eprintln!("{} {}", ansi_term::Color::Red.paint("Error: "), error);
    std::process::exit(1);
}

fn main() {
    let content = match get_source() {
        Some(v) => v,
        None => report_error("No assembly file provided or unable to read file"),
    };
    let mut state = State::new();
    for arg in std::env::args().skip(2).map(interpret_arg) {
        match arg {
            Ok((reg, val)) => state[&reg] = Wrapping(val),
            Err(arg) => report_error(&format!("Unable to parse arg: `{}`", arg)),
        }
    }
    let instruction: Vec<Instruction> = content.lines()
        .enumerate()
        .map(Instruction::parse(&mut state))
        .collect();
    let mut pc = 0;
    while pc != instruction.len() {
        pc = instruction[pc].apply(&mut state, pc);
    }
    println!("{}", ansi_term::Color::Green.paint("Finished:"));
    state.dump();
}
