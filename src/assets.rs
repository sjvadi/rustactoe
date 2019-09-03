#[derive(PartialEq, Debug)]
pub enum Type {
    Empty,
    Cross,
    Circle
}
pub enum Place {
    A1, A2, A3,
    B1, B2, B3,
    C1, C2, C3,
    NaN
}



pub struct Board {
    pub A1: Type,
    pub A2: Type,
    pub A3: Type,
    pub B1: Type,
    pub B2: Type,
    pub B3: Type,
    pub C1: Type,
    pub C2: Type,
    pub C3: Type
}

pub fn init_board() -> Board{
    let mut nb = Board {
        A1: Type::Empty,
        A2: Type::Empty,
        A3: Type::Empty,
        B1: Type::Empty,
        B2: Type::Empty,
        B3: Type::Empty,
        C1: Type::Empty,
        C2: Type::Empty,
        C3: Type::Empty
    };
    nb
}

impl Board {
    pub fn change(&mut self, p: Place, t: &Type){
        if t == &Type::Cross {
            match p {
                Place::A1 => self.A1 = Type::Cross,
                Place::A2 => self.A2 = Type::Cross,
                _ => println!("LAagstreepje")
            }
        }
    }
}