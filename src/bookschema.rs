use serde::{Deserialize, Serialize};

// Simplify the struct definitions with this macro:
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[allow(non_snake_case)]
        #[derive(Deserialize, Serialize)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(BookId {
    isbn: String,
});

pub_struct!(Volumes {
    kind: String,
    totalItems: isize,
    items: Vec<Item>,
});

pub_struct!(Item {
    id: String,
    volumeInfo: VolumeInfo,
});

pub_struct!(VolumeInfo {
    title: String,
    authors: Vec<String>,
    imageLinks: ImageLinks,
    publishedDate: String,
    description: String,
    pageCount: u32,
    printType: String,
    categories: Vec<String>,
});

pub_struct!(ImageLinks {
    smallThumbnail: String,
    thumbnail: String,
});