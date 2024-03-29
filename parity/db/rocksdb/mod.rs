// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

extern crate kvdb_rocksdb;
extern crate migration_rocksdb;
extern crate ethcore_blockchain;

use std::{io, fs};
use std::sync::Arc;
use std::path::Path;
use blooms_db;
use ethcore_db::NUM_COLUMNS;
use ethcore::client::{ClientConfig, DatabaseCompactionProfile};
use kvdb::KeyValueDB;
use self::ethcore_blockchain::{BlockChainDBHandler, BlockChainDB};
use self::kvdb_rocksdb::{Database, DatabaseConfig};

use cache::CacheConfig;

mod blooms;
mod migration;
mod helpers;

pub use self::migration::migrate;

struct AppDB {
	key_value: Arc<dyn KeyValueDB>,
	blooms: blooms_db::Database,
	trace_blooms: blooms_db::Database,
}

impl BlockChainDB for AppDB {
	fn key_value(&self) -> &Arc<dyn KeyValueDB> {
		&self.key_value
	}

	fn blooms(&self) -> &blooms_db::Database {
		&self.blooms
	}

	fn trace_blooms(&self) -> &blooms_db::Database {
		&self.trace_blooms
	}
}

/// Open a secret store DB using the given secret store data path. The DB path is one level beneath the data path.
#[cfg(feature = "secretstore")]
pub fn open_secretstore_db(data_path: &str) -> Result<Arc<KeyValueDB>, String> {
	use std::path::PathBuf;

	let mut db_path = PathBuf::from(data_path);
	db_path.push("db");
	let db_path = db_path.to_str().ok_or_else(|| "Invalid secretstore path".to_string())?;
	Ok(Arc::new(Database::open_default(&db_path).map_err(|e| format!("Error opening database: {:?}", e))?))
}

/// Create a restoration db handler using the config generated by `client_path` and `client_config`.
pub fn restoration_db_handler(client_path: &Path, client_config: &ClientConfig) -> Box<dyn BlockChainDBHandler> {
	let client_db_config = helpers::client_db_config(client_path, client_config);

	struct RestorationDBHandler {
		config: DatabaseConfig,
	}

	impl BlockChainDBHandler for RestorationDBHandler {
		fn open(&self, db_path: &Path) -> io::Result<Arc<dyn BlockChainDB>> {
			open_database(&db_path.to_string_lossy(), &self.config)
		}
	}

	Box::new(RestorationDBHandler {
		config: client_db_config,
	})
}

/// Open a new main DB.
pub fn open_db(client_path: &str, cache_config: &CacheConfig, compaction: &DatabaseCompactionProfile) -> io::Result<Arc<dyn BlockChainDB>> {
	let path = Path::new(client_path);

	let db_config = DatabaseConfig {
		memory_budget: Some(cache_config.blockchain() as usize * 1024 * 1024),
		compaction: helpers::compaction_profile(&compaction, path),
		.. DatabaseConfig::with_columns(NUM_COLUMNS)
	};

	open_database(client_path, &db_config)
}

pub fn open_database(client_path: &str, config: &DatabaseConfig) -> io::Result<Arc<dyn BlockChainDB>> {
	let path = Path::new(client_path);

	let blooms_path = path.join("blooms");
	let trace_blooms_path = path.join("trace_blooms");
	fs::create_dir_all(&blooms_path)?;
	fs::create_dir_all(&trace_blooms_path)?;

	let db = AppDB {
		key_value: Arc::new(Database::open(&config, client_path)?),
		blooms: blooms_db::Database::open(blooms_path)?,
		trace_blooms: blooms_db::Database::open(trace_blooms_path)?,
	};

	Ok(Arc::new(db))
}
