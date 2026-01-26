use bevy::{color::palettes::css::{BLUE, GREEN, RED, GRAY}, prelude::*};
use bevy_polyline::prelude::*;

use crate::cables::catenary::get_parabola;

mod catenary;

pub struct CablesPlugin;
impl Plugin for CablesPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_gizmo_group::<CableGizmos>()
        .add_observer(generate_added_cables)
        .add_systems(Update, cable_gizmos);
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

#[derive(Component)]
struct Cable {
    generated: bool,
    segment_num: u64,
    segments: Vec<Vec3>,
    pub color: LinearRgba,
}
impl Default for Cable {
    fn default() -> Self { Cable { generated: false, segment_num: 5, segments: Vec::new(), color: GRAY.into() } }
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
    mut commands: Commands,
    mut added_cables: Query<(Entity, &mut Cable, &StartsFrom, &EndsAt)>,
    cable_connections: Query<(&GlobalTransform, &CableConnection)>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
) {
    debug!("generating added cable...");
    let (cable_entity, mut cable, cable_start, cable_end) = added_cables.get_mut(event.entity).unwrap();
    let (start_transform, start_connection) = cable_connections.get(cable_start.0).unwrap();
    let (end_transform, end_connection) = cable_connections.get(cable_end.0).unwrap();

    let start_pos = start_transform.translation() + start_connection.connection_point_offset;
    let end_pos = end_transform.translation() + end_connection.connection_point_offset;

    // create FunctionCurve and sample at segments
    let curve = FunctionCurve::new(Interval::UNIT, |t| get_parabola(t, start_pos, end_pos).unwrap());
    let samples: Vec<Vec3> = (0..=cable.segment_num).map(|x| curve.sample(x as f32 / cable.segment_num as f32).unwrap()).collect();

    // insert polyline
    commands.entity(cable_entity).insert(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { vertices: samples.clone() })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 2.0,
            color: cable.color,
            perspective: false,
            ..default()
        })),
        ..default()
    });
    cable.generated = true;
    cable.segments = samples;
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct CableGizmos;

fn cable_gizmos(
    mut gizmos: Gizmos<CableGizmos>,
    cables: Query<(&StartsFrom, &EndsAt), With<Cable>>,
    cable_connections: Query<(&GlobalTransform, &CableConnection)>,
) {
    for (from, to) in cables {
        let (from_transform, from_conn) = cable_connections.get(from.0).unwrap();
        let (to_transform, to_conn) = cable_connections.get(to.0).unwrap();
        gizmos.sphere(from_transform.to_isometry(), 0.1, RED);
        gizmos.line(from_transform.translation(), from_transform.transform_point(from_conn.connection_point_offset), RED);
        gizmos.sphere(to_transform.to_isometry(), 0.1, BLUE);
        gizmos.line(to_transform.translation(), to_transform.transform_point(to_conn.connection_point_offset), BLUE);
    }
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