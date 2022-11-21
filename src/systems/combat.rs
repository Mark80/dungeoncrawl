use crate::prelude::*;

#[system]
#[read_component(WantToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(world)
        .map(|(entity, want_attack)| (*entity, want_attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = world
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = world
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            println!("Is player: {}", is_player);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}
