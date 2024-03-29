#[derive(Debug)]
pub struct Machine {
    pc: usize,
    memory: Vec<i32>,
    pub output: Vec<i32>,
    input: Vec<i32>,
}

#[derive(Debug)]
enum Parameter {
    Position,
    Immediate,
}

#[derive(Debug)]
enum Ops {
    Add(i32, i32),
    Multiply(i32, i32),
    Insert(i32),
    Output(i32),
    JumpTrue(i32, i32),
    JumpFalse(i32, i32),
    LessThan(i32, i32),
    Equals(i32, i32),
    Halt,
}

impl Machine {
    pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Machine {
        Machine {
            pc: 0 as usize,
            memory: memory.to_vec(),
            output: Vec::new(),
            input: input.to_vec(),
        }
    }

    pub fn execute(&mut self) {
        loop {
            let ops = self.get_ops().unwrap();
            match ops {
                Ops::Add(p1, p2) => {
                    let loc = self.memory[self.pc + 3];
                    self.memory[loc as usize] = p1 + p2;
                    self.pc += 4;
                }
                Ops::Multiply(p1, p2) => {
                    let loc = self.memory[self.pc + 3];
                    self.memory[loc as usize] = p1 * p2;
                    self.pc += 4;
                }
                Ops::Insert(p1) => {
                    let loc = self.memory[self.pc + 1];
                    self.memory[loc as usize] = p1;
                    self.pc += 2;
                }
                Ops::Output(p1) => {
                    self.output.push(p1);
                    self.pc += 2;
                }
                Ops::JumpTrue(p1, p2) => {
                    if p1 != 0 {
                        self.pc = p2 as usize
                    } else {
                        self.pc += 3;
                    }
                }
                Ops::JumpFalse(p1, p2) => {
                    if p1 == 0 {
                        self.pc = p2 as usize
                    } else {
                        self.pc += 3;
                    }
                }
                Ops::LessThan(p1, p2) => {
                    let loc = self.memory[self.pc + 3];
                    self.memory[loc as usize] = if p1 < p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Ops::Equals(p1, p2) => {
                    let loc = self.memory[self.pc + 3];
                    self.memory[loc as usize] = if p1 == p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Ops::Halt => break,
            }
        }
    }

    fn get_ops(&mut self) -> Option<Ops> {
        let ops_raw = self.memory[self.pc] % 100;
        let ops = match ops_raw {
            1 => Ops::Add(self.get_param(1), self.get_param(2)),
            2 => Ops::Multiply(self.get_param(1), self.get_param(2)),
            3 => Ops::Insert(self.input.remove(0)),
            4 => Ops::Output(self.get_param(1)),
            5 => Ops::JumpTrue(self.get_param(1), self.get_param(2)),
            6 => Ops::JumpFalse(self.get_param(1), self.get_param(2)),
            7 => Ops::LessThan(self.get_param(1), self.get_param(2)),
            8 => Ops::Equals(self.get_param(1), self.get_param(2)),
            99 => Ops::Halt,
            _ => panic!("incorrect ops type"),
        };
        Some(ops)
    }

    fn get_pmode(&self, nth: usize) -> Parameter {
        let digit = (self.memory[self.pc] as usize / 10_usize.pow(nth as u32 + 1)) % 10;
        return match digit {
            0 => Parameter::Position,
            _ => Parameter::Immediate,
        };
    }

    fn get_param(&self, nth: usize) -> i32 {
        let pmode = self.get_pmode(nth);
        return match pmode {
            Parameter::Position => self.memory[self.memory[self.pc + nth] as usize],
            Parameter::Immediate => self.memory[self.pc + nth],
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = vec![1101, 100, -1, 4, 0];
        let machine = Machine::new(input.clone(), vec![1]);
        assert_eq!(machine.memory, input);
    }

    #[test]
    fn test_execute() {
        let input = vec![1101, 100, -1, 4, 0];
        let mut machine = Machine::new(input, vec![1]);
        machine.execute();
        assert_eq!(machine.memory, vec![1101, 100, -1, 4, 99]);
    }
    #[test]
    fn solution_part1() {
        let input = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 48, 82, 225, 102, 59, 84, 224,
            1001, 224, -944, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223,
            1101, 92, 58, 224, 101, -150, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1,
            224, 223, 223, 1102, 10, 89, 224, 101, -890, 224, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 5, 224, 1, 224, 223, 223, 1101, 29, 16, 225, 101, 23, 110, 224, 1001, 224, -95,
            224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1102, 75, 72, 225,
            1102, 51, 8, 225, 1102, 26, 16, 225, 1102, 8, 49, 225, 1001, 122, 64, 224, 1001, 224,
            -113, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1102, 55, 72,
            225, 1002, 174, 28, 224, 101, -896, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224,
            224, 1, 224, 223, 223, 1102, 57, 32, 225, 2, 113, 117, 224, 101, -1326, 224, 224, 4,
            224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1, 148, 13, 224, 101, -120,
            224, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0,
            0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 8, 677,
            226, 224, 102, 2, 223, 223, 1006, 224, 329, 101, 1, 223, 223, 107, 677, 677, 224, 1002,
            223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223,
            1006, 224, 359, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 374,
            1001, 223, 1, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 389, 101, 1, 223,
            223, 107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 1001, 223, 1, 223, 1107,
            226, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 1001, 223, 1, 223, 108, 677, 677,
            224, 102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 1008, 677, 226, 224, 1002,
            223, 2, 223, 1006, 224, 449, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223,
            1006, 224, 464, 1001, 223, 1, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224,
            479, 1001, 223, 1, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001,
            223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223,
            1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 1107, 677,
            677, 224, 102, 2, 223, 223, 1005, 224, 539, 101, 1, 223, 223, 1107, 677, 226, 224, 102,
            2, 223, 223, 1005, 224, 554, 1001, 223, 1, 223, 108, 677, 226, 224, 1002, 223, 2, 223,
            1006, 224, 569, 1001, 223, 1, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
            584, 101, 1, 223, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 599, 1001, 223,
            1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 7,
            677, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1, 223, 223, 1008, 677, 677,
            224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223,
            2, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006,
            224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ];

        let mut machine = Machine::new(input, vec![1]);
        machine.execute();
        let output = machine.output.last().unwrap();
        assert_eq!(*output, 13547311);
    }

    #[test]
    fn eq_true_positon() {
        let mut machine = Machine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]);
        machine.execute();
        let output = machine.output.last().unwrap();
        assert_eq!(*output, 1);
    }

