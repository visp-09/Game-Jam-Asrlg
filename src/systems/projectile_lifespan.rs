use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system]
#[write_component(Projectile)]
pub fn projectile_lifespan(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut projectiles = <(Entity, &mut Projectile)>::query().filter(component::<Projectile>());
    projectiles.iter_mut(ecs).for_each(|(entity, proj)| {
        proj.lifetime = proj.lifetime - 1;
        if proj.lifetime == 0 {
            commands.add_component(*entity, Dead);
        }
    });
}
