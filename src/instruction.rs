pub enum Instruction {
    Fill8 {
        offset: u32,
        value: u8,
        length: u32,
    },
    Write16b {
        offset: u32,
        value: u32,
        length: u32,
    },
    Write16p {
        offset: u32,
        value: u32,
        length: u32,
    },
    Write32p {
        offset: u32,
        value: u32,
        length: u32,
    },
    Write32b {
        offset: u32,
        value: u32,
        length: u32,
    },
    Writep {
        offset: u32,
        length: u32,
        data: Vec<u8>,
    },
    Writeb {
        offset: u32,
        length: u32,
        data: Vec<u8>,
    },

    ,
    Fillp <XXXX> 
}

pub fn assemble(instruct: &Instruction) -> Vec<u8> {
    match instruct {
        Fill8 {
            offset: u32,
            value: u8,
            length: u32,
        } => {},
        Write16b {
            offset: u32,
            value: u32,
            length: u32,
        } => {},
    Write16p {
        offset: u32,
        value: u32,
        length: u32,
    } => {},
    Write32p {
        offset: u32,
        value: u32,
        length: u32,
    } => {},
    Write32b {
        offset: u32,
        value: u32,
        length: u32,
    } => {},
    Writep {
        offset: u32,
        length: u32,
        data: Vec<u8>,
    } => {},
    Writeb {
        offset: u32,
        length: u32,
        data: Vec<u8>,
    } => {},
        _ => {
            todo!("Unimplemented Instruction");
        }
    }
}
