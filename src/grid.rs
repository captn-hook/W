
use bevy::prelude::*;

pub fn grid(mut gizmos: Gizmos) {
    const SIZE: f32 = 5.;
    for x in -SIZE as i32..=SIZE as i32 {
        for z in -SIZE as i32..=SIZE as i32 {
            gizmos.line(
                Vec3::new(x as f32 + 0.5, 0., -SIZE),
                Vec3::new(x as f32 + 0.5, 0., SIZE),
                Color::WHITE,
            );
            gizmos.line(
                Vec3::new(-SIZE, 0., z as f32 + 0.5),
                Vec3::new(SIZE, 0., z as f32 + 0.5),
                Color::WHITE,
            );
        }
    }
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.)),
        Color::BLACK,
    );
    // gizmos.rect(
    //     Vec3::new(2., 1., 0.),
    //     Quat::from_rotation_y(3. / 2.),
    //     Vec2::splat(2.),
    //     Color::GREEN,
    // );
}
