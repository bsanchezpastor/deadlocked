use std::time::{Duration, Instant};

use glam::{Vec2, Vec3};
use rand::rng;
use rand_distr::{Distribution, Normal};
use shared::{Bones, WeaponClass};

use crate::{
    config::{Config, aim::TriggerbotConfig},
    cs2::{
        CS2,
        entity::player::{Player, PlayerHitboxHit},
    },
    math::angles_to_fov,
    os::uinput::InputDevice,
};

#[derive(Default)]
pub struct Triggerbot {
    shot_start: Option<Instant>,
    shot_end: Option<Instant>,
    target: Option<Player>,
    mouse_down: bool,
    shot_duration: Duration,
    pub active: bool,
    magnet_active: bool,
}

impl CS2 {
    pub fn triggerbot(&mut self, config: &Config, mouse: &mut InputDevice) -> bool {
        let hotkey = config.aim.triggerbot_hotkey;
        let (enabled, mode, settings) = {
            let settings = self.triggerbot_config(config);
            (settings.enabled, settings.mode, settings.clone())
        };
        if !enabled || !Self::check_hotkey(&self.input, mode, hotkey, &mut self.trigger.active) {
            if !enabled {
                self.trigger.active = false;
            }
            self.cancel_triggerbot(mouse);
            return false;
        }

        let Some(local_player) = Player::local_player(self) else {
            self.cancel_triggerbot(mouse);
            return false;
        };
        if !local_player.is_valid(self)
            || settings.flash_check && local_player.is_flashed(self)
            || settings.scope_check
                && local_player.weapon_class(self) == WeaponClass::Sniper
                && !local_player.is_scoped(self)
            || settings.velocity_check
                && local_player.velocity(self).length() > settings.velocity_threshold
            || local_player.clip_ammo(self) <= 0
        {
            self.cancel_triggerbot(mouse);
            return false;
        }

        if settings.magnet_enabled {
            let aim_config = self.aimbot_config(config).clone();
            let Some(target) = self.active_target() else {
                self.cancel_triggerbot(mouse);
                return false;
            };
            if !self.aimbot_weapon_allowed(&local_player)
                || aim_config.flash_check && local_player.is_flashed(self)
            {
                self.cancel_triggerbot(mouse);
                return false;
            }
            let locked = local_player
                .crosshair_entity(self)
                .is_some_and(|player| player.pawn == target.pawn);
            if self
                .trigger
                .target
                .is_some_and(|previous| previous != target)
            {
                self.cancel_triggerbot(mouse);
            }
            self.trigger.magnet_active = true;
            if locked {
                if self.trigger.shot_start.is_none() && self.trigger.shot_end.is_none() {
                    self.trigger.target = Some(target);
                    self.trigger.shot_duration = Duration::from_millis(settings.shot_duration);
                    self.trigger.shot_start = Some(Instant::now());
                }
            } else {
                if self.trigger.shot_start.is_some() || self.trigger.shot_end.is_some() {
                    self.cancel_triggerbot(mouse);
                    self.trigger.magnet_active = true;
                }
                let max_fov = self.aimbot_max_fov(&aim_config, self.target.distance);
                self.apply_aimbot_step(
                    &aim_config,
                    &local_player,
                    self.target.angle,
                    max_fov,
                    mouse,
                );
                self.recoil.previous = local_player.aim_punch(self);
            }
            return true;
        }
        if self.trigger.magnet_active {
            self.reset_aimbot_state();
            self.trigger.magnet_active = false;
        }

        let target = if settings.penetration_check {
            self.penetrable_target(&local_player, &settings)
        } else {
            local_player
                .crosshair_entity(self)
                .filter(|&target| self.target_is_enemy(&local_player, target))
        };
        let Some(target) = target else {
            self.cancel_triggerbot(mouse);
            return false;
        };

        if settings.head_only
            && !settings.penetration_check
            && !self.crosshair_on_head(&local_player, &target)
        {
            self.cancel_triggerbot(mouse);
            return false;
        }

        if self
            .trigger
            .target
            .is_some_and(|previous| previous != target)
        {
            self.cancel_triggerbot(mouse);
        }
        self.trigger.target = Some(target);

        if self.trigger.shot_start.is_some() || self.trigger.shot_end.is_some() {
            return false;
        }

        let delay = sample_delay(&settings);
        self.trigger.shot_duration = Duration::from_millis(settings.shot_duration);
        self.trigger.shot_start = Some(Instant::now() + delay);
        false
    }

