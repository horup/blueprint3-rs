use bevy::prelude::*;
use crate::{Bot, GamePiece, HudText, NewGameEvent, Player, ThingBuilder, ThingType, Tile, Tilemap, resources::{Game, GameState}};


fn tick(game:&mut ResMut<Game>, hud_texts:&mut Query<(&mut Text, &HudText)>, time:&Res<Time>) {
    if game.state == GameState::Loading {
        //hud_texts.for
    }
}

pub fn game_system(mut game:ResMut<Game>, game_pieces:Query<(Entity, &GamePiece)>, mut hud_texts:Query<(&mut Text, &HudText)>, mut commands: Commands, mut new_game_reader:EventReader<NewGameEvent>, time:Res<Time>) {
    tick(&mut game, &mut hud_texts, &time);
    /*for e in new_game_reader.iter() {
        // cleanup existing entities
        game_pieces.for_each_mut(|e| {
            let mut e = commands.entity(e.0);
            e.despawn_recursive();
        });

        // create camera
        commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GamePiece::default());

        
        // create tilemap
        let size = e.map_size;
        let mut tilemap = Tilemap::new(size, 4, "tiles.png");
        for y in 0..size {
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, 0, y);
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, size-1, y);
        }

        for x in 0..size {
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, x, 0);
            tilemap.set_tile(Tile {
                index:1,
                solid:true,
                ..Default::default()
            }, x, size - 1);
        }

        for y in 0..size {
            for x in 0..size {
                if x % 5 == 0 {
                    if y % 5 == 0 {
                        tilemap.set_tile(Tile {
                            index:1,
                            solid:true,
                            ..Default::default()
                        }, x, y);
                    }
                }
            }
        }

        commands.spawn().insert(tilemap);

        // spawn player
        commands.spawn().insert(ThingBuilder {
            translation:Vec3::new(2.5, 2.5, 0.0),
            rotation:Quat::default(),
            thing_type:ThingType::Tank,
            ..Default::default()
        })
        .insert(Player::default());

        let mut spawn_bot = |x, y| {
            commands.spawn().insert(ThingBuilder {
                translation:Vec3::new(x, y, 0.0),
                rotation:Quat::default(),
                thing_type:ThingType::Tank,
                ..Default::default()
            })
            .insert(Bot::default());
        };

        // spawn bot
        spawn_bot(size as f32 - 2.5, size as f32 - 2.5);
        spawn_bot(2.5, size as f32 - 2.5);
        spawn_bot(size as f32 - 2.5, 2.5);
    }*/
}
 
