async function getBookList() {
    const response = await fetch("/api/getBookList")
    books = await response.json()
    let content = ""
    for (const book of books) {
        content += `<button class=\"bookListElement\" onclick=\"openBookPage(\'${book.isbn}\')\">${book.title} by ${book.authors}</button>`
    }
    document.getElementById("contentContainer").innerHTML = content
}

async function main() {
    setupUserData()
    getBookList()
}
if (!userIsSignedIn()) {
    window.location.href='/'
}
document.addEventListener("DOMContentLoaded", main);