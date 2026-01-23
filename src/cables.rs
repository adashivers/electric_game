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
    segments_per_unit_length: u64,
}
impl Default for Cable {
    fn default() -> Self { Cable { generated: false, segments_per_unit_length: 1 } }
}

// spawn a cable with given endpoints.
// this only creates the entity with base components. the actual meshes of the cables will be created the next time the cable generating system runs.
pub fn spawn_cable(mut commands: &mut Commands, start_point: &Entity, end_point: &Entity) {
    commands.spawn((
        Cable::default(),
        StartsFrom(*start_point),
        EndsAt(*end_point),
    ));
}

// generate meshes for cables that have been added in the last tick.
pub fn generate_added_cables(
    mut commands: Commands, 
    mut added_cables: Query<(Entity, &mut Cable, &StartsFrom, &EndsAt), Added<Cable>>,
    cable_connections: Query<(&GlobalTransform, &CableConnection)>,
) {
    for (cable_entity, mut cable, cable_start, cable_end) in added_cables.iter_mut() {
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
}