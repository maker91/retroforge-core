use std::fmt::{Display, Formatter};

pub enum FrameBufferFormat {
    Monochrome,
}

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub format: FrameBufferFormat,
}

pub struct MachineInfo {
    pub name: String,
    pub display: DisplayInfo,
}

#[derive(Debug, Copy, Clone)]
pub enum MachineStepAction {
    Halt,
    Continue,
    WaitForFrame,
}

#[derive(Debug)]
pub enum MachineError {
    UnknownInstruction(String),
    StackOverflow,
    StackUnderflow,
    RegisterIndexOutOfBounds(u8),
    MemoryOutOfBounds(u16),
}

impl Display for MachineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownInstruction(instr) => write!(f, "Unknown instruction: {}", instr),
            Self::StackOverflow => write!(f, "Stack overflow"),
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::RegisterIndexOutOfBounds(reg) => {
                write!(f, "Register index out of bounds: {}", reg)
            }
            Self::MemoryOutOfBounds(addr) => write!(f, "Memory out of bounds: {:04X}", addr),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OutputAction {
    StartBeep,
    StopBeep,
    DrawFrame,
}

#[derive(Debug, Copy, Clone)]
pub enum InputAction {
    Pressed { code: KeyCode },
    Released { code: KeyCode },
}

// TODO: Implement more keycodes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyCode {
    Num1,
    Num2,
    Num3,
    Num4,
    Q,
    W,
    E,
    R,
    A,
    S,
    D,
    F,
    Z,
    X,
    C,
    V,
}

pub trait Machine {
    fn get_info(&self) -> MachineInfo;
    fn load_rom(&mut self, rom: &[u8]) -> Result<(), MachineError>;
    fn step(&mut self) -> Result<MachineStepAction, MachineError>;
    fn get_frame_buffer(&self) -> &[u8];
    fn get_output_actions(&mut self) -> Option<Vec<OutputAction>>;
    fn push_input_actions(&mut self, input_actions: &[InputAction]);
}
