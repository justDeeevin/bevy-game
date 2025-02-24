use bevy::prelude::*;
use std::sync::Arc;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello, (update_people, greet_people).chain()))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Devin".into())));
    commands.spawn((Person, Name("David".into())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0 = "Devin".into();
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn hello() {
    println!("droddyrox");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(Arc<str>);
