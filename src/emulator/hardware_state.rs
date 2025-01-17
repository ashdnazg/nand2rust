use crate::hardware::{
    Breakpoint, BreakpointVar, Emulator as _, Hardware, Instruction, UWord, RAM,
};

use super::common_state::CommonState;

pub struct HardwareState {
    pub selected_breakpoint: Breakpoint,
    pub hardware: Hardware,
}

impl Default for HardwareState {
    fn default() -> Self {
        let mut hardware = Hardware::default();
        let program: [UWord; 29] = [
            16384, 60432, 16, 58248, 17, 60040, 24576, 64528, 12, 58114, 17, 61064, 17, 64528, 16,
            65000, 58120, 24576, 60560, 16, 62672, 4, 58115, 16384, 60432, 16, 58248, 4, 60039,
        ];
        hardware.load_program(
            program
                .iter()
                .map(|raw| Instruction::new(*raw))
                .collect::<Vec<_>>(),
        );

        HardwareState {
            selected_breakpoint: Breakpoint {
                var: BreakpointVar::A,
                value: 0,
            },
            hardware,
        }
    }
}

impl HardwareState {
    pub fn from_file_contents(contents: &str) -> Self {
        HardwareState {
            selected_breakpoint: Breakpoint {
                var: BreakpointVar::A,
                value: 0,
            },
            hardware: Hardware::from_file_contents(contents),
        }
    }

    pub fn from_hack_file_contents(contents: &str) -> Self {
        HardwareState {
            selected_breakpoint: Breakpoint {
                var: BreakpointVar::A,
                value: 0,
            },
            hardware: Hardware::from_hack_file_contents(contents),
        }
    }
}

impl CommonState for HardwareState {
    fn run(&mut self, step_count: u64) -> bool {
        self.hardware.run(step_count)
    }

    fn ram_mut(&mut self) -> &mut RAM {
        &mut self.hardware.ram
    }

    fn reset(&mut self) {
        self.hardware.reset();
    }
}
