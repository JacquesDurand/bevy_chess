use bevy::prelude::*;

pub(crate) enum Pieces {
    Queen,
    Bishop,
    Rook,
    Pawn,
    King,
    Knight,
}

pub(crate) struct PieceColorMaterial {
    pub(crate) black: Handle<StandardMaterial>,
    pub(crate) white: Handle<StandardMaterial>,
}

pub(crate) struct ChessPiece {
    pub(crate) piece: Pieces,
    pub(crate) color_material: Handle<StandardMaterial>,
}

impl ChessPiece {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        position: Vec3,
        asset_server: AssetServer,
        transform: Vec3,
    ) {
        let meshes = self.get_asset(asset_server);

        commands
            .spawn()
            .insert_bundle(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with_children(|parent| {
                for mesh in meshes {
                    parent.spawn().insert_bundle(PbrBundle {
                        mesh: mesh.clone(),
                        material: self.color_material.clone(),
                        transform: {
                            let mut transform = Transform::from_translation(transform);
                            transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                            transform
                        },
                        ..Default::default()
                    });
                }
            });
    }

    pub fn get_asset(&self, asset_server: AssetServer) -> Vec<Handle<Mesh>> {
        match self.piece {
            Pieces::Queen => {
                let queen_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");
                vec![queen_handle]
            }
            Pieces::Bishop => {
                let bishop_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
                vec![bishop_handle]
            }
            Pieces::Rook => {
                let rook_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
                vec![rook_handle]
            }
            Pieces::Pawn => {
                let pawn_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
                vec![pawn_handle]
            }
            Pieces::King => {
                let king_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
                let king_cross_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
                vec![king_handle, king_cross_handle]
            }
            Pieces::Knight => {
                let knight_1_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
                let knight_2_handle: Handle<Mesh> =
                    asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
                vec![knight_1_handle, knight_2_handle]
            }
        }
    }
}
