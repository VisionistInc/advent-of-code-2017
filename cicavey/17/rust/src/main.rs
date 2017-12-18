fn main() {
    let step_size: usize = 349;
    let mut buf = vec![0];
    let mut pos: usize = 0;

    for i in 1..2018 {
        pos += step_size;
        pos %= buf.len();
        pos += 1;
        buf.insert(pos, i);
    }

    println!("{}", buf[pos+1]);

    // repeat, but don't actual insert anything
    let mut buf_len = 1;
    let mut sec_value = 0;
    for i in 1..50_000_000 {
        pos += step_size;
        pos %= buf_len;
        pos += 1;
        buf_len += 1;

        // record value at 1
        if pos == 1 {
            sec_value = i;
        }
    }

    println!("{}", sec_value);
}