    pub fn triggerbot_shoot(&mut self, mouse: &mut InputDevice) {
        let now = Instant::now();
        if self
            .trigger
            .shot_start
            .is_some_and(|shot_start| now >= shot_start)
        {
            mouse.left_press();
            self.trigger.mouse_down = true;
            self.trigger.shot_start = None;
            self.trigger.shot_end = Some(now + self.trigger.shot_duration);
        }

        if self
            .trigger
            .shot_end
            .is_some_and(|shot_end| now >= shot_end)
        {
            if self.trigger.mouse_down {
                mouse.left_release();
            }
            self.trigger.mouse_down = false;
            self.trigger.shot_end = None;
        }
    }

    fn cancel_triggerbot(&mut self, mouse: &mut InputDevice) {
        if self.trigger.magnet_active {
            self.reset_aimbot_state();
            self.trigger.magnet_active = false;
        }
        if self.trigger.mouse_down {
            mouse.left_release();
        }
        self.trigger.shot_start = None;
        self.trigger.shot_end = None;
        self.trigger.target = None;
        self.trigger.mouse_down = false;
        self.trigger.shot_duration = Duration::ZERO;
    }

    fn target_is_enemy(&self, local_player: &Player, target: Player) -> bool {
        target.is_valid(self) && (self.is_ffa() || target.team(self) != local_player.team(self))
    }

    fn crosshair_on_head(&self, local_player: &Player, target: &Player) -> bool {
        let head = target.bone_position(self, Bones::Head.u64());
        let target_angle = self.angle_to_target(local_player, &head, &Vec2::ZERO);
        let view_angles = local_player.view_angles(self);
        let fov = angles_to_fov(&view_angles, &target_angle);
        let distance = (local_player.position(self) - target.position(self)).length();
        let head_radius_fov = 3.5 / distance * 100.0;
        fov <= head_radius_fov
    }

    fn penetrable_target(
        &self,
        local_player: &Player,
        settings: &TriggerbotConfig,
    ) -> Option<Player> {
        let bvh = self.bvh.as_ref()?;
        let vdata = local_player.weapon_entity(self)?.vdata(self)?;
        let origin = local_player.eye_position(self);
        let view_angles = local_player.view_angles(self);

        let aim_punch = if local_player.shots_fired(self) > 0 {
            local_player.aim_punch(self) * 2.0
        } else {
            glam::Vec2::ZERO
        };
        let shoot_angles = view_angles + aim_punch;

        let pitch = shoot_angles.x.to_radians();
        let yaw = shoot_angles.y.to_radians();
        let direction = Vec3::new(
            pitch.cos() * yaw.cos(),
            pitch.cos() * yaw.sin(),
            -pitch.sin(),
        );

        let mut candidates = Vec::<(f32, PlayerHitboxHit, Player)>::new();
        for &target in &self.players {
            if !self.target_is_enemy(local_player, target) {
                continue;
            }
            let Some(hit) = target.ray_hitbox(self, origin, direction) else {
                continue;
            };
            if hit.distance <= vdata.range && (!settings.head_only || hit.group_id == 1) {
                candidates.push((hit.distance, hit, target));
            }
        }
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));

        for (distance, hit, target) in candidates {
            let hit_position = origin + direction * distance;
            let hitgroup_modifier = match hit.group_id {
                1 => vdata.headshot_multiplier, // Head
                3 => 1.25,                      // Stomach
                6 | 7 => 0.75,                  // Legs
                _ => 1.0,                       // Chest / Arms / other
            };
            if bvh
                .estimate_penetration_damage(
                    origin,
                    hit_position,
                    vdata.damage as f32,
                    vdata.penetration,
                    vdata.range,
                    vdata.range_modifier,
                )
                .is_some_and(|damage| {
                    damage * hitgroup_modifier >= settings.minimum_penetration_damage as f32
                })
            {
                return Some(target);
            }
        }
        None
    }
}

fn sample_delay(config: &TriggerbotConfig) -> Duration {
    let start = *config.delay.start().min(config.delay.end());
    let end = *config.delay.start().max(config.delay.end());
    let mean = (start as f64 + end as f64) * 0.5;
    let deviation = (end as f64 - start as f64) * 0.5;
    let millis = if deviation == 0.0 {
        mean
    } else {
        Normal::new(mean, deviation)
            .map(|normal| normal.sample(&mut rng()))
            .unwrap_or(mean)
    }
    .max(0.0) as u64;
    Duration::from_millis(millis)
}
