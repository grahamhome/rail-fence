mod tests;

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut blocks = vec![vec![]; self.rails];
        let mut block_indices = (0..self.rails).chain((1..self.rails - 1).rev()).cycle();
        text.chars()
            .for_each(|char| blocks[block_indices.next().unwrap()].push(char));
        blocks.iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decoding is a separate problem")
    }
}
