mod pieces;

use crate::pieces::{ChessPiece, PieceColorMaterial, Pieces};
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 800.,
            position: None,
            resize_constraints: Default::default(),
            scale_factor_override: None,
            title: "Chess".to_string(),
            vsync: false,
            resizable: false,
            decorations: false,
            cursor_visible: false,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            transparent: false,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
    commands.spawn().insert_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(1., 1., 1.).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn().insert_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = PieceColorMaterial {
        black: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        white: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
    };

    ChessPiece {
        piece: Pieces::Queen,
        color_material: material.black.clone(),
    }
    .spawn(
        &mut commands,
        Vec3::new(7., 0., 3.),
        asset_server.clone(),
        Vec3::new(-0.2, 0., -0.95),
    );

    ChessPiece {
        piece: Pieces::King,
        color_material: material.black.clone(),
    }
    .spawn(
        &mut commands,
        Vec3::new(7., 0., 4.),
        asset_server.clone(),
        Vec3::new(-0.2, 0., -1.9),
    );

    let black_bishop = ChessPiece {
        piece: Pieces::Bishop,
        color_material: material.black.clone(),
    };
    black_bishop.spawn(
        &mut commands,
        Vec3::new(7., 0., 2.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 0.),
    );
    black_bishop.spawn(
        &mut commands,
        Vec3::new(7., 0., 5.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 0.),
    );

    let black_rook = ChessPiece {
        piece: Pieces::Rook,
        color_material: material.black.clone()
    };
    black_rook.spawn(
        &mut commands,
        Vec3::new(7., 0., 0.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 1.8)
    );
    black_rook.spawn(
        &mut commands,
        Vec3::new(7., 0., 7.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 1.8)
    );

    let black_knight = ChessPiece {
        piece: Pieces::Knight,
        color_material: material.black.clone()
    };
    black_knight.spawn(
        &mut commands,
        Vec3::new(7., 0.,1.),
        asset_server.clone(),
        Vec3::new(-0.2,0.,0.9)
    );
    black_knight.spawn(
        &mut commands,
        Vec3::new(7., 0.,6.),
        asset_server.clone(),
        Vec3::new(-0.2,0.,0.9)
    );

    let black_pawn = ChessPiece {
        piece: Pieces::Pawn,
        color_material: material.black.clone()
    };

    for i in 0..8 {
        black_pawn.spawn(
            &mut commands,
            Vec3::new(6., 0., i as f32),
            asset_server.clone(),
            Vec3::new(-0.2, 0., 2.6)
        );
    }

    //whites

    ChessPiece {
        piece: Pieces::Queen,
        color_material: material.white.clone(),
    }
        .spawn(
            &mut commands,
            Vec3::new(0., 0., 3.),
            asset_server.clone(),
            Vec3::new(-0.2, 0., -0.95),
        );

    ChessPiece {
        piece: Pieces::King,
        color_material: material.white.clone(),
    }
        .spawn(
            &mut commands,
            Vec3::new(0., 0., 4.),
            asset_server.clone(),
            Vec3::new(-0.2, 0., -1.9),
        );

    let white_bishop = ChessPiece {
        piece: Pieces::Bishop,
        color_material: material.white.clone(),
    };
    white_bishop.spawn(
        &mut commands,
        Vec3::new(0., 0., 2.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 0.),
    );
    white_bishop.spawn(
        &mut commands,
        Vec3::new(0., 0., 5.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 0.),
    );

    let white_rook = ChessPiece {
        piece: Pieces::Rook,
        color_material: material.white.clone()
    };
    white_rook.spawn(
        &mut commands,
        Vec3::new(0., 0., 0.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 1.8)
    );
    white_rook.spawn(
        &mut commands,
        Vec3::new(0., 0., 7.),
        asset_server.clone(),
        Vec3::new(-0.1, 0., 1.8)
    );

    let white_knight = ChessPiece {
        piece: Pieces::Knight,
        color_material: material.white.clone()
    };
    white_knight.spawn(
        &mut commands,
        Vec3::new(0., 0.,1.),
        asset_server.clone(),
        Vec3::new(-0.2,0.,0.9)
    );
    white_knight.spawn(
        &mut commands,
        Vec3::new(0., 0.,6.),
        asset_server.clone(),
        Vec3::new(-0.2,0.,0.9)
    );

    let white_pawn = ChessPiece {
        piece: Pieces::Pawn,
        color_material: material.white.clone()
    };

    for i in 0..8 {
        white_pawn.spawn(
            &mut commands,
            Vec3::new(1., 0., i as f32),
            asset_server.clone(),
            Vec3::new(-0.2, 0., 2.6)
        );
    }
}
