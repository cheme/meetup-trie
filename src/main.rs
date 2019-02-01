

use trie_db::{Trie, TrieMut, DBValue, Recorder, Query};
use hash_db::Hasher;
use keccak_hasher::KeccakHasher;
use substrate_trie::Error;
use substrate_trie::NodeCodec;

type TrieError = trie_db::TrieError<<KeccakHasher as Hasher>::Out, Error>;
type MemoryDB = memory_db::MemoryDB<KeccakHasher, trie_db::DBValue>;
type HashDB<'a> = hash_db::HashDB<KeccakHasher, trie_db::DBValue> + 'a;
type TrieDB<'a> = trie_db::TrieDB<'a, KeccakHasher, NodeCodec<KeccakHasher>>;
type TrieDBMut<'a> = trie_db::TrieDBMut<'a, KeccakHasher, NodeCodec<KeccakHasher>>;
type Lookup<'a, Q> = trie_db::Lookup<'a, KeccakHasher, NodeCodec<KeccakHasher>, Q>;

fn main() {
  println!("Hello, world!");
}


#[test]
fn test_insert() {
  let input = vec![(&[0x48, 0x19], &[0xfe]), (&[0x13, 0x14], &[0xff])];
  let mut db = MemoryDB::default();
  let mut root = Default::default();
  {
    let mut t = TrieDBMut::new(&mut db, &mut root);
    t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
  }
  assert!(TrieDB::new(&db, &root).is_ok());
}


#[test]
fn test_read_lmdb() {

  let tempdirlmdb  = std::path::Path::new("./all2");
  let mut env = rkv::Rkv::environment_builder();
  env.set_map_size(10485760 * 2);
  let created_arc = rkv::Manager::singleton().write().unwrap()
    .get_or_create(tempdirlmdb, |p|rkv::Rkv::from_env(p,env))
    .unwrap();
  let env = created_arc.read().unwrap();
  let tempdir  = std::path::Path::new("orig");
  let store: rkv::Store = env.open_or_create_default().unwrap();
  let readerlm: rkv::Reader<Vec<u8>>  = env.read().unwrap();
  let mut iterlm = readerlm.iter_start(store.clone()).unwrap();
  let mut nbelt = 0;
  for (k, v) in iterlm {
      nbelt += 1;
  }
  assert_eq!(nbelt, 9320);
}


#[test]
fn test_read_rocksdb() {
  use kvdb::KeyValueDB;
  let tempdir  = std::path::Path::new("orig");
  let config = kvdb_rocksdb::DatabaseConfig::default();
  let mut db = kvdb_rocksdb::Database::open(&config, tempdir.to_str().unwrap()).unwrap();
  let mut db: &mut dyn KeyValueDB = &mut db;
  let mut nbelt = 0;
  for (k, v) in db.iter(None) {
      nbelt += 1;
  }
  assert_eq!(nbelt, 9320);
}
