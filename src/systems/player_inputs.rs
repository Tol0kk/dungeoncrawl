use macroquad::prelude::KeyCode;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_inputs(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<KeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = key {
        dbg!(&key);
        let delta = match key {
            KeyCode::Left => Point::new(-1, 0),
            KeyCode::Right => Point::new(1, 0),
            KeyCode::Up => Point::new(0, -1),
            KeyCode::Down => Point::new(0, 1),
            KeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .map(|(entity, pos)| (*entity, *pos))
                    .next()
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_, _, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _, _)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));

                        if let Ok(e) = ecs.entry_ref(*entity) {
                            if e.get_component::<Weapon>().is_ok() {
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, c, _)| c.0 == player)
                                    .for_each(|(e, _, _)| commands.remove(*e))
                            }
                        }
                    });
                Point::zero()
            }
            KeyCode::F1 => use_item(0, ecs, commands),
            KeyCode::F2 => use_item(1, ecs, commands),
            KeyCode::F3 => use_item(2, ecs, commands),
            KeyCode::F4 => use_item(3, ecs, commands),
            KeyCode::F5 => use_item(4, ecs, commands),
            KeyCode::F6 => use_item(5, ecs, commands),
            KeyCode::F7 => use_item(6, ecs, commands),
            KeyCode::F8 => use_item(7, ecs, commands),
            KeyCode::F9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        };
        *turn_state = TurnState::PlayerTurn;
    }

    fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
        let player_entity = <Entity>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .copied()
            .next()
            .unwrap();
        let item_entity = <(Entity, &Carried)>::query()
            .filter(component::<Item>())
            .iter(ecs)
            .filter(|(_, carried)| carried.0 == player_entity)
            .enumerate()
            .filter(|(item_count, _)| *item_count == n)
            .map(|(_, (item_entity, _))| *item_entity)
            .next();

        if let Some(item_entity) = item_entity {
            commands.push((
                (),
                ActivateItem {
                    used_by: player_entity,
                    item: item_entity,
                },
            ));
        }
        Point::zero()
    }
}
