use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use examples_common_3d::XpbdExamplePlugin;

#[derive(Resource)]
struct GravitySwitch(bool);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(SubstepCount(50))
        .insert_resource(Gravity((Vec3::ZERO).into()))
        .insert_resource(GravitySwitch(false))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_gravity)
        .run();
}

fn setup(mut commands: Commands) {
    let angle_t = Vec3::new(0., 1., 0.);

    let object_t = Vec3::new(-2., 0., 2.);
    let object_r = Quat::from_rotation_arc(Vec3::Y, (object_t - angle_t).normalize());

    let object_radius = 0.1_f64;
    let object_height = f64::from((object_t - angle_t).length()) - 2. * object_radius;

    let center_radius = 0.4_f64;
    let center = commands
        .spawn((
            RigidBody::Static,
            Collider::sphere(center_radius),
            TransformBundle::from_transform(Transform::from_translation(angle_t)),
        ))
        .id();

    let offset_1 = center_radius * 0.25 * std::f64::consts::PI;
    let object_offset = Vec3::new(-offset_1 as f32, 0., offset_1 as f32);
    let object = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::capsule(object_height, object_radius),
            TransformBundle::from_transform(Transform {
                translation: (angle_t + object_t) * 0.5 // midpoint
                    + object_offset,
                rotation: object_r,
                scale: Vec3::ONE,
            }),
            IgnoredCollisions::from_iter([center]),
        ))
        .id();

    let compliance = 0.;
    let a1 = Vec3::X;
    let a2 = object_r * Vec3::X;
    let n = a1.cross(a2).normalize();
    let angle = a1.cross(a2).dot(n).asin() as f64;

    // Joint
    commands.spawn(
        SphericalJoint::new(center, object)
            .with_swing_limits(angle, angle)
            .with_compliance(compliance)
            .with_local_anchor_1(Vector::new(-offset_1, 0., offset_1))
            .with_local_anchor_2(Vector::new(0., -object_height * 0.5 + -object_radius, 0.)),
    );

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn toggle_gravity(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut switch: ResMut<GravitySwitch>,
) {
    if keys.just_pressed(KeyCode::KeyG) {
        if switch.0 {
            commands.insert_resource(Gravity((Vec3::ZERO).into()));
            *switch = GravitySwitch(false);
            println!("Gravity OFF");
        } else {
            commands.insert_resource(Gravity((Vec3::NEG_Y).into()));
            *switch = GravitySwitch(true);
            println!("Gravity ON");
        }
    }
}
