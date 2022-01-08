/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use std::collections::hash_map::Entry;

use bevy::utils::HashMap;

use crate::compact_str::CompactStr;

pub struct NamedResourceProvider<T, const N: usize> {
    resources: HashMap<CompactStr<N>, T>,
}

impl<T, const N: usize> NamedResourceProvider<T, N> {

    pub fn insert(&mut self, id: CompactStr<N>, value: T) -> Result<&mut T, &'static str> {
        self.insert_with(id, || value)
    }
    
    pub fn insert_with<F: FnOnce() -> T>(&mut self, id: CompactStr<N>, value: F) -> Result<&mut T, &'static str> {
        match self.resources.entry(id) {
            Entry::Occupied(_) => Err("Entry with ID already exists"),
            Entry::Vacant(v)   => Ok(v.insert(value())),
        }
    }
    
    pub fn insert_or_replace(&mut self, id: CompactStr<N>, value: T) -> &mut T {
        self.resources.insert(id, value);
        self.resources.get_mut(&id).unwrap()
    }

    pub fn remove(&mut self, id: CompactStr<N>) -> Option<T> {
        self.resources.remove(&id)
    }

    pub fn get(&self, id: CompactStr<N>) -> Option<&T> {
        self.resources.get(&id)
    }

    pub fn get_mut(&mut self, id: CompactStr<N>) -> Option<&mut T> {
        self.resources.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CompactStr<N>, &T)> {
        self.resources.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&CompactStr<N>, &mut T)> {
        self.resources.iter_mut()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.resources.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.resources.values_mut()
    }

    pub fn keys(&self) -> impl Iterator<Item = &CompactStr<N>> {
        self.resources.keys()
    }

    pub fn has(&self, id: CompactStr<N>) -> bool {
        self.resources.contains_key(&id)
    }

}

impl<T: Default, const N: usize> NamedResourceProvider<T, N> {

    pub fn insert_default(&mut self, id: CompactStr<N>) -> Result<&mut T, &'static str> {
        self.insert_with(id, || Default::default())
    }

    pub fn insert_or_replace_default(&mut self, id: CompactStr<N>) -> &mut T {
        self.insert_or_replace(id, Default::default())
    }

}

