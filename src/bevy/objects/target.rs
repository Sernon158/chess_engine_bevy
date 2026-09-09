use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::EaseMethod;
use bevy_tweening::EntityCommandsTweeningExtensions;
use crate::bevy::objects::BoardComponent;
use crate::bevy::objects::PieceComponent;
use crate::bevy::components::PieceMoveAnimator;
use crate::engine::game::StandardGame;
use crate::engine::logger::LogItem;

#[derive(SceneComponent, Clone, Default)]
#[scene(TargetCompData)]
pub struct TargetComponent;


#[derive(Clone, Default)]
pub struct TargetCompData {
    pub target_pos: (usize, usize)
}


impl TargetComponent {
    pub fn scene(data: TargetCompData) -> impl Scene {
        let board_pos = BoardComponent::get_index_coords(data.target_pos);

        bsn! {
            #PieceMoveTarget
            Self
            Sprite {
                image: "textures/target.png",
                custom_size: Vec2::new(75., 75.)
            }
            Transform::from_xyz(board_pos.x, board_pos.y, 2.)
            Pickable

            on(Self::on_click)
        }
    }
    
    pub fn on_click(
        input: On<Pointer<Click>>,
        mut game: ResMut<StandardGame>,
        mut pieces: Query<(Entity, &PieceComponent, &Transform)>,
        targets: Query<(Entity, &Transform), With<TargetComponent>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let Some(piece) = game.selected_piece
            else { return };

        let Ok((_, target_transform)) = targets.get(input.entity)
            else { return };

        for (entity, _) in targets.iter() {
            commands.entity(entity).despawn();
        }

        let target_pos = BoardComponent::get_coords_index(target_transform.translation);

        for (
            entity,
            component,
            transform
        ) in pieces.iter_mut() {
            let piece_pos = game.board.get_piece_position(&component.0);

            if target_pos == piece_pos {
                let sound: Handle<AudioSource> = asset_server.load("sounds/piece_capture.mp3");
                commands.spawn(AudioPlayer(sound));

                commands.entity(entity).despawn();

                game.logger.add(LogItem {
                    capture: true,
                    piece,
                    target_piece: Some(component.0),
                    start: piece_pos,
                    target: target_pos
                });
            }

            if component.0 != piece { continue };

            commands.entity(entity)
                .scale_to(
                    Vec3::splat(1.),
                    Duration::from_millis(300),
                    EaseMethod::EaseFunction(EaseFunction::CubicOut),
                )
                .insert(PieceMoveAnimator::new(
                    transform.translation,
                    Vec3::new(
                        target_transform.translation.x,
                        target_transform.translation.y,
                        1.
                    )
                ));

            let sound: Handle<AudioSource> = asset_server.load("sounds/piece_move.mp3");
            commands.spawn(AudioPlayer(sound));

            game.selected_piece = None;
            game.animation_playing = true;
        }
    }
}