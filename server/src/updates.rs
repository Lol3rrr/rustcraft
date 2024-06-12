mod water;
pub use water::WaterUpdates;

pub trait Update {
    fn update(&mut self, region: &mut crate::world::Region);
}
