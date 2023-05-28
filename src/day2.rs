use std::fs;

fn evaluate_program(mut program: Vec<usize>)->(usize,usize) {
    let mut position = 0;
    let mut new_value;
    let mut new_value_position;
    let mut terminate_program = false;

    while !terminate_program {

        let opcode = program[position];
        if opcode == 99 {
            terminate_program = true;
            continue;
        }

        new_value_position = program[position + 3];
        if opcode == 1 {
            new_value =  program[ program[position + 1] ] + program[ program[ position + 2] ];
            program[ new_value_position ] = new_value;
            position = position + 4;
        }
        else if opcode == 2 {
            new_value =  program[ program[position + 1] ] * program[ program[ position + 2] ];
            program[ new_value_position ] = new_value;
            position = position + 4;
        }
        else {
            println!("Unknown opcode: {:?}", opcode);
            terminate_program = true;
        }

    }
    (program[0], program[1]*100 + program[2])
}
pub fn day2(file_path: String) {
    let mut numbers: Vec<usize>;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    numbers = contents.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    for noun in 0..100 {
        numbers[1] = noun;
        for verb in 0..100 {
            numbers[2] = verb;
            let (result, answer) = evaluate_program(numbers.clone());
            if result == 19690720 {
                println!("{:?}",(result,answer));
            }
        }
    }
}
