use std::fmt::format;

#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            nonce: 11316,
            data: String::from("Genesis Block"),
            hash: String::from("000076f13e6ccf3bd2aa1eda0e35622e6f409bf9e8b0"),
            previous_hash: String::from("00000000000000000000000000000000000000000000"),
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            Some(last_block) => {
                if self.is_block_valid(&block, &last_block) {
                    self.blocks.push(block);
                } else {
                    println!("Invalid block");
                }
            }
            None => {
                println!("No blocks in the chain");
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Invalid previous hash");
            return false;
        }
        if !new_block.hash.starts_with("0000") {
            println!("Invalid hash");
            return false;
        }
        if new_block.id != latest_block.id + 1 {
            println!("Invalid id");
            return false;
        }
        if sha256::digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            new_block.previous_hash,
            new_block.data,
            new_block.timestamp,
            new_block.nonce
        )) != new_block.hash
        {
            println!("Invalid hash");
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("No blocks in the chain"),
            1 => println!("Just the genesis block in the chain"),
            _ => {
                for i in 1..chain.len() {
                    if !self.is_block_valid(&chain[i], &chain[i - 1]) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Block {
    fn new(id: u64, data: String, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let (nonce, hash) = Self::mine_block(id, &previous_hash, &data, timestamp);
        Self {
            id,
            nonce,
            data,
            hash,
            previous_hash,
            timestamp,
        }
    }

    fn mine_block(id: u64, previous_hash: &str, data: &str, timestamp: i64) -> (u64, String) {
        println!("Mining block {}", id);
        let mut nonce = 1;

        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let hash = sha256::digest(block_string);
            if &hash[..4] == "0000" {
                println!("Block mined: nonce = {}, hash = {}", nonce, &hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut new_BC = Blockchain::new();
    new_BC.starting_block();

    println!("{:?}", new_BC);

    let new_block = Block::new(
        2,
        String::from("This is a new block"),
        new_BC.blocks[0].hash.to_owned(),
    );

    println!("{:?}", new_block);

    new_BC.try_add_block(new_block);

    println!("Chain valid: {:?}", new_BC.is_chain_valid(&new_BC.blocks));
}
