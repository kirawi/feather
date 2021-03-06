use bytemuck::{Pod, Zeroable};
/// Marker component for armor stand entities.
///
/// # Example
/// A system that queries for all armor stands:
/// ```no_run
/// use quill::{Game, Position, entities::ArmorStand};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &ArmorStand)>() {
///         println!("Found a armor stand with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct ArmorStand;

pod_component_impl!(ArmorStand);
