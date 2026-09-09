use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::EaseMethod;
use bevy_tweening::EntityCommandsTweeningExtensions;
use crate::bevy::components::PieceMoveAnimator;
use crate::bevy::objects::BoardComponent;
use crate::bevy::objects::PieceComponent;
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
        let start_pos = game.board.get_piece_position(&piece);

        let target_piece = match game.board.get(target_pos.0, target_pos.1) {
            p if p.is_none() => None,
            p => Some(p.clone())
        };
        
        let mut sound: Option<&'static str> = None;
        let mut capture = false;
        let mut check = false;
        
        if target_piece.is_some() {
            sound = Some("sounds/piece_capture.mp3");
            capture = true;
        }

        if game.is_king_checked_after_move(
            piece.color.get_opposite(),
            start_pos,
            target_pos
        ) {
            sound = Some("sounds/check.mp3");
            check = true;
        }

        if let Some(sound) = sound {
            commands.spawn(AudioPlayer::<AudioSource>(asset_server.load(sound)));
        }
        
        // Always play a piece move sound. Even if one of the other sounds has
        // played, this will be played to complement them.
        commands.spawn(AudioPlayer::<AudioSource>(asset_server.load("sounds/piece_move.mp3")));

        game.logger.add(LogItem {
            capture, check, piece,
            target_piece,
            start: start_pos,
            target: target_pos
        });

        let (entity, _, transform) = pieces.iter()
            .find(|(_, pc, _)| pc.0 == piece)
            .unwrap();

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

        game.selected_piece = None;
        game.animation_playing = true;

        if let Some(target_piece) = target_piece
        && let Some((entity, _, _)) = pieces.iter_mut()
            .find(|(_, component, _)| component.0 == target_piece)
        {
            commands.entity(entity).despawn();
        }
    }
}