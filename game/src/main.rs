use bevy::{prelude::*};


#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

pub struct Iniplug;

impl Movable {
    fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 15.0,
            speed: 75.0,
            rotation_speed: 4.0,
        }
    }
}


impl Plugin for Iniplug{
    fn build(&self,app:&mut App){
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_circ,move_enemy));
    }
}

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Iniplug)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}

fn setup(mut commands:Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<ColorMaterial>>){
    let entity_spawn = Vec3::ZERO;
    commands.spawn((
        Background,
        Mesh2d(meshes.add(Rectangle::new(1200.0, 820.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    ));
    commands.spawn((
        Background,
        Mesh2d(meshes.add(Rectangle::new(1160.0, 800.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 0.0)))
    ));
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(Vec2::new(0.0, 40.0), Vec2::new(-20.0, 0.0), Vec2::new(20.0, 0.0)))),
        MeshMaterial2d(materials.add(Color::srgb(0.15, 0.6, 0.3))),
        Movable::new(entity_spawn)
    ));
    commands.spawn((
        Camera2d,
        Transform::from_xyz(10.0, 20.0, 0.0),
        Camera
    ));
    commands.spawn((
        Enemy,
        Health{
            hp:100.00,
            extra:0.0
        },
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0,0.0,0.0),
        Creature,
        Movable::new(entity_spawn)
    ));
}


fn move_circ(mut circs: Query<(&mut Transform, &mut Movable), Without<Enemy>> ,keys: Res<ButtonInput<KeyCode>>, timer: Res<Time>) {
    for (mut transform, mut circ) in &mut circs {
        let directiony = transform.local_y();
        if keys.just_pressed(KeyCode::ShiftLeft) {
            circ.speed *= 2.0;
        }
        if keys.just_released(KeyCode::ShiftLeft) {
            circ.speed /= 2.0;
        }
        if keys.pressed(KeyCode::KeyW) {
            transform.translation += directiony * circ.speed * timer.delta_secs() * 2.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation += -directiony * circ.speed * timer.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.rotate_z(circ.rotation_speed * timer.delta_secs());
        } 
        if keys.pressed(KeyCode::KeyD) {
            transform.rotate_z(-circ.rotation_speed * timer.delta_secs());
        }
    }
}

fn move_enemy(mut enemies: Query<(&mut Transform, &mut Movable), With<Enemy>> ,timer: Res<Time>) {
    for (mut transform, enemie) in &mut enemies {
        let directiony = transform.local_y();
        transform.translation += directiony * enemie.speed * timer.delta_secs() * 2.0;
    }
}