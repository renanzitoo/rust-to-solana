use sha2::{Sha256, Digest};
use rand::RngCore;
use std::collections::HashMap;

type State = HashMap<String, u64>;

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
    previous_hash: String,
    hash: String,
    nonce: u64,
    transactions: Vec<Transaction>
}

struct Wallet {
    address: String,
    balance: u64,
}

impl Wallet {
    fn new(balance: u64) -> Self {
        Wallet { address: generate_address(), balance }
    }

    fn send(&self, to: String, amount: u64, state: &State) -> Option<Transaction> {
        let sender_balance = state.get(&self.address).copied().unwrap_or(0);

        if sender_balance >= amount {
            Some(Transaction {
                from: self.address.clone(),
                to,
                amount,
            })
        } else {
            None
        }
    }

}

impl Block {
    fn calculate_hash(index: &u64, timestamp: &u64, previous_hash: &String, nonce: &u64, transactions: &Vec<Transaction>) -> String {
        let mut hasher = Sha256::new();
        let tx_data = transactions.iter().map(|tx| format!("{}->{}:{}", tx.from, tx.to, tx.amount)).collect::<Vec<String>>().join(",");
        hasher.update(format!("{}{}{}{}{}", index, timestamp, previous_hash, nonce, tx_data));
        format!("0x{:x}", hasher.finalize())
    }
}

fn generate_address() -> String {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0_u8; 20];
    rng.fill_bytes(&mut random_bytes);
    format!("0x{}", hex::encode(random_bytes))
}

fn mine_block(previous_hash: String, difficulty: usize, index: u64, timestamp: u64, transactions: Vec<Transaction>) -> Block {
    let mut nonce = 0;
    let target_prefix = format!("0x{}", "0".repeat(difficulty));

    loop {
        let hash = Block::calculate_hash(&index, &timestamp, &previous_hash, &nonce, &transactions);

        if hash.starts_with(&target_prefix) {
            println!("Block mined: {}", hash);
            return Block {
                index,
                timestamp,
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
    if block.hash != Block::calculate_hash(&block.index, &block.timestamp, &block.previous_hash, &block.nonce, &block.transactions) {
        return false;
    }
    true
}

fn process_block(block: &Block, state: &mut State) -> bool {
    let mut temp_state = state.clone();
    
    for tx in &block.transactions{
        let sender_balance = temp_state.get(&tx.from).cloned().unwrap_or(0);
        if sender_balance < tx.amount {
            return false; // Invalid transaction
        }
        temp_state.insert(tx.from.clone(), sender_balance - tx.amount);
        let receiver_balance = temp_state.get(&tx.to).cloned().unwrap_or(0);
        temp_state.insert(tx.to.clone(), receiver_balance + tx.amount);
    }
    *state = temp_state;
    true
}

fn main() {
    let difficulty = 2;

    let mut transactions_first_block = Vec::<Transaction>::new();
    let mut transactions_second_block = Vec::<Transaction>::new();

    let wallet1 = Wallet::new(100);
    let wallet2 = Wallet::new(50);

    let mut current_state: State = HashMap::new();
    current_state.insert(wallet1.address.clone(), wallet1.balance);
    current_state.insert(wallet2.address.clone(), wallet2.balance);

    let Some(tx1) = wallet1.send(wallet2.address.clone(), 20, &current_state) else {
        eprintln!("Failed to create tx1: insufficient funds");
        return;
    };
    let Some(tx2) = wallet2.send(wallet1.address.clone(), 10, &current_state) else {
        eprintln!("Failed to create tx2: insufficient funds");
        return;
    };
    transactions_first_block.push(tx1);
    transactions_first_block.push(tx2);


    let genesis_block  = mine_block("0x0".to_string(), difficulty, 0, 1627847260, transactions_first_block);

    let mut blockchain = Vec::<Block>::new();

    if process_block(&genesis_block, &mut current_state) {
        blockchain.push(genesis_block);
    } else {
        eprintln!("Genesis block rejected due to invalid state transition");
        return;
    }

    let Some(tx3) = wallet1.send(wallet2.address.clone(), 30, &current_state) else {
        eprintln!("Failed to create tx3: insufficient funds");
        return;
    };
    let Some(tx4) = wallet2.send(wallet1.address.clone(), 15, &current_state) else {
        eprintln!("Failed to create tx4: insufficient funds");
        return;
    };

    transactions_second_block.push(tx3);
    transactions_second_block.push(tx4);

    let previous_hash = blockchain
        .last()
        .map(|block| block.hash.clone())
        .unwrap_or_else(|| "0x0".to_string());

    let first_block = mine_block(previous_hash, difficulty, 1, 1627847261, transactions_second_block);

    if let Some(previous_block) = blockchain.last() {
        if is_valid_block(&first_block, previous_block) && process_block(&first_block, &mut current_state) {
            blockchain.push(first_block);
        } else {
            eprintln!("First block rejected (invalid hash link or state transition)");
        }
    }

    println!("Blockchain: {:#?}", blockchain);
    println!("Current state: {:#?}", current_state);

}
