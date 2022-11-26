// pub enum SleepQuality {
//     Good = 1,
//     Bad = 2,
//     Ok = 3,
// }

// impl SleepQuality {
//     pub fn from_u8(q: u8) -> Option<SleepQuality> {
//         match q {
//             1 => Some(SleepQuality::Good),
//             2 => Some(SleepQuality::Bad),
//             3 => Some(SleepQuality::Ok),
//             _ => None
//         }
//     }
// }

pub struct DBSleep {
    pub id: i64,
    pub night: String,
    pub amount: f64,
    pub quality: i64,
}

pub struct DBTag {
    pub id: i64,
    pub name: String,
    pub color: i64,
}

pub struct DBSleepTags {
    pub id: i64,
    pub sleep_id: i64,
    pub tag_id: i64,
}

// pub struct Color(pub u8, pub u8, pub u8);

// impl Color {
//     pub fn to_decimal(&self) -> usize {
//         ((self.0 as usize) << 16) + ((self.1 as usize) << 8) + (self.2 as usize)
//     }

//     pub fn from_decimal(color: usize) -> Color {
//         let r = ((color & 0xff0000) >> 16) as u8;
//         let g = ((color & 0x00ff00) >> 8) as u8;
//         let b = (color & 0x0000ff) as u8;
//         Color(r,g,b)
//     }
// }
// pub struct Date(pub u16, pub u8, pub u8);

// impl Date {
//     pub fn to_db_string(&self) -> String {
//         format!("{}-{}-{}", self.0, self.1, self.2)
//     }

//     pub fn from_db_string(date: &str) -> Date {
//         let ymd: Vec<&str> = date.split("-").collect();
//         let y: u16 = ymd[0].parse().unwrap();
//         let m: u8 = ymd[1].parse().unwrap();
//         let d: u8 = ymd[2].parse().unwrap();

//         Date(y,m,d)
//     }
// }