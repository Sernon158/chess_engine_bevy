use bevy::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::Animator;
use bevy_tweening::EaseMethod;

use crate::bevy::components::PieceMoveAnimator;
use crate::bevy::components::{ BoardComponent, IdleAnimator, PieceComponent, TargetComponent };
use crate::bevy::events::AnimationCompleted;
use crate::bevy::methods::new_animator;
use crate::bevy::structs::AnimationLoop;
use crate::board::Board;
use crate::game::StandardGame;
use crate::logger::LogItem;
use crate::pieces::Piece;

pub fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    let default_board = Board::new();

    for piece in default_board.concat().iter() {
        let piece_position = default_board.get_piece_position(piece);
        let coords = BoardComponent::get_index_coords(piece_position);
        let Some(image) = piece.get_display()
            else { continue };

        let mut element = commands.spawn((

            Sprite {
                image: asset_server.load(
                    format!("textures/pieces/{}.png", image).as_str()
                ),
                custom_size: Some(Vec2::new(75., 75.)),
                ..default()
            },

            Transform::from_xyz(coords.x, coords.y, 1.),

            IdleAnimator::new(),

            new_animator(
                TransformScaleLens {
                    start: Vec3::splat(1.0),
                    end: Vec3::splat(1.25)
                },
                0.3,
                EaseMethod::EaseFunction(EaseFunction::CubicOut),
                false,
                AnimationLoop::one_run()
            ),

            PieceComponent(piece.clone())

        ));

        element.observe(__on_click);
        element.observe(__on_move_animation_ended);

    }
}

pub fn __on_click(
    input: Trigger<Pointer<Click>>,
    mut query: Query<(&PieceComponent, &mut Animator<Transform>)>,
    mut game: ResMut<StandardGame>,
    targets: Query<Entity, With<TargetComponent>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let Ok((piece, mut animator)) = query.get_mut(input.entity())
        else { return };
    
    let (color, id) = piece.0.get_data();

    if game.animation_playing { return };
    if game.current_turn != *color { return };

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
                    &game.board.get_piece_by_id(*id).unwrap()
                ),
                &game
            );
        
            if !can_move { continue };

            /*todo!("
                Remove all 'target' instances and create them using the
                function BoardComponent::get_index_coords to create new
                'targets' on those positions and add them a click trigger.
            ");*/

            let board_pos = BoardComponent::get_index_coords(target_pos);

            let mut target = commands.spawn((

                Sprite {
                    image: asset_server.load("textures/target.png"),
                    custom_size: Some(Vec2::new(75., 75.)),
                    ..default()
                },

                Transform::from_xyz(board_pos.x, board_pos.y, 2.),

                TargetComponent

            ));

            target.observe(__on_click_target);
        }

        if !second_click {
            animator.set_speed(1.);

            let tween = animator.tweenable_mut();
            tween.set_progress(0.00001);
        }
        
        for (_, mut animator) in query.iter_mut() {
            let tween = animator.tweenable_mut();
            if tween.progress() == 0.00001 { continue };

            tween.rewind();
            animator.set_speed(0.);
        }
    }
}

fn __on_click_target(
    input: Trigger<Pointer<Click>>,
    mut game: ResMut<StandardGame>,
    mut query: Query<(Entity, &PieceComponent, &Transform, &mut Animator<Transform>)>,
    targets: Query<(Entity, &Transform), With<TargetComponent>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let Some(piece) = game.selected_piece
        else { return };

    let Ok((_, target_transform)) = targets.get(input.entity())
        else { return };

    for (entity, _) in targets.iter() {
        commands.entity(entity).despawn();
    }

    let target_pos = BoardComponent::get_coords_index(target_transform.translation);

    for (
        entity,
        component,
        transform,
        mut click_animator
    ) in query.iter_mut() {
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

        let tween = click_animator.tweenable_mut();
        
        tween.rewind();
        click_animator.set_speed(0.);

        let mut piece_entity = commands.entity(entity);
        piece_entity.insert(
            PieceMoveAnimator::new(
                transform.translation,
                Vec3::new(
                    target_transform.translation.x,
                    target_transform.translation.y,
                    1.
                )
            )
        );

        let target_board_pos = BoardComponent::get_coords_index(target_transform.translation);
        let _target_piece = game.board.get(target_board_pos.0, target_board_pos.1);

        let sound: Handle<AudioSource> = asset_server.load("sounds/piece_move.mp3");
        commands.spawn(AudioPlayer(sound));

        game.selected_piece = None;
        game.animation_playing = true;
    }
}

fn __on_move_animation_ended(
    input: Trigger<AnimationCompleted<PieceMoveAnimator>>,
    mut game: ResMut<StandardGame>,
    query: Query<(Entity, &PieceComponent, &Transform)>,
    mut commands: Commands
) {
    let Ok((entity, piece_component, transform)) = query.get(input.entity())
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

        let (curr_color, curr_id) = piece_component.0.get_data();

        let (_, target_id) = game.board.get(target_x, target_y).get_data();

        (curr_x, curr_y, target_x, target_y, *curr_color, *curr_id, *target_id)
    };

    let mut moved_piece = piece_component.0;
    moved_piece.set_id(curr_id);

    game.board.set(target_x, target_y, moved_piece);
    game.board.set(curr_x, curr_y, Piece::None(target_id));

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

// TODO: Create capture sound, delete piece captured...