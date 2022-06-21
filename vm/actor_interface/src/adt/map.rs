// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::ActorVersion;
use cid::Cid;
use fil_actors_runtime_v7;
use fil_types::HAMT_BIT_WIDTH;
use forest_hash_utils::{BytesKey, Hash};
use ipld_blockstore::BlockStore;
use ipld_blockstore::FvmRefStore;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::marker::PhantomData;

pub enum Map<'a, BS, V> {
    // V0(actorv0::Map<'a, BS, V>),
    // V2(actorv2::Map<'a, BS, V>),
    // V3(actorv3::Map<'a, BS, V>),
    // V4(actorv4::Map<'a, BS, V>),
    // V5(actorv5::Map<'a, BS, V>),
    // V6(actorv6::Map<'a, BS, V>),
    V7(fil_actors_runtime_v7::fvm_ipld_hamt::Hamt<FvmRefStore<'a, BS>, V, BytesKey>),
    _UnusedMap(PhantomData<(&'a BS, V)>),
}

impl<'a, BS, V> Map<'a, BS, V>
where
    V: Serialize + DeserializeOwned + PartialEq,
    BS: BlockStore,
{
    pub fn new(store: &'a BS, version: ActorVersion) -> Self {
        match version {
            // ActorVersion::V0 => Map::V0(actorv0::make_map(store)),
            // ActorVersion::V2 => Map::V2(actorv2::make_map(store)),
            // ActorVersion::V3 => Map::V3(actorv3::make_empty_map(store, HAMT_BIT_WIDTH)),
            // ActorVersion::V4 => Map::V4(actorv4::make_empty_map(store, HAMT_BIT_WIDTH)),
            // ActorVersion::V5 => Map::V5(actorv5::make_empty_map(store, HAMT_BIT_WIDTH)),
            // ActorVersion::V6 => Map::V6(actorv6::make_empty_map(store, HAMT_BIT_WIDTH)),
            _ => unimplemented!(),
        }
    }

    /// Load map with root
    pub fn load(cid: &Cid, store: &'a BS, version: ActorVersion) -> Result<Self, Box<dyn Error>> {
        match version {
            // ActorVersion::V0 => Ok(Map::V0(actorv0::make_map_with_root(cid, store)?)),
            // ActorVersion::V2 => Ok(Map::V2(actorv2::make_map_with_root(cid, store)?)),
            // ActorVersion::V3 => Ok(Map::V3(actorv3::make_map_with_root(cid, store)?)),
            // ActorVersion::V4 => Ok(Map::V4(actorv4::make_map_with_root(cid, store)?)),
            // ActorVersion::V5 => Ok(Map::V5(actorv5::make_map_with_root(cid, store)?)),
            // ActorVersion::V6 => Ok(Map::V6(actorv6::make_map_with_root(cid, store)?)),
            ActorVersion::V7 => Ok(Map::V7(
                fil_actors_runtime_v7::fvm_ipld_hamt::Hamt::load_with_bit_width(
                    cid,
                    FvmRefStore::new(store),
                    HAMT_BIT_WIDTH,
                )?,
            )),
            _ => panic!("unsupported actor version: {}", version),
        }
    }

    /// Returns a reference to the underlying store of the `Map`.
    pub fn store(&self) -> &'a BS {
        match self {
            // Map::V0(m) => m.store(),
            // Map::V2(m) => m.store(),
            // Map::V3(m) => m.store(),
            // Map::V4(m) => m.store(),
            // Map::V5(m) => m.store(),
            // Map::V6(m) => m.store(),
            Map::V7(m) => m.store().bs,
            _ => unimplemented!(),
        }
    }

    /// Inserts a key-value pair into the `Map`.
    pub fn set(&mut self, key: BytesKey, value: V) -> Result<(), Box<dyn Error>> {
        match self {
            // Map::V0(m) => Ok(m.set(key, value)?),
            // Map::V2(m) => Ok(m.set(key, value)?),
            // Map::V3(m) => {
            //     m.set(key, value)?;
            //     Ok(())
            // }
            // Map::V4(m) => {
            //     m.set(key, value)?;
            //     Ok(())
            // }
            // Map::V5(m) => {
            //     m.set(key, value)?;
            //     Ok(())
            // }
            // Map::V6(m) => {
            //     m.set(key, value)?;
            //     Ok(())
            // }
            _ => unimplemented!(),
        }
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Result<Option<&V>, Box<dyn Error>>
    where
        BytesKey: Borrow<Q>,
        Q: Hash + Eq,
        V: DeserializeOwned,
    {
        match self {
            // Map::V0(m) => Ok(m.get(k)?),
            // Map::V2(m) => Ok(m.get(k)?),
            // Map::V3(m) => Ok(m.get(k)?),
            // Map::V4(m) => Ok(m.get(k)?),
            // Map::V5(m) => Ok(m.get(k)?),
            // Map::V6(m) => Ok(m.get(k)?),
            Map::V7(m) => Ok(m.get(k)?),
            _ => unimplemented!(),
        }
    }

    /// Returns `true` if a value exists for the given key in the `Map`.
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> Result<bool, Box<dyn Error>>
    where
        BytesKey: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            // Map::V0(m) => Ok(m.contains_key(k)?),
            // Map::V2(m) => Ok(m.contains_key(k)?),
            // Map::V3(m) => Ok(m.contains_key(k)?),
            // Map::V4(m) => Ok(m.contains_key(k)?),
            // Map::V5(m) => Ok(m.contains_key(k)?),
            // Map::V6(m) => Ok(m.contains_key(k)?),
            Map::V7(m) => Ok(m.contains_key(k)?),
            _ => unimplemented!(),
        }
    }

    /// Removes a key from the `Map`, returning the value at the key if the key
    /// was previously in the `Map`.
    pub fn delete<Q: ?Sized>(&mut self, k: &Q) -> Result<Option<(BytesKey, V)>, Box<dyn Error>>
    where
        BytesKey: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            // Map::V0(m) => Ok(m.delete(k)?),
            // Map::V2(m) => Ok(m.delete(k)?),
            // Map::V3(m) => Ok(m.delete(k)?),
            // Map::V4(m) => Ok(m.delete(k)?),
            // Map::V5(m) => Ok(m.delete(k)?),
            // Map::V6(m) => Ok(m.delete(k)?),
            _ => unimplemented!(),
        }
    }

    /// Flush root and return Cid for `Map`
    pub fn flush(&mut self) -> Result<Cid, Box<dyn Error>> {
        match self {
            // Map::V0(m) => Ok(m.flush()?),
            // Map::V2(m) => Ok(m.flush()?),
            // Map::V3(m) => Ok(m.flush()?),
            // Map::V4(m) => Ok(m.flush()?),
            // Map::V5(m) => Ok(m.flush()?),
            // Map::V6(m) => Ok(m.flush()?),
            Map::V7(m) => Ok(m.flush()?),
            _ => unimplemented!(),
        }
    }

    /// Iterates over each KV in the `Map` and runs a function on the values.
    pub fn for_each<F>(&self, mut f: F) -> Result<(), Box<dyn Error>>
    where
        V: DeserializeOwned,
        F: FnMut(&BytesKey, &V) -> Result<(), Box<dyn Error>>,
    {
        match self {
            // Map::V0(m) => m.for_each(f),
            // Map::V2(m) => m.for_each(f),
            // Map::V3(m) => m.for_each(f),
            // Map::V4(m) => m.for_each(f),
            // Map::V5(m) => m.for_each(f),
            // Map::V6(m) => m.for_each(f),
            Map::V7(m) => m
                .for_each(|key, val| f(key, val).map_err(|e| anyhow::anyhow!("{}", e)))
                .map_err(|e| e.into()),
            _ => unimplemented!(),
        }
    }
}
