use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    ui::UiTransform,
};

#[derive(Default, SystemDesc)]
pub struct HUDSystem;

impl<'a> System<'a> for HUDSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, UiTransform>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Health>,
    );

    fn run(&mut self, (entities, mut ui_transform, players, healths): Self::SystemData) {
        let health_entity = (&entities, &ui_transform)
            .join()
            .find(|x| x.1.id == "health")
            .map(|x| x.0)
            .unwrap();

        if let Some(health_display) = ui_transform.get_mut(health_entity) {
            for (player, health) in (&players, &healths).join() {
                health_display.width = health.current as f32 / health.max as f32 * 0.8;
            }
        }
    }
}
