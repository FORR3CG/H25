use std::fmt::Display;

pub struct Soluvara {
    id: u32,
    verd: u32,
}

impl Soluvara {
    pub fn new(id: u32, verd: u32) -> Self {
        Self { id, verd }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn verd(&self) -> u32 {
        self.verd
    }

    pub fn set_verd(&mut self, verd: u32) {
        self.verd = verd
    }
}

impl TryFrom<&str> for Soluvara {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 2 {
            Err("Ekki nægur fjöldi talna til að búa til söluvöru!".to_string())
        } else if let Ok(id) = lidir[0].parse::<u32>() {
            if let Ok(verd) = lidir[1].parse::<u32>() {
                Ok(Self {id, verd})
            } else {
                Err(format!("Gat ekki búið til verð úr '{}'!", lidir[1]))
            }
        } else {
            Err(format!("Gat ekki búið til id úr '{}'!", lidir[0]))
        }
    }
}

impl Display for Soluvara {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, verð: {} kr.", self.id, self.verd)
    }
}