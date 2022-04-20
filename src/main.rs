mod adored;
mod animal;
mod human;
mod kitty;
use animal::Animal;
use bevy::{math::const_vec3, prelude::*, sprite::collide_aabb::collide};
use human::{boy::Boy, girl::Girl, Human};
use kitty::Kitty;
const CAT_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BOY_COLOR: Color = Color::rgb(0.5, 0.3, 0.7);
const GIRL_COLOR: Color = Color::rgb(0.3, 0.5, 0.7);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

const CAT_SIZE: Vec3 = const_vec3!([40.0, 40.0, 0.0]);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
enum Control {
    Boy,
    Girl,
}
struct Status {
    controler: Control,
    catposition: Vec3,
    adoredman: Option<Control>,
}
fn prepare_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn()
        .insert(Kitty::default())
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 20.0, 0.0),
                scale: CAT_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: CAT_COLOR,
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(Boy::born("Mike".to_string()))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-100.0, 0.0, 0.0),
                scale: CAT_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BOY_COLOR,
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(Girl::born("ILISABETH".to_string()))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                scale: CAT_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: GIRL_COLOR,
                ..default()
            },
            ..default()
        });
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Adopted: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 50.0,
                        ..default() //color: SCORE_COLOR,
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
//fn getolder(mut query: Query<&mut Kitty>) {
//    let mut cat = query.single_mut();
//    cat.grow();
//    println!("{}", cat.age);
//}
fn update_scoreboard(cat: Query<&Kitty>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    let cat = cat.single();
    if let Some(admin) = &cat.admin {
        text.sections[1].value = format!("Kitty' admin is {}", admin)
    } else {
        text.sections[1].value = "Kitty is disadored".to_string();
    }
}
fn move_player_boy(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<Status>,
    //mut query_girl: Query<&mut Transform, With<Girl>>,
    mut query_boy: Query<&mut Transform, With<Boy>>,
) {
    let mut boy_transform = query_boy.single_mut();
    match state.controler {
        Control::Boy => {
            if keyboard_input.pressed(KeyCode::Up) {
                boy_transform.translation.y += 10.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                boy_transform.translation.y -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                boy_transform.translation.x -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                boy_transform.translation.x += 10.0;
            }
        }
        Control::Girl => {}
    }
    if let Some(Control::Boy) = state.adoredman {
        state.catposition = boy_transform.translation.clone();
        state.catposition.x -= 50.0;
    }
}
fn switch_player(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<Status>) {
    if keyboard_input.pressed(KeyCode::B) {
        state.controler = Control::Boy;
    }
    if keyboard_input.pressed(KeyCode::G) {
        state.controler = Control::Girl;
    }
}
fn move_kitty(state: Res<Status>, mut query: Query<&mut Transform, With<Kitty>>) {
    let mut cat = query.single_mut();
    cat.translation = state.catposition;
}
fn adore_kitty(
    mut state: ResMut<Status>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Kitty, &Transform), With<Kitty>>,
    mut query_man: Query<(&mut Boy, &Transform), With<Boy>>,
    mut query_girl: Query<(&mut Girl, &Transform), With<Girl>>,
) {
    let (mut cat, transform_cat) = query.single_mut();
    let cat_size = transform_cat.scale.truncate();
    let (mut boy, transform_boy) = query_man.single_mut();
    let (mut girl, transform_girl) = query_girl.single_mut();
    if keyboard_input.pressed(KeyCode::A) && state.adoredman.is_none() {
        match state.controler {
            Control::Boy => {
                let boy_size = transform_boy.scale.truncate();
                if collide(
                    transform_boy.translation,
                    boy_size,
                    transform_cat.translation,
                    cat_size,
                )
                .is_some()
                {
                    boy.adorepet(&mut *cat);
                    cat.grow();
                    state.adoredman = Some(Control::Boy);
                }
            }
            Control::Girl => {
                let girl_size = transform_girl.scale.truncate();
                if collide(
                    transform_girl.translation,
                    girl_size,
                    transform_cat.translation,
                    cat_size,
                )
                .is_some()
                {
                    girl.adorepet(&mut *cat);
                    cat.grow();
                    state.adoredman = Some(Control::Girl);
                }
            }
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        match (&state.controler, &state.adoredman) {
            (Control::Boy, Some(Control::Boy)) => {
                boy.disadorepet(&mut *cat);
                state.adoredman = None;
            }
            (Control::Girl, Some(Control::Girl)) => {
                girl.disadorepet(&mut *cat);
                state.adoredman = None;
            }
            (_, _) => {}
        }
    }
}
fn move_player_girl(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<Status>,
    mut query_girl: Query<&mut Transform, With<Girl>>,
    //mut query_boy: Query<&mut Transform, With<Boy>>
) {
    let mut girl_transform = query_girl.single_mut();

    match state.controler {
        Control::Boy => {}
        Control::Girl => {
            if keyboard_input.pressed(KeyCode::Up) {
                girl_transform.translation.y += 10.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                girl_transform.translation.y -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                girl_transform.translation.x -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                girl_transform.translation.x += 10.0;
            }
        }
    }
    if let Some(Control::Girl) = state.adoredman {
        state.catposition = girl_transform.translation.clone();
        state.catposition.x -= 50.0;
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Status {
            controler: Control::Boy,
            catposition: Vec3::new(0.0, 20.0, 0.0),
            adoredman: None,
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(prepare_scene)
        .add_system(switch_player)
        .add_system(move_player_girl)
        .add_system(move_player_boy)
        .add_system(adore_kitty)
        .add_system(move_kitty)
        //.add_system(getolder)
        .add_system(update_scoreboard)
        .run();
}
