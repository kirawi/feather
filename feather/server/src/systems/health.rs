use crate::{
	ClientId,
	Server,
	Game,
};
use ecs::{EntityRef, SysResult, SystemExecutor};
use quill_common::components::Health;
use common::events::{DamageEvent, DamageType};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
	systems.group::<Server>()
		.add_system(damage_handler);
}

fn damage_handler(game: &mut Game, server: &mut Server) -> SysResult {
	// for (entity, (client_id, event, health)) in game.ecs.query::<(&ClientId, &DamageEvent, &mut Health)>().iter() {
	// 	match event.damage_type {
	// 		DamageType::FallDamage(_) => {},
	// 	}

	// 	if let Some(client) = server.clients.get(*client_id) {
	// 		health.deal_damage(1);
	// 		client.update_health(&health);
	// 	}
	// }


	if game.tick_count % 8 == 0 {
		for (player, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
			if let Some(client) = server.clients.get(*client_id) {
				health.deal_damage(1);
				client.update_health(&health);
			}
		}
	}
	
	Ok(())
}