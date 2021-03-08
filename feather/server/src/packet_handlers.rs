use base::{Position, Text};
use common::{chat::ChatKind, Game};
use ecs::{Entity, EntityRef, SysResult};
use interaction::{
    handle_held_item_change, handle_interact_entity, handle_player_block_placement,
    handle_player_digging,
};
use protocol::{
    packets::{
        client,
        server::{Animation, Hand},
    },
    ClientPlayPacket,
};
use quill_common::components::Name;

use crate::{NetworkId, Server};

use self::health::handle_client_status;

mod interaction;
pub mod inventory;
mod movement;
mod health;

/// Handles a packet received from a client.
pub fn handle_packet(
    game: &mut Game,
    server: &mut Server,
    player_id: Entity,
    packet: ClientPlayPacket,
) -> SysResult {
    let player = game.ecs.entity(player_id)?;
    match packet {
        ClientPlayPacket::PlayerPosition(packet) => {
            movement::handle_player_position(server, player, packet)
        }
        ClientPlayPacket::PlayerPositionAndRotation(packet) => {
            movement::handle_player_position_and_rotation(server, player, packet)
        }
        ClientPlayPacket::PlayerRotation(packet) => {
            movement::handle_player_rotation(server, player, packet)
        }
        ClientPlayPacket::PlayerMovement(packet) => {
            movement::handle_player_movement(player, packet)
        }

        ClientPlayPacket::Animation(packet) => handle_animation(server, player, packet),

        ClientPlayPacket::ChatMessage(packet) => handle_chat_message(game, player, packet),

        ClientPlayPacket::PlayerDigging(packet) => handle_player_digging(game, packet, player_id),

        ClientPlayPacket::CreativeInventoryAction(packet) => {
            inventory::handle_creative_inventory_action(player, packet)
        }
        ClientPlayPacket::ClickWindow(packet) => {
            inventory::handle_click_window(server, player, packet)
        }

        ClientPlayPacket::PlayerBlockPlacement(packet) => {
            handle_player_block_placement(game, server, packet, player_id)
        }

        ClientPlayPacket::HeldItemChange(packet) => handle_held_item_change(player, packet),
        ClientPlayPacket::InteractEntity(packet) => {
            handle_interact_entity(game, server, packet, player_id)
        }

        ClientPlayPacket::ClientStatus(packet) => {
            handle_client_status(game, server, player_id, packet)
        }

        ClientPlayPacket::TeleportConfirm(_)
        | ClientPlayPacket::QueryBlockNbt(_)
        | ClientPlayPacket::SetDifficulty(_)
        | ClientPlayPacket::ClientSettings(_)
        | ClientPlayPacket::TabComplete(_)
        | ClientPlayPacket::WindowConfirmation(_)
        | ClientPlayPacket::ClickWindowButton(_)
        | ClientPlayPacket::CloseWindow(_)
        | ClientPlayPacket::PluginMessage(_)
        | ClientPlayPacket::EditBook(_)
        | ClientPlayPacket::QueryEntityNbt(_)
        | ClientPlayPacket::GenerateStructure(_)
        | ClientPlayPacket::KeepAlive(_)
        | ClientPlayPacket::LockDifficulty(_)
        | ClientPlayPacket::VehicleMove(_)
        | ClientPlayPacket::SteerBoat(_)
        | ClientPlayPacket::PickItem(_)
        | ClientPlayPacket::CraftRecipeRequest(_)
        | ClientPlayPacket::PlayerAbilities(_)
        | ClientPlayPacket::EntityAction(_)
        | ClientPlayPacket::SteerVehicle(_)
        | ClientPlayPacket::SetDisplayedRecipe(_)
        | ClientPlayPacket::SetRecipeBookState(_)
        | ClientPlayPacket::NameItem(_)
        | ClientPlayPacket::ResourcePackStatus(_)
        | ClientPlayPacket::AdvancementTab(_)
        | ClientPlayPacket::SelectTrade(_)
        | ClientPlayPacket::SetBeaconEffect(_)
        | ClientPlayPacket::UpdateCommandBlock(_)
        | ClientPlayPacket::UpdateCommandBlockMinecart(_)
        | ClientPlayPacket::UpdateJigsawBlock(_)
        | ClientPlayPacket::UpdateStructureBlock(_)
        | ClientPlayPacket::UpdateSign(_)
        | ClientPlayPacket::Spectate(_)
        | ClientPlayPacket::UseItem(_) => Ok(()),
    }
}

fn handle_animation(
    server: &mut Server,
    player: EntityRef,
    packet: client::Animation,
) -> SysResult {
    let pos = *player.get::<Position>()?;
    let network_id = *player.get::<NetworkId>()?;

    let animation = match packet.hand {
        Hand::Main => Animation::SwingMainArm,
        Hand::Off => Animation::SwingOffhand,
    };

    server.broadcast_nearby_with(pos, |client| {
        client.send_entity_animation(network_id, animation.clone())
    });
    Ok(())
}

fn handle_chat_message(game: &Game, player: EntityRef, packet: client::ChatMessage) -> SysResult {
    let name = player.get::<Name>()?;
    let message = Text::translate_with("chat.type.text", vec![name.to_string(), packet.message]);
    game.broadcast_chat(ChatKind::PlayerChat, message);
    Ok(())
}