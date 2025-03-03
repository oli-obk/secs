use crate::world::{Entity, World, WorldQuery};

pub trait Query<'a>: Sized {
    type Short<'b>;

    #[track_caller]
    fn get_components(world: &'a World, f: impl for<'b> FnMut(Entity, Self::Short<'b>));
}

impl<'a, C1: 'static> Query<'a> for (&'a C1,) {
    type Short<'b> = (&'b C1,);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            f(entity, (c1,));
        }
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a C1, &'a C2) {
    type Short<'b> = (&'b C1, &'b C2);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            f(entity, (c1, c2));
        }
    }
}

impl<'a, C1: 'static> Query<'a> for (&'a mut C1,) {
    type Short<'b> = (&'b mut C1,);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            f(entity, (c1,));
        }
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a mut C1, &'a mut C2) {
    type Short<'b> = (&'b mut C1, &'b mut C2);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            f(entity, (c1, c2));
        }
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a C1, &'a mut C2) {
    type Short<'b> = (&'b C1, &'b mut C2);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            f(entity, (c1, c2));
        }
    }
}

impl<'a, C1: 'static, C2: 'static> Query<'a> for (&'a mut C1, &'a C2) {
    type Short<'b> = (&'b mut C1, &'b C2);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            f(entity, (c1, c2));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a C2, &'a C3) {
    type Short<'b> = (&'b C1, &'b C2, &'b C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        let Some(s3) = world.get_sparse_set::<C3>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a mut C2, &'a mut C3) {
    type Short<'b> = (&'b mut C1, &'b mut C2, &'b mut C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        let Some(mut s3) = world.get_sparse_set_mut::<C3>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &mut s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a C2, &'a C3) {
    type Short<'b> = (&'b mut C1, &'b C2, &'b C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        let Some(s3) = world.get_sparse_set::<C3>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a mut C2, &'a mut C3) {
    type Short<'b> = (&'b C1, &'b mut C2, &'b mut C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        let Some(mut s3) = world.get_sparse_set_mut::<C3>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &mut s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a mut C2, &'a C3) {
    type Short<'b> = (&'b mut C1, &'b mut C2, &'b C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        let Some(s3) = world.get_sparse_set::<C3>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a mut C2, &'a C3) {
    type Short<'b> = (&'b C1, &'b mut C2, &'b C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(mut s2) = world.get_sparse_set_mut::<C2>() else {
            return;
        };
        let Some(s3) = world.get_sparse_set::<C3>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &mut s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a mut C1, &'a C2, &'a mut C3) {
    type Short<'b> = (&'b mut C1, &'b C2, &'b mut C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(mut s1) = world.get_sparse_set_mut::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        let Some(mut s3) = world.get_sparse_set_mut::<C3>() else {
            return;
        };
        for (entity, c1) in &mut *s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &mut s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}

impl<'a, C1: 'static, C2: 'static, C3: 'static> Query<'a> for (&'a C1, &'a C2, &'a mut C3) {
    type Short<'b> = (&'b C1, &'b C2, &'b mut C3);

    #[track_caller]
    fn get_components(world: &'a World, mut f: impl for<'b> FnMut(Entity, Self::Short<'b>)) {
        let Some(s1) = world.get_sparse_set::<C1>() else {
            return;
        };
        let Some(s2) = world.get_sparse_set::<C2>() else {
            return;
        };
        let Some(mut s3) = world.get_sparse_set_mut::<C3>() else {
            return;
        };
        for (entity, c1) in &*s1 {
            let Some(c2) = s2.get(entity) else { continue };
            let c2 = &s2.dense[c2];
            let Some(c3) = s3.get(entity) else { continue };
            let c3 = &mut s3.dense[c3];
            f(entity, (c1, c2, c3));
        }
    }
}
