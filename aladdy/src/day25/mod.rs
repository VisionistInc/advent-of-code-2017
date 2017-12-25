use std::io::Result;

const CHECK_AFTER: usize = 12134527;

pub fn solve<L: Iterator<Item=Result<String>>>(_input: &mut L) -> () {
    let mut tape = vec![0_usize; 4 * 1024 * 1024]; // 4MB should be enough
    let mut cursor = tape.len()/2; // start in the middle of the tape

    let mut state = States::A;

    for _ in 0..CHECK_AFTER {
        let (new_state, mov, val)= state.transition(tape[cursor]);

        tape[cursor] = val;
        match mov {
            Move::Left => cursor -= 1,
            Move::Right => cursor += 1,
        }
        state = new_state;
    }

    let sum: usize = tape.iter().sum();
    println!("Checksum after {} steps: {}", CHECK_AFTER, sum);
}

enum States {
    A,
    B,
    C,
    D,
    E,
    F,
}

enum Move {
    Left,
    Right,
}

impl States {
    // transition consumes the current state and value at the cursor
    // returns a new state, direction to move, and the value to write
    fn transition(self, val: usize) -> (States, Move, usize) {
        assert!(val == 0 || val == 1);
        use day25::States::*;
        use day25::Move::*;

        match self {
            A => {
                if val == 1 {
                    (C, Left, 0)
                } else {
                    (B, Right, 1)
                }
            },
            B => {
                if val == 1 {
                    (C, Right, 1)
                } else {
                    (A, Left, 1)
                }
            },
            C => {
                if val == 1 {
                    (D, Left, 0)
                } else {
                    (A, Right, 1)
                }
            },
            D => {
                if val == 1 {
                    (C, Left, 1)
                } else {
                    (E, Left, 1)
                }
            },
            E => {
                if val == 1 {
                    (A, Right, 1)
                } else {
                    (F, Right, 1)
                }
            },
            F => {
                if val == 1 {
                    (E, Right, 1)
                } else {
                    (A, Right, 1)
                }
            },
        }
    }
}
