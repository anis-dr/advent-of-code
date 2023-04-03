use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input file");

    let range_pairs = parse_input(&input);

    let fully_contain_count = range_pairs
        .iter()
        .filter(|range_pair| range_pair.fully_overlap())
        .count();

    println!("{} Fully contain", fully_contain_count);

    let partially_contain_count = range_pairs
        .iter()
        .filter(|range_pair| range_pair.partially_contain())
        .count();

    println!("{} Partially contain", partially_contain_count);
}

#[derive(Debug)]
struct ElvesJobPair {
    first_elf: (i32, i32),
    second_elf: (i32, i32),
}

impl ElvesJobPair {
    fn fully_overlap(&self) -> bool {
        let first_elf = self.first_elf;
        let second_elf = self.second_elf;

        let first_elf_range = first_elf.0..=first_elf.1;
        let second_elf_range = second_elf.0..=second_elf.1;

        (first_elf_range.contains(&second_elf.0) && first_elf_range.contains(&second_elf.1))
            || (second_elf_range.contains(&first_elf.0) && second_elf_range.contains(&first_elf.1))
    }

    fn partially_contain(&self) -> bool {
        let first_elf = self.first_elf;
        let second_elf = self.second_elf;

        let first_elf_range = first_elf.0..=first_elf.1;
        let second_elf_range = second_elf.0..=second_elf.1;

        (first_elf_range.contains(&second_elf.0) || first_elf_range.contains(&second_elf.1))
            || (second_elf_range.contains(&first_elf.0) || second_elf_range.contains(&first_elf.1))
    }
}

fn parse_input(input: &str) -> Vec<ElvesJobPair> {
    input
        .lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(',').collect();
            let first_elf_jobs_range: Vec<i32> =
                pair[0].split('-').map(|n| n.parse().unwrap()).collect();
            let second_elf_job_range: Vec<i32> =
                pair[1].split('-').map(|n| n.parse().unwrap()).collect();

            ElvesJobPair {
                first_elf: (first_elf_jobs_range[0], first_elf_jobs_range[1]),
                second_elf: (second_elf_job_range[0], second_elf_job_range[1]),
            }
        })
        .collect()
}
