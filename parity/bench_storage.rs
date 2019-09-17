// use std::slice;
// use std::mem;
// use std::io::{self, Read};
// use std::fs::File;

// use db;
// use ethcore::replayer::*;
// use ethcore::account_state::{Account, State, Backend, CleanupMode};
// use ethcore::state_db::{StateDB};
// use ethcore_db;
// use ethereum_types::*;
// use journaldb::*;
// use rlp::*;

use super::ExecutionAction;

#[derive(Debug, PartialEq)]
pub struct BenchStorageCommand {
    pub tx_file: String,
    pub db_dir: String,
    pub txs_to_process: Option<u32>,
    pub skip_bytes: Option<u32>,
}

// #[repr(u32)]
// #[derive(Clone, Debug)]
// pub enum EthTxType {
//     BlockRewardAndTxFee,
//     UncleReward,
//     Transaction,
//     Dao,
//     GenesisAccount,
// }

// #[derive(Clone, Debug)]
// pub struct RealizedEthTx {
//     // Sender spends fee + amount.
//     // Receiver receives amount.
//     sender: Option<H160>,
//     // None for conotract creation.
//     receiver: Option<H160>,
//     tx_fee_wei: U256,
//     amount_wei: U256,
//     types: EthTxType,
// }

// impl Encodable for RealizedEthTx {
//     fn rlp_append(&self, s: &mut RlpStream) {
//         s.begin_list(5)
//             .append(&self.sender)
//             .append(&self.receiver)
//             .append(&self.tx_fee_wei)
//             .append(&self.amount_wei)
//             .append(&(self.types.clone() as u32));
//     }
// }

// impl Decodable for RealizedEthTx {
//     fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
//         Ok(RealizedEthTx {
//             sender: rlp.val_at(0)?,
//             receiver: rlp.val_at(1)?,
//             tx_fee_wei: rlp.val_at(2)?,
//             amount_wei: rlp.val_at(3)?,
//             types: unsafe { mem::transmute(rlp.val_at::<u32>(4)?) },
//         })
//     }
// }

// impl Replayer {
//     fn new(db_dir: String) -> Replayer {
//         let kvdb = db::open_db(&db_dir, &Default::default(), &Default::default()).unwrap();
//         let journaldb = JournalDB::new(kvdb, Algorithm::EarlyMerge, ethcore_db::COL_STATE);
//         let statedb = StateDB::new(journaldb, 10 * 1024 * 1024);
//         Replayer {
//             state_db: statedb.clone()
//         }
//     }
// }

// impl Replay for Replayer {
//     fn add_tx(tx: RealizedEthTx) {
//     }
// }

pub fn execute(_cmd: BenchStorageCommand) -> Result<ExecutionAction, String> {
    unimplemented!()
}

// pub fn execute(cmd: BenchStorageCommand) -> Result<ExecutionAction, String> {
//     let mut tx_replayer = Replayer::new(cmd.db_dir);

//     let mut rlp_file = File::open(cmd.tx_file).expect("open rlp file");
//     const BUFFER_SIZE: usize = 10000000;
//     let mut buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);

//     let mut num_txs_read = 0;
//     let mut total_bytes_read = 0;
//     'read: loop {
//         let buffer_ptr = buffer.as_mut_ptr();
//         let buffer_rest = unsafe {
//             slice::from_raw_parts_mut(
//                 buffer_ptr.offset(buffer.len() as isize),
//                 buffer.capacity() - buffer.len()
//             )
//         };
//         debug!(
//             "buffer rest len {}, buffer len {}",
//             buffer_rest.len(),
//             buffer.len()
//         );
//         let read_result = rlp_file.read(buffer_rest);
//         match read_result {
//             Ok(bytes_read) => {
//                 if bytes_read == 0 {
//                     info!("eof");
//                     break 'read;
//                 }

//                 unsafe {
//                     buffer.set_len(buffer.len() + bytes_read);
//                 }
//                 if buffer.len() == buffer.capacity() {
//                     buffer.reserve_exact(buffer.capacity());
//                 }

//                 let mut to_parse = buffer.as_slice();
//                 'parse: loop {
//                     let payload_info_result = Rlp::new(to_parse).payload_info();
//                     if payload_info_result.is_err() {
//                         if *payload_info_result.as_ref().unwrap_err()
//                             == DecoderError::RlpIsTooShort
//                         {
//                             let mut buffer_new = Vec::<u8>::with_capacity(BUFFER_SIZE);
//                             buffer_new.extend_from_slice(to_parse);
//                             drop(to_parse);
//                             buffer = buffer_new;
//                             if buffer.len() == buffer.capacity() {
//                                 buffer.reserve_exact(buffer.capacity());
//                             }
//                             continue 'read;
//                         }
//                     }
//                     let payload_info = payload_info_result.unwrap();

//                     let rlp_len = payload_info.total();

//                     num_txs_read += 1;
//                     total_bytes_read += rlp_len;
//                     if cmd.txs_to_process.is_some()
//                         && num_txs_read > cmd.txs_to_process.unwrap()
//                     {
//                         println!("Already read {} transactions. total bytes read = {}. exiting...",
//                                  num_txs_read, total_bytes_read);
//                         break 'read;
//                     }
//                     let tx = RealizedEthTx::decode(&Rlp::new(&to_parse[0..rlp_len]))
//                         .unwrap();
//                     to_parse = &to_parse[rlp_len..];

//                     tx_replayer.add_tx(
//                         tx,
//                         &mut latest_state,
//                         &mut last_state_root
//                     );
//                 }
//             }
//             Err(err) => {
//                 if err.kind() == io::ErrorKind::Interrupted
//                     || err.kind() == io::ErrorKind::WouldBlock
//                 {
//                     continue;
//                 }
//                 eprintln!("{}", err);
//                 return Err(err.to_string())
//             }
//         }
//     }

//     Ok(ExecutionAction::Instant("Done!".to_string()))
// }
