use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

// Simplify the struct definitions with this macro:
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[allow(non_snake_case)]
        #[derive(Clone, Deserialize, Serialize)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(User {
    name: String,
    color: String,
});

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

pub_struct!(Book {
    isbn: String,
    title: String,
    authors: Vec<String>,
    imageLinks: ImageLinks,
    publishedDate: String,
    description: String,
    pageCount: u32,
    printType: String,
    categories: Vec<String>,
    ratings: Vec<Rating>,
    comments: Vec<Comment>,
});

pub_struct!(BookListElement {
    isbn: String,
    title: String,
    authors: Vec<String>,
    imageLinks: ImageLinks,
    ratings: Vec<Rating>,
    categories: Vec<String>,
});

pub_struct!(BookListQuery {
    username: Option<String>,
    filter: Option<BookListFilter>,
    sort: Option<BookListSort>,
    direction: Option<SortDirection>,
});

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum BookListFilter {
    Read,
    Unread,
    Both
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum BookListSort {
    Title,
    Author,
    Date,
    Genre
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum SortDirection {
    Up,
    Down
}

pub_struct!(Rating {
    username: String,
    stars: Stars,
});

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone)]
#[repr(u8)]
pub enum Stars {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

pub_struct!(Comment {
    username: String,
    content: String,
});

pub_struct!(ImageLinks {
    smallThumbnail: String,
    thumbnail: String,
});

impl VolumeInfo {
    pub fn into_book(&self, isbn: String) -> Book {
        Book {
            isbn: isbn,
            title: self.title.to_owned(),
            authors: self.authors.to_owned(),
            imageLinks: self.imageLinks.to_owned(),
            publishedDate: self.publishedDate.to_owned(),
            description: self.description.to_owned(),
            pageCount: self.pageCount.to_owned(),
            printType: self.printType.to_owned(),
            categories: self.categories.to_owned(),
            ratings: vec![],
            comments: vec![],
        }
    }
}
