use sha2::{Sha256, Digest};
use rand::RngCore;

#[derive(Debug, Clone)]
struct Transaction{
    from: String,
    to: String,
    amount: u64,
}


#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
    transactions: Vec<Transaction>
}

struct Wallet {
    address: String,
    balance: u64,
}

fn generate_address() -> String {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0_u8; 20];
    rng.fill_bytes(&mut random_bytes);
    format!("0x{}", hex::encode(random_bytes))
}


impl Wallet {
    fn new(balance: u64) -> Self {
        Wallet { address: generate_address(), balance }
    }

    fn send(&mut self, to: String, amount: u64) -> Option<Transaction> {
        if self.balance >= amount {
            Some(Transaction {
                from: self.address.clone(),
                to,
                amount,
            })
        } else {
            None
        }
    }

    fn get_balance(&self, chain: &[Block]) -> u64 {
        let mut balance = self.balance;

        for block in chain {
            for tx in &block.transactions {
                if tx.to == self.address {
                    balance = balance.saturating_add(tx.amount);
                }

                if tx.from == self.address {
                    balance = balance.saturating_sub(tx.amount);
                }
            }
        }

        balance
    }
}

impl Block {
    fn calculate_hash(index: &u64, timestamp: &u64, data: &String, previous_hash: &String, nonce: &u64, transactions: &Vec<Transaction>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce));
        format!("0x{:x}", hasher.finalize())
    }
}

fn mine_block(data: String, previous_hash: String, difficulty: usize, index: u64, timestamp: u64, transactions: Vec<Transaction>) -> Block {
    let mut nonce = 0;
    let target_prefix = format!("0x{}", "0".repeat(difficulty));

    loop {
        let hash = Block::calculate_hash(&index, &timestamp, &data, &previous_hash, &nonce, &transactions);

        if hash.starts_with(&target_prefix) {
            println!("Block mined: {}", hash);
            

            return Block {
                index,
                timestamp,
                data,
                previous_hash,
                hash,
                nonce,
                transactions
            };
        }

        nonce += 1;
    }
}

fn is_valid_block(block: &Block, previous_block: &Block) -> bool {
    if block.previous_hash != previous_block.hash {
        return false;
    }
    if block.hash != Block::calculate_hash(&block.index, &block.timestamp, &block.data, &block.previous_hash, &block.nonce, &block.transactions) {
        return false;
    }
    true
}

fn main() {
    let difficulty = 2;

    let mut transactions = Vec::<Transaction>::new();

    let mut wallet1 = Wallet::new(100);
    let mut wallet2 = Wallet::new(50);

    let tx1 = wallet1.send(wallet2.address.clone(), 20);
    let tx2 = wallet2.send(wallet1.address.clone(), 10);
    transactions.push(tx1.unwrap());
    transactions.push(tx2.unwrap());


    let genesis_block  = mine_block("Genesis_block".to_string(), "0x0".to_string(), difficulty, 0, 1627847260, transactions);
    let first_block = mine_block("First block".to_string(), genesis_block.hash.clone(), difficulty, 1, 1627847261, Vec::new());


    let mut blocks = Vec::<Block>::new();
    blocks.push(genesis_block.clone());
    blocks.push(first_block.clone());

    println!("Wallet1 balance: {}", wallet1.get_balance(&blocks));

    println!("Is block valid? {}", is_valid_block(&first_block, &genesis_block));
    println!("{}", generate_address());

}
