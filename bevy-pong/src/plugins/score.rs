use crate::{player::Player, FONT_SIZE};

use bevy::{prelude::*, utils::HashMap};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<GainPoint>()
            .add_systems(PostUpdate, score)
            .add_systems(Startup, spawn_score);
    }
}

#[derive(Event)]
pub struct GainPoint(pub Player);

#[derive(Default, Resource)]
struct Score(HashMap<Player, i32>);

fn score(
    mut event_reader: EventReader<GainPoint>,
    mut score_text: Query<(&mut Text, &Player)>,
    mut score: ResMut<Score>,
) {
    for event in event_reader.read() {
        let player = event.0;

        *score.0.entry(player).or_default() += 1;
        let score = score.0.get(&player).cloned().unwrap_or_default();

        for (mut text, owner) in &mut score_text {
            if *owner != player {
                continue;
            }

            text.0 = score.to_string();
            break;
        }
    }
}

fn spawn_score(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::ZERO,
            width: Val::Percent(30.0),
            height: Val::Percent(20.0),
            align_content: AlignContent::Stretch,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                Player::Player1,
            ));

            parent.spawn((
                Text::new("|"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                Player::Player2,
            ));
        });
}
