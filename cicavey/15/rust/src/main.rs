struct Generator {
    value: u32,
    factor: u32,
}

impl Iterator for Generator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.value = ((self.value as u64 * self.factor as u64) % 2147483647 as u64) as u32;
        Some(self.value)
    }
}

struct PickyGenerator {
    value: u32,
    factor: u32,
    multiple: u32
}

impl Iterator for PickyGenerator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        loop {
            self.value = ((self.value as u64 * self.factor as u64) % 2147483647 as u64) as u32;
            if self.value % self.multiple == 0 {
                break;
            }
        };
        Some(self.value)
    }
}

fn main() {
    let gen_a =  Generator { value: 634, factor: 16807 };
    let gen_b =  Generator { value: 301, factor: 48271 };
    let part1 = gen_a.zip(gen_b)
        .take(40_000_000)
        .filter(|p| p.0 & 0xFFFF == p.1 & 0xFFFF)
        .count();
    println!("{}", part1);

    let pgen_a =  PickyGenerator { value: 634, factor: 16807, multiple: 4 };
    let pgen_b =  PickyGenerator { value: 301, factor: 48271, multiple: 8 };

    let part2 = pgen_a.zip(pgen_b)
        .take(5_000_000)
        .filter(|p| p.0 & 0xFFFF == p.1 & 0xFFFF)
        .count();
    println!("{}", part2);
}
