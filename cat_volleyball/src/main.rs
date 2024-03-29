use bevy::prelude::*;
use rand::Rng;

const ARENA_WIDTH: f32 = 200.0;
const ARENA_HEIGHT: f32 = 200.0;
const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH: f32 = 22.0;
const PLAYER_SPEED: f32 = 60.0;
const BALL_VELOCITY_X: f32 = 30.0;
const BALL_VELOCITY_Y: f32 = 0.0;
const BALL_RADIUS: f32 = 4.0;
pub const GRAVITY_ACCELERATION: f32 = -40.0;

pub const SCORE_FONT_SIZE: f32 = 20.0;

#[derive(Component)]
struct ScoreBoard {
    side: Side
}

#[derive(Component)]
pub struct Ball{
    pub velocity: Vec2,
    pub radius: f32,
    pub bounce: Handle<AudioSource>,
    pub score: Handle<AudioSource>,
}

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    side: Side,
}

#[derive(Resource)]
struct Score {
    left: usize,
    right: usize,
}

impl Side {
    // Get keycode for left
    fn go_left_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::A,
            Side::Right => KeyCode::Left,
        }
    }

    // Get keycode for move right
    fn go_right_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::D,
            Side::Right => KeyCode::Right,
        }
    }

    // Determine the permissible range of the cat
    fn range(&self) -> (f32, f32) {
        match self {
            Side::Left => (
                PLAYER_WIDTH / 2.0,
                ARENA_WIDTH / 2.0 - PLAYER_WIDTH / 2.0
            ),
            Side::Right => (
                ARENA_WIDTH / 2.0 + PLAYER_WIDTH / 2.0,
                ARENA_WIDTH - PLAYER_WIDTH / 2.0,
            ),
        }
    }
}

fn player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let left = if keyboard_input.pressed(
            player.side.go_left_key())
        {
            -1.0f32
        } else {
            0.0
        };
        let right = if keyboard_input.pressed(
            player.side.go_right_key())
        {
            1.0f32
        } else {
            0.0
        };
        let direction = left + right;
        let offset = direction * PLAYER_SPEED
            * time.raw_delta_seconds();

        // Apply movement deltas
        transform.translation.x += offset;
        let (left_limit, ring_limit) = player.side.range();
        transform.translation.x = transform.translation.x.clamp(
            left_limit, ring_limit);
    }
}

fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Ball, &mut Transform)>
) {
    for (mut ball, mut transform) in query.iter_mut() {
        // Apply movement deltas
        transform.translation.x += ball.velocity.x
            * time.raw_delta_seconds();
        transform.translation.y += (ball.velocity.y
            + time.raw_delta_seconds()
            * GRAVITY_ACCELERATION / 2.0)
            * time.raw_delta_seconds();
        ball.velocity.y += time.raw_delta_seconds()
            * GRAVITY_ACCELERATION;
    }
}

fn initialize_player(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    cat_sprite: usize,
    side: Side,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Player { side },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(cat_sprite),
            texture_atlas: atlas,
            transform: Transform::from_xyz(x,y,0.0),
            ..default()
        },
    ));
}

//// Initializes one ball in the middle-ish of the arena.
fn initialize_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas: Handle<TextureAtlas>,
    ball_sprite: usize,
){
    let bounce_audio = asset_server.load("audio/bounce.ogg");
    let score_audio = asset_server.load("audio/score.ogg");
    commands.spawn((
        Ball {
            velocity: Vec2::new(BALL_VELOCITY_X, BALL_VELOCITY_Y),
            radius: BALL_RADIUS,
            bounce: bounce_audio,
            score: score_audio,
        },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(ball_sprite),
            texture_atlas: atlas,
            transform: Transform::from_xyz(
                ARENA_WIDTH / 2.0,
                ARENA_HEIGHT / 2.0, 0.0),
            ..default()
        },
    ));
}

