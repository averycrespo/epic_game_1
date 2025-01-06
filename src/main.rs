use std::error::Error;
use bevy::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
    
    return Ok(());
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup,)
            ;
            // .add_systems(Update, print_names)
            // .add_systems(Update, normal_system);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (Person {
            name: "Player".to_string(),
        }, 
        Employed {
            job: Job::Police,
        })
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn normal_system() {
    println!("Worked");
}

// Required for bevy
#[derive(Component)]
pub struct Person{
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

pub enum Job{
    Police, 
    Firefighter,
}