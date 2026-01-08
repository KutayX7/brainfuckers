pub struct BFState {
    code: Vec<u8>, // The brainfuck code
    ptape: Vec<u8>, // Vector of memory cells (positive direction, including 0)
    ntape: Vec<u8>, // Vector of memory cells (negative direction)
    instruction_position: usize, // Index of the current instruction
    cursor_position: isize, // Index of the current memory cell
    loops: bool, // Whether the memory tape loops around or expands
    output: Vec<u8>, // Used to buffer characters before printing (for UTF-8 Unicode)
    newline_0: bool, // Newline character will be converted into null (0) in the input
}

/*
 * WARNING:
 * This library was meant to be used with the default options.
 * Any modification may result in undefined behaviour.
 *
 * This is *mostly* turing-complete by default
 * (due to unrestricted tape length in both directions).
 * You're limited by your compiler, OS, architecture, and available memory.
 */

const NEWLINE:                     u8 = 10;
const BF_OPCODE_BLOCK_BEGIN:       u8 = 91;
const BF_OPCODE_BLOCK_END:         u8 = 93;
const BF_OPCODE_DECREMENT_VALUE:   u8 = 45;
const BF_OPCODE_INCREMENT_VALUE:   u8 = 43;
const BF_OPCODE_INPUT:             u8 = 44;
const BF_OPCODE_PRINT:             u8 = 46;
const BF_OPCODE_SHIFT_LEFT:        u8 = 60;
const BF_OPCODE_SHIFT_RIGHT:       u8 = 62;

pub fn new_bf_state(code: &str) -> BFState {
    return BFState {
        code: code.as_bytes().to_vec(),
        ptape: vec![0; 3000],
        ntape: Vec::new(),
        instruction_position: 0,
        cursor_position: 0,
        loops: false,
        output: Vec::new(),
        newline_0: false
    };
}

pub fn step_bf(state: &mut BFState) -> bool {
    if state.instruction_position >= state.code.len() {
        return false;
    }

    let opcode = state.code[state.instruction_position];
    let current_value = get_value_at(state, state.cursor_position);

    match opcode {
        BF_OPCODE_INCREMENT_VALUE => {
            set_value_at(state, state.cursor_position, wrapping_increment(current_value));
            state.instruction_position += 1;
        },
        BF_OPCODE_DECREMENT_VALUE => {
            set_value_at(state, state.cursor_position, wrapping_decrement(current_value));
            state.instruction_position += 1;
        },
        BF_OPCODE_SHIFT_LEFT => {
            if state.loops && (state.cursor_position <= 0) {
                state.cursor_position = (state.ptape.len() - 1).try_into().unwrap();
            }
            else {
                state.cursor_position -= 1;
            }
            state.instruction_position += 1;
        },
        BF_OPCODE_SHIFT_RIGHT => {
            if state.loops && (state.cursor_position >= state.ptape.len().try_into().unwrap()) {
                state.cursor_position = 0;
            }
            else {
                state.cursor_position += 1;
            }
            state.instruction_position += 1;
        },
        BF_OPCODE_PRINT => {
            print_char(state);
            state.instruction_position += 1;
        },
        BF_OPCODE_INPUT => {
            read_char_from_stdin(state);
            state.instruction_position += 1;
        },
        BF_OPCODE_BLOCK_BEGIN => {
            if get_value_at(state, state.cursor_position) == 0 {
                let mut depth = 0;
                for i in state.instruction_position..state.code.len() {
                    match state.code[i] {
                        BF_OPCODE_BLOCK_BEGIN => depth += 1,
                        BF_OPCODE_BLOCK_END => {
                            depth -= 1;
                            if depth == 0 { state.instruction_position = i; break; };
                        },
                        _ => {}
                    }
                }
            }
            state.instruction_position += 1;
        },
        BF_OPCODE_BLOCK_END => {
            if get_value_at(state, state.cursor_position) != 0 {
                let mut depth = 0;
                for i in (0..=state.instruction_position).rev() {
                    match state.code[i] {
                        BF_OPCODE_BLOCK_END => depth += 1,
                        BF_OPCODE_BLOCK_BEGIN => {
                            depth -= 1;
                            if depth == 0 { state.instruction_position = i; break; };
                        },
                        _ => {}
                    }
                }
            }
            state.instruction_position += 1;
        },
        _ => {
            state.instruction_position += 1;
        }
    }

    return true;
}

fn get_value_at(state: &BFState, mut index: isize) -> u8 {
    let ptape_len: isize = (state.ptape.len()).try_into().unwrap();

    if ptape_len == 0 {
        panic!("Memory tape length is 0. This is an invalid state.")
    }

    if state.loops {
        if index < 0 {
            index = ptape_len - 1;
        }
        else if index >= ptape_len {
            index = 0;
        }
    }

    if index >= 0 {
        let index: usize = index.try_into().unwrap();
        return *state.ptape.get(index).unwrap_or(&0);
    }
    let index: usize = ((-1) - index).try_into().unwrap();
    return *state.ntape.get(index).unwrap_or(&0);
}

fn set_value_at(state: &mut BFState, index: isize, value: u8) {
    /*
     * WARNING: This won't check for the tape type!
     * In the case of a looping tape, index must be in the range;
     * otherwise the tape will be expanded.
     */

    {
        let ptape_len: isize = (state.ptape.len()).try_into().unwrap();
        let ntape_len: isize = (state.ntape.len()).try_into().unwrap();

        if index >= ptape_len {
            state.ptape.resize((index + 1).try_into().unwrap(), 0);
        }
        if -index > ntape_len {
            state.ntape.resize((-index).try_into().unwrap(), 0);
        }
    }

    if index >= 0 {
        let index: usize = index.try_into().unwrap();
        state.ptape[index] = value;
    }
    else {
        let index: usize = ((-1) - index).try_into().unwrap();
        state.ntape[index] = value;
    }
}

fn read_char_from_stdin(state: &mut BFState) {
    let mut buff = vec![0];
    match std::io::Read::read_exact(&mut std::io::stdin(), &mut buff) {
        Ok(()) => {
            let c: u8 = buff[0];
            let c = if c == NEWLINE && state.newline_0 { 0 } else { c };
            let cursor = state.cursor_position;
            set_value_at(state, cursor, c);
        },
        Err(_) => {
            let cursor = state.cursor_position;
            set_value_at(state, cursor, 0);
        }
    }
}

fn print_char(state: &mut BFState) {
    let value = get_value_at(state, state.cursor_position);
    state.output.push(value);

    match String::from_utf8(state.output.clone()) {
        Ok(s) => {
            print!("{}", s);
            state.output.clear();
        },
        Err(_) => {}
    }
}

fn wrapping_increment(x: u8) -> u8 {
    if x < 255 { return x + 1 };
    return 0
}

fn wrapping_decrement(x: u8) -> u8 {
    if x > 0 { return x - 1 };
    return 255
}
