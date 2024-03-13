use crate::formats::poster::{Poster, Posters};

impl From<Poster> for Posters {
    fn from(value: Poster) -> Self {
        Self {
            title: value.label,
            width: Some(1),
            height: Some(1),
            pages: vec![value.clone()], // Its silly but like i dont know a better way, lmao
        }
    }
}