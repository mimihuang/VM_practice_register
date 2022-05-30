use std::process;

mod opcodes;
mod utils;

const REGISTERS_COUNT: usize = 8;

#[derive(Copy, Clone)]
struct Program<'a> {
    code: &'a Vec<u8>,
    pos: usize
}

impl<'a> Program<'a> {
    pub fn new(code: &'a Vec<u8>) -> Program<'a> {
        Program { code: code, pos: 0 }
    }

    pub fn fetch(&mut self) -> u8 {
        self.pos += 1;
        if self.pos > self.code.len() {
            process::exit(0);
        }
        return self.code[self.pos - 1];
    }
}

fn eval(mut program: Program, mut registers: [u64; REGISTERS_COUNT]) {
    loop {
        let operator = program.fetch();
        let operands = (
            program.fetch(),

            vec![
                program.fetch(), program.fetch(),
                program.fetch(), program.fetch(),
                program.fetch(), program.fetch(),
                program.fetch(), program.fetch()
            ]
        );

        match operator {
            opcodes::EXIT => {
                process::exit(0);
            }
            opcodes::LOAD => {
                registers[operands.0 as usize] = utils::concat_bytes(&operands.1);
            }
            opcodes::ADD => {
                registers[operands.0 as usize] = registers[utils::concat_bytes(&operands.1[..4]) as usize]
                    + registers[utils::concat_bytes(&operands.1[4..]) as usize];
            }
            opcodes::SUB => {
                registers[operands.0 as usize] = registers[utils::concat_bytes(&operands.1[..4]) as usize]
                    - registers[utils::concat_bytes(&operands.1[4..]) as usize];
            }
            opcodes::MUL => {
                registers[operands.0 as usize] = registers[utils::concat_bytes(&operands.1[..4]) as usize]
                    * registers[utils::concat_bytes(&operands.1[4..]) as usize];
            }
            opcodes::DIV => {
                registers[operands.0 as usize] = registers[utils::concat_bytes(&operands.1[..4]) as usize]
                    / registers[utils::concat_bytes(&operands.1[4..]) as usize];
            }
            opcodes::GE => {
                if registers[utils::concat_bytes(&operands.1[..4]) as usize]-registers[utils::concat_bytes(&operands.1[4..]) as usize]>0 {
                    registers[operands.0 as usize] = 1 }
                else {
                    registers[operands.0 as usize] = 0
                };
            }
            _ => {
                println!("error: invalid instruction code");
                process::exit(1);
            }
        }

        println!("{:?}", registers);
    }
}

fn main() {
    let registers: [u64; REGISTERS_COUNT] = [0; REGISTERS_COUNT];
    let code = vec![
        opcodes::LOAD,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 
        opcodes::LOAD,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        opcodes::ADD,
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        opcodes::SUB,
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        opcodes::MUL,
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        opcodes::DIV,
            0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        opcodes::GE,
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        opcodes::EXIT,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let program = Program::new(&code);

    eval(program, registers);
}
