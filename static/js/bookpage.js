async function main() {
    isbn = window.location.toString().split('/').pop()
    const response = await fetch(`/api/book/${isbn}`)
    book = await response.json()
    cont = document.getElementById('contentContainer')
    document.getElementById('output').innerHTML = "<h2>" + book.title + "</h2>"
        + "<b>Author: </b>" + book.authors[0] + "<br>"
        + "<b>Genre: </b>" + book.categories[0] + "<br>"
        + "<b>Published: </b>" + book.publishedDate
    document.getElementById('thumb').src = book.imageLinks.thumbnail
    document.getElementById('description').innerHTML = book.description
}

document.addEventListener("DOMContentLoaded", main);