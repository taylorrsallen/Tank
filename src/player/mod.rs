use crate::*;

use bevy::window::PrimaryWindow;

mod manager;
pub use manager::*;
mod splitscreen;
pub use splitscreen::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankPlayerPlugin;
impl Plugin for TankPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PrimaryPlayer>()
            .register_type::<Player>()
            .register_type::<PlayerMainCameraRef>()
            .register_type::<PlayerSelector>()
            .register_type::<PlayerController>()
            .register_type::<SplitscreenSettings>()
            .insert_resource(SplitscreenSettings::default())
            .add_systems(OnEnter(AppState::EngineInit), onsys_spawn_primary_player)
            .add_systems(PostUpdate, (
                sys_update_player_ids,
                sys_init_added_players,
                sys_update_changed_player_cameras,
                sys_update_primary_player_devices,
                sys_mark_splitscreen_changes,
                sys_update_resized_camera_viewports,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Inserted automatically on Player 0, marking that they cannot be despawned.
/// 
/// Receives all input devices not bound to other players.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PrimaryPlayer;

/// Marker component.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

impl Player {
    pub fn next_id(player_id_query: &Query<&Id, With<Player>>) -> u32 {
        let mut id = 0;
        let existing_ids: Vec<u32> = player_id_query.iter().map(|id| id.get()).collect();
        while existing_ids.contains(&id) { id += 1; }
        return id;
    }

    pub fn try_get_window_entity(
        main_camera_ref: &PlayerMainCameraRef,
        primary_window_query: &Query<Entity, With<PrimaryWindow>>,
        camera_query: &Query<&Camera>,
    ) -> Option<Entity> {
        let camera_entity = if let Some(entity) = *main_camera_ref.try_get() { entity } else { return None };
        let camera = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { return None };
        Some(Cameras::window_entity_from_camera(camera, &primary_window_query))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Can be Camera3d or Camera2d, as long as it's a camera.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerMainCameraRef(Option<Entity>);

impl PlayerMainCameraRef {
    pub fn new(camera: Option<Entity>) -> Self { Self { 0: camera } }
    pub fn try_get(&self) -> &Option<Entity> { &self.0 }
    pub fn set(&mut self, camera: Option<Entity>) { self.0 = camera }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A list of the [GuiData] entities this [Player] is viewing
#[derive(Component, Default, Deref, DerefMut, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct PlayerGuiViewer(pub Vec<Entity>);

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Collects entities the player has selected.
/// 
/// TODO: Store fundementally different selections separately? (RTS Units vs. Menu elements)
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerSelector {
    pub selected_entities: Vec<Entity>,
}

impl PlayerSelector {
    pub fn new(selected_entities: Vec<Entity>) -> Self {
        Self { selected_entities }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Send inputs to a single entity.
/// 
/// It's up to the game to implement how inputs are sent by reading [InputActions] alongside this.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerController {
    pub controlled_entity: Option<Entity>,
}

impl PlayerController {
    pub fn new(controlled_entity: Option<Entity>) -> Self {
        Self { controlled_entity }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect, PartialEq, Eq, Clone, Copy)]
#[reflect(Component)]
pub enum PlayerLookState {
    /// Look inputs rotate the camera
    #[default]
    Camera,
    /// Look inputs move the cursor
    Cursor,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Players;
impl Players {
    pub fn spawn_default_player(commands: &mut Commands) -> Entity {
        let main_camera_entity = commands.spawn(MainCameraBundle::default()).id();
        commands.spawn(PlayerBundle::new(
                Some(main_camera_entity),
                None,
                None,
                &vec![],
            ))
            .id()
    }

    pub fn despawn_player(
        player_entity: Entity,
        main_camera_ref_query: &Query<&PlayerMainCameraRef>,
        commands: &mut Commands,
    ) {
        if let Ok(main_camera_ref) = main_camera_ref_query.get(player_entity) {
            if let Some(main_camera) = main_camera_ref.try_get() { commands.entity(*main_camera).despawn_recursive(); }
        }
    
        commands.entity(player_entity).despawn_recursive();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_main_camera_ref: PlayerMainCameraRef,
    pub player_look_state: PlayerLookState,
    pub player_controller: PlayerController,
    pub input_actions: InputActions,
    pub input_action_bindings: InputActionBindings,
    pub input_device_receiver: InputDeviceReceiver,
    pub raw_button_input: RawButtonInput,
    pub raw_axis_input: RawAxisInput,
    pub id: Id,
}

impl PlayerBundle {
    pub fn new(
        main_camera: Option<Entity>,
        controlled_entity: Option<Entity>,
        input_action_bindings: Option<InputActionBindings>,
        input_devices: &[InputDevice],
    ) -> Self {
        Self {
            player_main_camera_ref: PlayerMainCameraRef::new(main_camera),
            player_controller: PlayerController::new(controlled_entity),
            input_action_bindings: if let Some(bindings) = input_action_bindings { bindings } else { InputActionBindings::default() },
            input_device_receiver: InputDeviceReceiver::from_devices(&input_devices),
            ..default()
        }
    }
}