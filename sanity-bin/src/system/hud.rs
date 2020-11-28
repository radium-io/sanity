use amethyst::{
    derive::SystemDesc,
    ecs::{prelude::*, Entity},
    ui::UiTransform,
};

#[derive(Default, SystemDesc)]
pub struct HUDSystem {
    health_bar: Option<Entity>,
}

impl<'a> System<'a> for HUDSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, UiTransform>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Health>,
    );

    fn run(&mut self, (entities, mut ui_transform, players, healths): Self::SystemData) {
        if self.health_bar.is_none() {
            self.health_bar = (&entities, &ui_transform)
                .join()
                .find(|x| x.1.id == "health")
                .map(|x| x.0);
        }

        if self.health_bar.is_some() {
            if let Some(health_display) = ui_transform.get_mut(self.health_bar.unwrap()) {
                for (entity, player) in (&entities, &players).join() {
                    if let Some(health) = healths.get(entity) {
                        health_display.width = health.current as f32 / health.max as f32 * 0.8;
                    } else {
                        // dead
                        health_display.width = 0.;
                    }
                }
            }
        }
    }
}
