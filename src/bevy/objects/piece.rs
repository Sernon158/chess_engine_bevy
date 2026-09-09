use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::EaseMethod;
use bevy_tweening::EntityCommandsTweeningExtensions;
use crate::bevy::components::PieceMoveAnimator;
use crate::bevy::objects::BoardComponent;
use crate::bevy::objects::IdleAnimator;
use crate::bevy::objects::TargetComponent;
use crate::bevy::tools::AnimationCompleted;
use crate::engine::game::StandardGame;
use crate::engine::logger::LogItem;
use crate::engine::pieces::Piece;

#[derive(SceneComponent, Clone, Default)]
#[scene(PieceCompData)]
pub struct PieceComponent(pub Piece);


#[derive(Clone, Default)]
pub struct PieceCompData {
    pub piece: Piece,
    pub piece_pos: (usize, usize)
}


impl PieceComponent {
    pub fn scene(PieceCompData { piece, piece_pos }: PieceCompData) -> Option<impl Scene> {
        let coords = BoardComponent::get_index_coords(piece_pos);

        let Some(piece_name) = piece.get_display()
            else { return None };

        Some(bsn! {
            #ChessPiece
            Self(piece)
            Sprite {
                image: format!("textures/pieces/{piece_name}.png"),
                custom_size: Vec2::new(75., 75.)
            }
            Transform::from_xyz(coords.x, coords.y, 1.)

            IdleAnimator
            Pickable

            on(Self::on_click)
            on(Self::on_moving_animation_ended)
        })
    }


    pub fn on_click(
        input: On<Pointer<Click>>,
        mut query: Query<(Entity, &PieceComponent)>,
        mut game: ResMut<StandardGame>,
        targets: Query<Entity, With<TargetComponent>>,
        mut commands: Commands
    ) {
        let Ok((_, piece)) = query.get_mut(input.entity)
            else { return };
        
        let Piece { id, color, .. } = piece.0;

        if game.animation_playing { return };
        if game.current_turn != color { return };

        let mut second_click = false;

        if let PointerButton::Primary = input.event().button {
            for entity in targets.iter() {
                commands.entity(entity).despawn();
            }

            if game.selected_piece == Some(piece.0) {
                game.selected_piece = None;
                second_click = true;
            } else {
                game.selected_piece = Some(piece.0);
            }

            for board_piece in game.board.concat().iter() {
                if second_click { continue };

                let target_pos = game.board.get_piece_position(board_piece);

                let can_move = piece.0.can_move(
                    target_pos,
                    game.board.get_piece_position(
                        &game.board.get_piece_by_id(id).unwrap()
                    ),
                    &game,
                    true
                );
            
                if !can_move { continue };
                
                commands.spawn_scene(bsn! {
                    @TargetComponent { @target_pos }
                });
            }
            
            for (entity, _) in query.iter_mut() {
                commands.entity(entity).scale_to(
                    Vec3::splat(1.),
                    Duration::from_millis(300),
                    EaseMethod::EaseFunction(EaseFunction::CubicOut),
                );
            }

            if !second_click {
                commands.entity(input.entity).scale_to(
                    Vec3::splat(1.25),
                    Duration::from_millis(300),
                    EaseMethod::EaseFunction(EaseFunction::CubicOut),
                );
            }
        }
    }


    fn on_moving_animation_ended(
        input: On<AnimationCompleted<PieceMoveAnimator>>,
        mut game: ResMut<StandardGame>,
        query: Query<(Entity, &PieceComponent, &Transform)>,
        mut commands: Commands
    ) {
        let Ok((entity, piece_component, transform)) = query.get(input.entity)
            else { return };

        commands.entity(entity).remove::<PieceMoveAnimator>();

        let (
            curr_x, curr_y,
            target_x, target_y,
            curr_color, curr_id,
            target_id,
        ) = {
            let (curr_x, curr_y) = game.board.get_piece_position(&piece_component.0);
            let (target_x, target_y) = BoardComponent::get_coords_index(transform.translation);

            let curr_piece = piece_component.0;
            let target_id = game.board.get(target_x, target_y).id;

            (curr_x, curr_y, target_x, target_y, curr_piece.color, curr_piece.id, target_id)
        };

        let mut moved_piece = piece_component.0;
        moved_piece.set_id(curr_id);

        game.board.set(target_x, target_y, moved_piece);
        game.board.set(curr_x, curr_y, Piece::none(target_id));

        game.logger.add(LogItem {
            capture: false,
            piece: moved_piece,
            target_piece: None,
            start: (curr_x, curr_y),
            target: (target_x, target_y)
        });

        game.current_turn = curr_color.get_opposite();
        game.animation_playing = false;
    }
}