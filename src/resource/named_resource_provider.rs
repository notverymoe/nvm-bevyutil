/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use bevy::utils::{HashMap, hashbrown::hash_map::Entry};

use crate::compact_str::CompactStr;

pub struct NamedResourceProvider<K: CompactStr, T> {
    resources: HashMap<K, T>,
}

impl<K: CompactStr, T> NamedResourceProvider<K, T> {

    pub fn insert(&mut self, id: K, value: T) -> Result<&mut T, &'static str> {
        self.insert_with(id, || value)
    }
    
    pub fn insert_with<F: FnOnce() -> T>(&mut self, id: K, value: F) -> Result<&mut T, &'static str> {
        match self.resources.entry(id) {
            Entry::Occupied(_) => Err("Entry with ID already exists"),
            Entry::Vacant(v)   => Ok(v.insert(value())),
        }
    }
    
    pub fn insert_or_replace(&mut self, id: K, value: T) -> &mut T {
        self.resources.insert(id, value);
        self.resources.get_mut(&id).unwrap()
    }

    pub fn remove(&mut self, id: K) -> Option<T> {
        self.resources.remove(&id)
    }

    pub fn get(&self, id: K) -> Option<&T> {
        self.resources.get(&id)
    }

    pub fn get_mut(&mut self, id: K) -> Option<&mut T> {
        self.resources.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &T)> {
        self.resources.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut T)> {
        self.resources.iter_mut()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.resources.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.resources.values_mut()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.resources.keys()
    }

    pub fn has(&self, id: K) -> bool {
        self.resources.contains_key(&id)
    }

}

impl<K: CompactStr, T: Default> NamedResourceProvider<K, T> {

    pub fn insert_default(&mut self, id: K) -> Result<&mut T, &'static str> {
        self.insert_with(id, || Default::default())
    }

    pub fn insert_or_replace_default(&mut self, id: K) -> &mut T {
        self.insert_or_replace(id, Default::default())
    }

}