fn initialize_scoreboard(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    side: Side,
    x: f32,
) {
    commands.spawn((
        ScoreBoard { side },
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: SCORE_FONT_SIZE,
                color: Color::WHITE,
                font: asset_server.load("font/square.ttf"),
        })])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(25.0),
                    left: Val::Px(x),
                    ..default()
                },
                ..default()
            })
            .with_text_alignment(match side {
                Side::Left => TextAlignment::Left,
                Side::Right => TextAlignment::Right,
            }),
    ));
}

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
) {
    audio.play_with_settings(
        asset_server.load(
            "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg"
        ),
        PlaybackSettings::LOOP.with_volume(0.25),
    );
    let spritesheet = asset_server.load(
        "textures/spritesheet.png");
    let mut sprite_atlas = TextureAtlas::new_empty(
        spritesheet,
        Vec2::new(58.0, 34.0));

    let left_cat_corner = Vec2::new(11.0, 1.0);
    let right_cat_corner = Vec2::new(35.0, 1.0);
    let cat_size = Vec2::new(22.0, 32.0);

    let ball_corner = Vec2::new(1.0, 1.0);
    let ball_size = Vec2::new(8.0, 8.0);

    let ball_index =
        sprite_atlas.add_texture(Rect::from_corners(
            ball_corner,
            ball_corner + ball_size
        ));

    let left_cat_index = sprite_atlas.add_texture(
        Rect::from_corners(
            left_cat_corner,
            left_cat_corner + cat_size,
        )
    );
    let right_cat_index = sprite_atlas.add_texture(
        Rect::from_corners(
            right_cat_corner,
            right_cat_corner + cat_size,
        )
    );

    let texture_atlas_handle = texture_atlases.add(sprite_atlas);

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(
            ARENA_WIDTH/2.0,
            ARENA_HEIGHT/2.0,1.0),
        ..default()
    });

    initialize_ball(
        &mut  commands,
        &asset_server,
        texture_atlas_handle.clone(),
        ball_index,
    );

    initialize_player(
        &mut commands,
        texture_atlas_handle.clone(),
        left_cat_index,
        Side::Left,
        PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT/2.0,
    );

    initialize_player(
        &mut commands,
        texture_atlas_handle,
        right_cat_index,
        Side::Right,
        ARENA_HEIGHT - PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT/2.0,
    );

    initialize_scoreboard(
        &mut commands,
        &asset_server,
        Side::Left,
        ARENA_WIDTH / 2.0 - 25.0
    );

    initialize_scoreboard(
        &mut commands,
        &asset_server,
        Side::Right,
        ARENA_WIDTH / 2.0 + 25.0
    )

}

fn point_in_rect(
    x: f32,
    y: f32,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn bounce(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
    audio: Res<Audio>,
) {
    for (mut ball, ball_trans) in ball_query.iter_mut()
    {
        let ball_x = ball_trans.translation.x;
        let ball_y = ball_trans.translation.y;

        if ball_y >= (ARENA_HEIGHT - ball.radius)
            && ball.velocity.y > 0.0
        {
            audio.play(ball.bounce.clone()); // bounce sound added
            ball.velocity.y = -ball.velocity.y;
        } else if ball_x <= ball.radius && ball.velocity.x < 0.0 {
            audio.play(ball.bounce.clone()); // bounce sound added
            ball.velocity.x = -ball.velocity.x;
        } else if ball_x >= (ARENA_WIDTH - ball.radius)
            && ball.velocity.x > 0.0
        {
            audio.play(ball.bounce.clone()); // bounce sound added
            ball.velocity.x = -ball.velocity.x;
        }
        for (player, player_trans) in player_query.iter() {
            let player_x = player_trans.translation.x;
            let player_y = player_trans.translation.y;
            if point_in_rect(
                ball_x,
                ball_y,
                player_x - PLAYER_WIDTH / 2.0 - ball.radius,
                player_y - PLAYER_HEIGHT / 2.0 - ball.radius,
                player_x + PLAYER_WIDTH / 2.0 + ball.radius,
                player_y + PLAYER_HEIGHT / 2.0 + ball.radius,
            ) {
                if ball.velocity.y < 0.0 {
                    audio.play(ball.bounce.clone());
                    ball.velocity.y = -ball.velocity.y;

                    let mut rng = rand::thread_rng();
                    match player.side {
                        Side::Left => {
                            ball.velocity.x = ball.velocity.x.abs()
                                * rng.gen_range(0.6..1.4)
                        }
                        Side::Right => {
                            ball.velocity.x = -ball.velocity.x.abs()
                                * rng.gen_range(0.6..1.4)
                        }
                    }
                }
            }
        }
    }
}

fn scoring(
    mut query: Query<(&mut Ball, &mut Transform)>,
    mut score: ResMut<Score>,
    audio: Res<Audio>,
) {
    for ( mut ball, mut transform) in query.iter_mut() {
        let ball_x = transform.translation.x;
        let ball_y = transform.translation.y;

        if ball_y < ball.radius {
            audio.play(ball.score.clone());
            if ball_x <= ARENA_WIDTH / 2.0 {
                score.right += 1;
                ball.velocity.x = ball.velocity.x.abs();
            } else {
                score.left += 1;
                ball.velocity.x = -ball.velocity.x.abs();
            }
            transform.translation.x = ARENA_WIDTH / 2.0;
            transform.translation.y = ARENA_HEIGHT / 2.0;

            ball.velocity.y = 0.0;
        }
    }
}

fn score_display(
    score: Res<Score>,
    mut query: Query<(&mut Text, &ScoreBoard)>
) {
    for (mut text, scoreboard) in query.iter_mut() {
        text.sections[0].value = match scoreboard.side {
            Side::Left => score.left.to_string(),
            Side::Right => score.right.to_string(),
        };
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Volleyball".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb (0.0, 0.0, 0.0)))
        .insert_resource(Score { left: 0, right: 0})
        .add_startup_system(setup)
        .add_system(player)
        .add_system(bounce)
        .add_system(move_ball)
        .add_system(scoring)
        .add_system(score_display)
        .run();
}
