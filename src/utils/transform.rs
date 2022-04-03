// use bevy::prelude::{Transform};

// pub trait Math { 
//     fn angle_between(&self, b: Transform) -> f32;
//     fn distance(&self, b: Transform) -> f32;
// }

// impl Math for Transform {
//     fn angle_between(&self, b: Transform) -> f32 {
//         return self.translation.angle_between(b.translation);

//         // Apparently vec3's already have this :)
//         // Yay, for me not knowing this and writing all this code
//         // let (x2, y2) = (b.translation.x, b.translation.y);
//         // let (x1, y1) = (self.translation.x, self.translation.y);
//         // let y = y2 - y1;
//         // let x = x2 - x1;

//         // return y.atan2(x);
//     }

//     fn distance(&self, b: Transform) -> f32 {
//         return self.translation.distance(b.translation);

//         // Yep they had this one too, cool, cool, cool.
//         // let (x1, y1) = (self.translation.x, self.translation.y);
//         // let (x2, y2) = (b.translation.x, b.translation.y);

//         // return ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
//     }
// }