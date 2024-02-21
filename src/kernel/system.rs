use crate::println;

pub struct LightningBolt {}

impl LightningBolt {

    fn start(&self) {
        println!("[LightningBolt] Booting kernel...");
        println!("[LightningBolt] Kernel booted!");
    }

}

pub static LIGHTNING_BOLT: LightningBolt = LightningBolt {};

pub fn start() {
    LIGHTNING_BOLT.start();
}