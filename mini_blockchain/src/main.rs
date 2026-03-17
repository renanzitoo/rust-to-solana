use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, timestamp: u64, data: String, previous_hash: String, nonce: u64) -> Self {
       let hash = Self::calculate_hash(&index, &timestamp, &data, &previous_hash, &nonce);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn calculate_hash(index: &u64, timestamp: &u64, data: &String, previous_hash: &String, nonce: &u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce));
        format!("{:x}", hasher.finalize())
    }
}

fn mine_block(data: String, previous_hash: String, difficulty: usize, index: u64, timestamp: u64) -> Block {
    let mut nonce = 0;
    loop {
        let hash = Block::calculate_hash(&index, &timestamp, &data, &previous_hash, &nonce);

        if hash.starts_with(&"0".repeat(difficulty)) {
            println!("Block mined: {}", hash);

            return Block {
                index,
                timestamp,
                data,
                previous_hash,
                hash,
                nonce,
            };
        }

        nonce += 1;
    }
}

fn is_valid_block(block: &Block, previous_block: &Block) -> bool {
    if block.previous_hash != previous_block.hash {
        return false;
    }
    if block.hash != Block::calculate_hash(&block.index, &block.timestamp, &block.data, &block.previous_hash, &block.nonce) {
        return false;
    }
    true
}

fn main() {
    let difficulty = 4;

    let genesis_block  = mine_block("Genesis_block".to_string(), "0".to_string(), difficulty, 0, 1627847260);
    let first_block = mine_block("First block".to_string(), genesis_block.hash.clone(), difficulty, 1, 1627847261);

    print!("Is block valid? {}", is_valid_block(&first_block, &genesis_block));
}
