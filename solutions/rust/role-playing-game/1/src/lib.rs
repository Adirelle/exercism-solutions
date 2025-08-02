// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn revive(&self) -> Option<Player> {
        self.is_dead().then_some(Player {
            health: 100,
            mana: self.revive_mana(),
            level: self.level,
        })
    }

    fn revive_mana(&self) -> Option<u32> {
        (self.level >= 10).then_some(100)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health -= mana_cost.min(self.health);
                0
            }
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            _ => 0,
        }
    }
}
