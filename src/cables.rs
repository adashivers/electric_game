use bevy::prelude::*;


/* 
a component for designating an entity with a global position as a cable connection point.
ideally, this component should be inserted in an entity containing a transform. if not, a transform will be created for it.
inserting this component will also insert two relationshiptarget components (CablesStartingHere and CablesEndingHere) that
will keep track of cables connecting to it in an oriented way.
*/
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[type_path = "api"]
#[require(GlobalTransform, CablesStartingHere, CablesEndingHere)]
pub struct CableConnection {
    pub connection_point_offset: Vec3,
}

#[derive(Component ,Default)]
#[relationship_target(relationship = StartsFrom)]
struct CablesStartingHere(Vec<Entity>);

#[derive(Component ,Default)]
#[relationship_target(relationship = EndsAt)]
struct CablesEndingHere(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = CablesStartingHere)]
struct StartsFrom(Entity);

#[derive(Component)]
#[relationship(relationship_target = CablesEndingHere)]
struct EndsAt(Entity);

#[derive(Component, Reflect)]
struct Cable {
    generated: bool,
    segments_per_unit: u64,
}
impl Default for Cable {
    fn default() -> Self { Cable { generated: false, segments_per_unit: 1 } }
}

pub fn spawn_cable(mut commands: &mut Commands, start_point: &Entity, end_point: &Entity) {
    commands.spawn((
        Cable::default(),
        StartsFrom(*start_point),
        EndsAt(*end_point),
    ));
}

pub fn generate_cables(mut commands: Commands, added_cables: Query<(Entity, &Cable, &StartsFrom, &EndsAt), Added<Cable>>) {
    for (cable_entity, cable, cable_start, cable_end) in added_cables {
        // TODO
    }
}