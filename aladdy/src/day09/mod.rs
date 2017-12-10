use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let line = input.next().unwrap().unwrap();
    let (consumed, stream) = clean(line);

    let mut score = 0;
    let mut depth = 0;
    for c in stream.chars() {
        match c {
            '{' => depth += 1,
            '}' => {
                score += depth;
                depth -= 1;
            },
            _ => panic!("unexpected character in stream"),
        }
    }

    println!("Stream has group score: {}", score);
    println!("Removed {} garbage characters", consumed);
}

fn clean(stream: String) -> (usize, String) {
    let mut out: String = String::with_capacity(stream.len());

    let mut consumed = 0;
    let mut canceled = false;
    let mut garbage = false;
    for c in stream.chars() {
        if canceled {
            canceled = false;
            continue;
        }
        if c == '!' {
            canceled = true;
            continue;
        }
        
        if garbage {
            if c == '>' {
                garbage = false;
            } else {
                consumed += 1;
            }
            continue;
        }

        match c {
            '{' | '}' => out.push(c),
            '<' => garbage = true,
            ',' => {},
            _ => {
                println!("Unexpected character outside garbage: {}", c);
            },
        }
    }

    (consumed, out)
}

