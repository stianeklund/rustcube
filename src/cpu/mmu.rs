
use super::machine_status::MachineStatus;

#[derive(Default, Clone, Copy, Debug)]
struct Bat {
   bepi: u16,
   bl: u16,
   vs: bool,
   vp: bool,
   brpn: u16,
   wimg: u8,
   pp: u8
}

#[derive(Debug)]
pub struct Mmu {
   dbat: [Bat; 4],
   ibat: [Bat; 4],
}

impl Mmu {

   pub fn new() -> Mmu {
      Mmu {
         dbat: [Bat::default(); 4],
         ibat: [Bat::default(); 4]
      }
   }

   pub fn write_ibatu(&mut self, index: usize, value: u32) {
      let bat = &mut self.ibat[index];

      // FixMe: validate BAT value
      // MSRIR | MSRDR = 1
      // (Vs & ~MSRPR) | (Vp & MSRPR) = 1

      bat.bepi = ((value >> 17) & 0b111_1111_1111_1111) as u16;
      bat.bl   = ((value >> 2) & 0b111_1111_1111) as u16;
      bat.vs   = ((value >> 1) & 0b1) != 0;
      bat.vp   = (value & 0b1) != 0;
   }

   pub fn write_dbatu(&mut self, index: usize, value: u32) {
      let bat = &mut self.dbat[index];

      // FixMe: validate BAT value
      // MSRIR | MSRDR = 1
      // (Vs & ~MSRPR) | (Vp & MSRPR) = 1

      bat.bepi = ((value >> 17) & 0b111_1111_1111_1111) as u16;
      bat.bl   = ((value >> 2) & 0b111_1111_1111) as u16;
      bat.vs   = ((value >> 1) & 0b1) != 0;
      bat.vp   = (value & 0b1) != 0;
   }

   pub fn write_ibatl(&mut self, index: usize, value:u32) {
      let bat = &mut self.ibat[index];

      // FixMe: validate BAT value

      bat.brpn = (value >> 17 & 0b111_1111_1111_1111) as u16;
      bat.wimg = (value >> 3 & 0b1_1111) as u8;
      bat.pp   = (value & 0b11) as u8;
   }

   pub fn write_dbatl(&mut self, index: usize, value:u32) {
      let bat = &mut self.dbat[index];

      // FixMe: validate BAT value

      bat.brpn = (value >> 17 & 0b111_1111_1111_1111) as u16;
      bat.wimg = (value >> 3 & 0b1_1111) as u8;
      bat.pp   = (value & 0b11) as u8;
   }

   pub fn translate_instr_address(&self, msr: &MachineStatus, ea: u32) -> u32 {
      self.translate(&self.ibat, msr, ea)
   }

   pub fn translate_data_address(&self, msr: &MachineStatus, ea: u32) -> u32 {
      self.translate(&self.dbat, msr, ea)
   }

   fn translate(&self, bats: &[Bat; 4], msr: &MachineStatus, ea: u32) -> u32 {
      if msr.data_address_translation { // block address translation mode (BAT)

         for x in 0..bats.len() {
            let bat = &bats[x];

            let ea_15   = (ea >> 17) as u16;
            let ea_bepi = (ea_15 & 0x7800) ^ ((ea_15 & 0x7FF) & (!bat.bl));

            if ea_bepi == bat.bepi {

               if (!msr.privilege_level && bat.vs) || (msr.privilege_level && bat.vp) {
                  let upper = (bat.brpn ^ ((ea_15 & 0x7FF) & bat.bl)) as u32;
                  let lower = (ea & 0x1FFFF) as u32;

                  return (upper << 17) ^ lower;
               }

            }
         }

         panic!("FixMe: perform address translation with Segment Register {:#x}", ea);

      } else { // read address translation mode
         ea
      }
   }
}
