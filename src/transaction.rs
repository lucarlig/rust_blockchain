use super::*;
use std::collections::HashSet;

pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&self.value.to_le_bytes());

        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|input| input.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );

        bytes
    }
}
