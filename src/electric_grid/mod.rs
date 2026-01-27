use bevy::{platform::collections::HashMap, prelude::*, scene::SceneInstanceReady};
use bevy_polyline::PolylinePlugin;
use cables::*;

pub mod cables;



pub struct ElectricGridPlugin;
impl Plugin for ElectricGridPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            CablesPlugin,
            PolylinePlugin,
        ))
        .add_observer(connect_cables)
        .add_observer(use_tower_spawners)
        .add_systems(Startup, load_models);
    }
}

#[derive(Resource)]
pub struct TowerScene(pub Handle<Gltf>);

fn load_models(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let tower_scene: Handle<Gltf> = asset_server.load("transmission_tower\\TRANSMISSION_TOWER.glb");
    commands.insert_resource(TowerScene(tower_scene));
}

#[derive(Component, Default)]
pub struct TowerSpawner(pub Vec<Vec3>);

#[derive(Component)]
struct Tower {
    prev: Option<Entity>
}

impl TowerSpawner {
    // TODO: test
    pub fn spawn(&self, commands: &mut Commands, gltf_assets: &Res<Assets<Gltf>>, tower_scene: &Res<TowerScene>) {
        let tower_gltf = gltf_assets.get(&tower_scene.0).unwrap();
        let mut pos_dir_iter = self.0.clone().into_iter()
            .zip(get_dirs(&self.0).into_iter());

        let (pos, dir) = pos_dir_iter.next().unwrap();
        let mut last_entity = commands.spawn((
            Name::new("Transmission Tower"),
            Transform::from_translation(pos).looking_to(dir, Vec3::Y),
            SceneRoot(tower_gltf.scenes[0].clone()),
            Tower{ prev: None },
        )).id();
        loop {
            match pos_dir_iter.next() {
                Some((pos, dir)) => {
                    last_entity = commands.spawn((
                        Name::new("Transmission Tower"),
                        Transform::from_translation(pos).looking_to(dir, Vec3::Y),
                        SceneRoot(tower_gltf.scenes[0].clone()),
                        Tower{ prev: Some(last_entity.clone()) },
                    )).id();
                },
                None => break,
            }
        }
    }
}

fn use_tower_spawners(
    trigger: On<Add, TowerSpawner>,
    tower_spawners: Query<&TowerSpawner>,
    mut commands: Commands, 
    gltf_assets: Res<Assets<Gltf>>, 
    tower_scene: If<Res<TowerScene>>,
) {
    tower_spawners.get(trigger.entity).unwrap().spawn(&mut commands, &gltf_assets, &tower_scene);
    commands.entity(trigger.entity).despawn();
}

// TODO: test
fn get_dirs(spawn_positions: &Vec<Vec3>) -> Vec<Dir3> {
    if spawn_positions.is_empty() { unimplemented!("get_dirs cannot take an empty Vec") }
    if spawn_positions.len() == 1 {
        return vec![Dir3::X];
    }
    let mut dir_vec: Vec<Dir3> = Vec::new();
    let mut iter = spawn_positions.iter();
    let mut last_vec = iter.next().unwrap();
    loop {
        match iter.next() {
            Some(this_vec) => {
                let dir = this_vec - last_vec;
                let dir = Vec3::new(dir.x, 0.0, dir.z);
                dir_vec.push(Dir3::new(dir).unwrap());
                last_vec = this_vec;
            },
            None => {
                dir_vec.push(dir_vec.last().unwrap().clone());
                break;
            }
        }
    }
    dir_vec
}

// TODO: test
fn connect_cables(
    trigger: On<SceneInstanceReady>,
    towers: Query<&Tower>,
    children: Query<&Children>,
    connections: Query<&CableConnection>,
    mut commands: Commands,
) {
    let tower = towers.get(trigger.entity);
    if tower.is_err() { return; }
    debug!("found tower");
    let tower = tower.unwrap();
    let tower_entity = trigger.entity;
    if tower.prev == None {
        debug!("tower has no prev, skipping cable connection");
        return;
    }
    let prev_tower_entity = tower.prev.unwrap();

    let found_connections = get_cable_connections_in_scene(&tower_entity, &children, &connections);
    let prev_found_connetions = get_cable_connections_in_scene(&prev_tower_entity, &children, &connections);
    for (index, connection_entity) in found_connections {
        if let Some(prev_connection_entity) = prev_found_connetions.get(&index) {
            spawn_cable(&mut commands, &prev_connection_entity, &connection_entity, None);
        }
    }
    
}

// TODO: test
fn get_cable_connections_in_scene(
    scene_entity: &Entity, 
    children: &Query<&Children>,
    connections: & Query<&CableConnection>,
) -> HashMap<u32, Entity> {
    let mut found_connections: HashMap<u32, Entity> = HashMap::new();
    for entity in children.iter_descendants_depth_first(*scene_entity) {
        if let Ok(connection) = connections.get(entity) {
            debug!("found connection in tower with index {}", connection.index);
            found_connections.insert(connection.index, entity);
        }
    }
    found_connections
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}