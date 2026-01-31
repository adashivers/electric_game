use bevy::{ color::palettes::css::YELLOW, prelude::*};
use super::cables::*;

pub struct SparkMovementPlugin;
impl Plugin for SparkMovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_gizmo_group::<SparkGizmos>()
        .add_systems(Update, (move_spark, spark_gizmos));
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Spark { 
    pub(crate) connected_to_cable_entity: Entity,
    pub speed: f32, // per second
    pub(crate) dist_along: f32,
}

impl Spark {
    pub fn new(start_cable_entity: Entity, speed: f32) -> Self {
        Spark {
            connected_to_cable_entity: start_cable_entity,
            speed,
            dist_along: 0.,
        }
    }
}

fn move_spark(
    mut sparks: Query<(&mut Spark, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cables: Query<(&Cable, &StartsFrom, &EndsAt)>,
    cable_start_connections: Query<&CablesStartingHere>,
    cable_end_connections: Query<&CablesEndingHere>,
) {
    if let Ok((mut spark, mut spark_transform)) = sparks.single_mut() {
        if keyboard.pressed(KeyCode::KeyW) {
            spark.dist_along += spark.speed * time.delta_secs();
            set_spark_transform_and_dist_along(&mut spark, &mut spark_transform, &cables, &cable_start_connections, &cable_end_connections);
        } else if keyboard.pressed(KeyCode::KeyS) {
            spark.dist_along -= spark.speed * time.delta_secs();
            set_spark_transform_and_dist_along(&mut spark, &mut spark_transform, &cables, &cable_start_connections, &cable_end_connections);
        }
    }
}

fn set_spark_transform_and_dist_along(
    spark: &mut Spark, 
    spark_transform: &mut Transform, 
    cables: &Query<(&Cable, &StartsFrom, &EndsAt)>,
    cable_start_connections: &Query<&CablesStartingHere>,
    cable_end_connections: &Query<&CablesEndingHere>,
) {
    let (connected_cable, prev_cable_connection, next_cable_connection) = cables.get(spark.connected_to_cable_entity).unwrap();

    // if t is within bounds
    if Interval::UNIT.contains(spark.dist_along) {
        spark_transform.translation = connected_cable.get_pos_along(spark.dist_along);
        return; 
    } 
    // if not, we will have to get to the next cable in the relationship, OR stop if there is none
    else if spark.dist_along > 1.0 {
        // overshoot, get next
        match cable_start_connections.get(next_cable_connection.0) {
            Ok(cables_starting_at_next_connector) => {
                // for now, just get the first starter
                match cables_starting_at_next_connector.collection().iter().next() {
                    // next cable exists, move to it
                    Some(next_cable_entity) => {
                        spark.dist_along = spark.dist_along - 1.0;
                        spark.connected_to_cable_entity = *next_cable_entity;
                        // try again on new cable
                        set_spark_transform_and_dist_along(spark, spark_transform, cables, cable_start_connections, cable_end_connections);
                        return;
                    },
                    // end of the line, just clamp t at 1 and stay on the same cable
                    None => {
                        spark.dist_along = 1.0;
                        spark_transform.translation = connected_cable.get_pos_along(spark.dist_along);
                        return; 
                    }
                }
            },
            Err(_) => {
                unreachable!();
            }
        }
    } else if spark.dist_along < 0.0 {
        // undershoot, get prev
        match cable_end_connections.get(prev_cable_connection.0) {
            Ok(cables_ending_at_prev_connector) => {
                match cables_ending_at_prev_connector.collection().iter().next() {
                    // prev cable exists, move to it
                    Some(prev_cable_entity) => {
                        spark.dist_along = spark.dist_along + 1.0;
                        spark.connected_to_cable_entity = *prev_cable_entity;
                        // try again on new cable
                        set_spark_transform_and_dist_along(spark, spark_transform, cables, cable_start_connections, cable_end_connections);
                        return;
                    },
                    // end of the line, just clamp t at 0 and stay on the same cable
                    None => {
                        spark.dist_along = 0.0;
                        spark_transform.translation = connected_cable.get_pos_along(spark.dist_along);
                        return; 
                    }
                }
            },
            Err(_) => {
                unreachable!();
            }
        }
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct SparkGizmos;

fn spark_gizmos(
    mut gizmos: Gizmos<SparkGizmos>,
    spark: Query<&GlobalTransform, With<Spark>>,
) {
    for spark_transform in spark {
        gizmos.sphere(spark_transform.to_isometry(), 1.0, YELLOW);
    }
}