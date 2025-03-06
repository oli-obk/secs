#[cfg(not(feature = "multithreaded"))]
use elsa::FrozenMap;
#[cfg(feature = "multithreaded")]
use elsa::sync::FrozenMap;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{any::TypeId, collections::HashMap};

use crate::world::{Entity, SendSync};

pub struct SparseSet<C> {
    pub sparse: HashMap<Entity, usize>,
    /// Entries that have been freed and are thus available again.
    pub empty: Vec<usize>,
    pub dense: Vec<Option<C>>,
    pub ids: Vec<Entity>,
}

impl<C> SparseSet<C> {
    pub fn new(entity: Entity, component: C) -> Self {
        let mut sparse = HashMap::new();
        sparse.insert(entity, 0);

        Self {
            sparse,
            empty: vec![],
            dense: vec![Some(component)],
            ids: vec![entity],
        }
    }

    pub fn insert(&mut self, entity: Entity, component: C) {
        self.sparse.insert(entity, self.dense.len());
        if let Some(idx) = self.empty.pop() {
            self.ids[idx] = entity;
            self.dense[idx] = Some(component);
        } else {
            self.dense.push(Some(component));
            self.ids.push(entity);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(idx) = self.sparse.remove(&entity) {
            self.empty.push(idx);
            self.dense[idx].take();
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        let &id = self.sparse.get(&entity)?;
        Some(self.dense[id].as_ref().unwrap())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        let &id = self.sparse.get(&entity)?;
        Some(self.dense[id].as_mut().unwrap())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &C)> {
        self.ids
            .iter()
            .copied()
            .zip(self.dense.iter())
            .filter_map(|(e, i)| Some((e, i.as_ref()?)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.ids
            .iter()
            .copied()
            .zip(self.dense.iter_mut())
            .filter_map(|(e, i)| Some((e, i.as_mut()?)))
    }
}

trait Set: SendSync {
    fn remove(&mut self, entity: Entity);
}

impl<C: SendSync> Set for SparseSet<C> {
    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }
}

#[derive(Default)]
pub struct SparseSets {
    sets: FrozenMap<TypeId, Box<RwLock<dyn Set>>>,
}

impl SparseSets {
    pub fn insert<C: SendSync>(&self, entity: Entity, component: C) {
        self.sets.insert(
            TypeId::of::<C>(),
            Box::new(RwLock::new(SparseSet::new(entity, component))),
        );
    }

    pub fn remove(&mut self, entity: Entity) {
        for set in self.sets.as_mut().values() {
            let Some(mut guard) = set.try_write() else {
                panic!(
                    "Tried to access component mutably, but it is already being read or written to",
                )
            };
            guard.remove(entity);
        }
    }

    #[track_caller]
    pub fn get<C: 'static>(&self) -> Option<MappedRwLockReadGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_read() else {
            panic!(
                "Tried to access component `{}`, but it was already being written to",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockReadGuard::map(guard, |dynbox| unsafe {
            let dynthing: *const dyn Set = dynbox;
            &*dynthing.cast::<SparseSet<C>>()
        }))
    }

    #[track_caller]
    pub fn get_mut<C: 'static>(&self) -> Option<MappedRwLockWriteGuard<SparseSet<C>>> {
        let set = self.sets.get(&TypeId::of::<C>())?;
        let Some(guard) = set.try_write() else {
            panic!(
                "Tried to access component `{}` mutably, but it was already being written to or read from",
                std::any::type_name::<C>()
            )
        };
        Some(RwLockWriteGuard::map(guard, |dynbox| unsafe {
            let dynthing: *mut dyn Set = dynbox;
            &mut *dynthing.cast::<SparseSet<C>>()
        }))
    }
}
