use crate::formats::poster::{Poster, Posters};

impl From<Poster> for Posters {
    fn from(value: Poster) -> Self {
        Self {
            title: value.clone().label, // Its silly but like i dont know a better way, lmao
            width: Some(1),
            height: Some(1),
            pages: vec![value],
        }
    }
}