use std::fs::read_to_string;

struct LimitedQueue {
    inner: Vec<char>,
    limit: usize,
}

impl LimitedQueue {
    fn new(limit: usize) -> Self {
        LimitedQueue {
            inner: Vec::with_capacity(limit),
            limit,
        }
    }

    fn is_full(&self) -> bool {
        self.inner.len() == self.limit
    }

    fn enqueue(&mut self, item: char) {
        if self.is_full() {
            self.inner.remove(0);
        }
        self.inner.push(item);
    }

    fn all_different(&self) -> bool {
        let mut chars = self.inner.clone();
        chars.sort();
        chars.dedup();
        chars.len() == self.inner.len()
    }
}

fn main() {
    // Read the input file
    let input = read_to_string("input.txt").unwrap();
    let chars = input.chars();

    // Looking for packet with 4 different characters
    let mut packet_queue = LimitedQueue::new(4);
    // Push the first 3 elements into the queue
    let mut packet_input = chars.clone();
    for _ in 0..3 {
        packet_queue.enqueue(packet_input.next().unwrap());
    }
    // Iterate over the characters
    for (index, c) in packet_input.enumerate() {
        packet_queue.enqueue(c);
        if packet_queue.all_different() {
            println!(
                "Found a match: {:?}, after {} steps",
                packet_queue.inner,
                index + 4
            );
            break;
        }
    }

    // Looking for packet with 14 different characters
    let mut packet_queue = LimitedQueue::new(14);
    // Push the first 13 elements into the queue
    let mut message_input = chars.clone();
    for _ in 0..13 {
        packet_queue.enqueue(message_input.next().unwrap());
    }

    // Iterate over the characters
    for (index, c) in message_input.enumerate() {
        packet_queue.enqueue(c);
        if packet_queue.all_different() {
            println!(
                "Found a match: {:?}, after {} steps",
                packet_queue.inner,
                index + 14
            );
            break;
        }
    }
}