    #[test]
    fn le_false_positon() {
        let mut machine = Machine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]);
        machine.execute();
        let output = machine.output.last().unwrap();
        assert_eq!(*output, 0);
    }

    #[test]
    fn eq_true_immediate() {
        let mut machine = Machine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8]);
        machine.execute();
        let output = machine.output.last().unwrap();
        assert_eq!(*output, 1);
    }

    #[test]
    fn le_false_immediate() {
        let mut machine = Machine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![8]);
        machine.execute();
        let output = machine.output.last().unwrap();
        assert_eq!(*output, 0);
    }

    #[test]
    fn nonzero_jumptrue_position() {
        let mut machine = Machine::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![1],
        );
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 1);
    }

    #[test]
    fn zero_jumptrue_position() {
        let mut machine = Machine::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
        );
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 0);
    }

    #[test]
    fn nonzero_jumptrue_immediate() {
        let mut machine = Machine::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![1],
        );
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 1);
    }

    #[test]
    fn zero_jumptrue_immediate() {
        let mut machine = Machine::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
        );
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 0);
    }

    #[test]
    fn larger_example() {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let mut machine = Machine::new(input.clone(), vec![0]);
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 999);

        let mut machine = Machine::new(input.clone(), vec![8]);
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 1000);

        let mut machine = Machine::new(input.clone(), vec![16]);
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 1001);
    }

    #[test]
    fn part2_solution() {
        let input = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 48, 82, 225, 102, 59, 84, 224,
            1001, 224, -944, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223,
            1101, 92, 58, 224, 101, -150, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1,
            224, 223, 223, 1102, 10, 89, 224, 101, -890, 224, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 5, 224, 1, 224, 223, 223, 1101, 29, 16, 225, 101, 23, 110, 224, 1001, 224, -95,
            224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1102, 75, 72, 225,
            1102, 51, 8, 225, 1102, 26, 16, 225, 1102, 8, 49, 225, 1001, 122, 64, 224, 1001, 224,
            -113, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1102, 55, 72,
            225, 1002, 174, 28, 224, 101, -896, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224,
            224, 1, 224, 223, 223, 1102, 57, 32, 225, 2, 113, 117, 224, 101, -1326, 224, 224, 4,
            224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1, 148, 13, 224, 101, -120,
            224, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0,
            0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 8, 677,
            226, 224, 102, 2, 223, 223, 1006, 224, 329, 101, 1, 223, 223, 107, 677, 677, 224, 1002,
            223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223,
            1006, 224, 359, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 374,
            1001, 223, 1, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 389, 101, 1, 223,
            223, 107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 1001, 223, 1, 223, 1107,
            226, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 1001, 223, 1, 223, 108, 677, 677,
            224, 102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 1008, 677, 226, 224, 1002,
            223, 2, 223, 1006, 224, 449, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223,
            1006, 224, 464, 1001, 223, 1, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224,
            479, 1001, 223, 1, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001,
            223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223,
            1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 1107, 677,
            677, 224, 102, 2, 223, 223, 1005, 224, 539, 101, 1, 223, 223, 1107, 677, 226, 224, 102,
            2, 223, 223, 1005, 224, 554, 1001, 223, 1, 223, 108, 677, 226, 224, 1002, 223, 2, 223,
            1006, 224, 569, 1001, 223, 1, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224,
            584, 101, 1, 223, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 599, 1001, 223,
            1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 7,
            677, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1, 223, 223, 1008, 677, 677,
            224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223,
            2, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006,
            224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ];

        let mut machine = Machine::new(input, vec![5]);
        machine.execute();
        let output = machine.output.iter().max().unwrap();
        assert_eq!(*output, 236453);
    }
}
