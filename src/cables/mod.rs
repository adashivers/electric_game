use bevy::prelude::*;


pub struct CablesPlugin;
impl Plugin for CablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(generate_added_cables);
    }
}

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
    segments_per_unit_length: u64,
}
impl Default for Cable {
    fn default() -> Self { Cable { generated: false, segments_per_unit_length: 1 } }
}

// spawn a cable with given endpoints.
// this only creates the entity with base components. the actual meshes of the cables will be created the next time the cable generating system runs.
pub fn spawn_cable(commands: &mut Commands, start_point: &Entity, end_point: &Entity) -> Entity {
    commands.spawn((
        Cable::default(),
        StartsFrom(*start_point),
        EndsAt(*end_point),
    )).id()
}

// generate meshes for cables that have been added in the last tick.
fn generate_added_cables(
    event: On<Add, Cable>,
    mut added_cables: Query<(&mut Cable, &StartsFrom, &EndsAt)>,
    cable_connections: Query<(&GlobalTransform, &CableConnection)>,
) {
    debug!("generating added cable...");
    let (mut cable, cable_start, cable_end) = added_cables.get_mut(event.entity).unwrap();
    let (start_transform, start_connection) = cable_connections.get(cable_start.0).unwrap();
    let (end_transform, end_connection) = cable_connections.get(cable_end.0).unwrap();

    let start_pos = start_transform.translation() + start_connection.connection_point_offset;
    let end_pos = end_transform.translation() + end_connection.connection_point_offset;

    // todo (with my approximation as to how hard each task will be): 
    // 1. (medium) create a function Catenary that takes in parameter t \in [0, 1], start pos, end pos and length and returns the catenary position at t
    // 2. (one line) create a FunctionCurve catenary_curve by supplying |t| Catenary(start_pos, end_pos, length, t)
    // 3. (one line) sample catenary_curve using Cable.segments_per_unit_length, t, and length (should be enough information, need to do the math on this)
    // 4. (hard) generate mesh or use shaders to display rope 
    //    (not sure how to do this yet.. look into https://bevy.org/examples/shaders/automatic-instancing/ and https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#wgsl)
    // 5. (medium) in a separate system, add stuff like wind sim using sin disturbations
    
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;

    fn spawn_some_endpoints(world: &mut World) -> (Entity, Entity) {
        let from = world.spawn(Transform::from_translation(Vec3::ZERO)).id();
        let to = world.spawn(Transform::from_translation(Vec3::Y)).id();
        (from, to)
    }

    #[test]
    fn test_cable_spawn_components_correct() {
        let mut app = App::new();
        let world = app.world_mut();
        let (from, to) = spawn_some_endpoints(world);

        let cable_entity = spawn_cable(&mut world.commands(), &from, &to);
        world.flush();

        let mut query_state = world.query::<(&Cable, &StartsFrom, &EndsAt)>();
        let query_result = query_state.get(world, cable_entity);
        assert!(query_result.is_ok());

        let (cable, exposed_from, exposed_to) = query_result.unwrap();
        assert!(!cable.generated);
        assert_eq!(from.index(), exposed_from.0.index());
        assert_eq!(to.index(), exposed_to.0.index());
    }
}