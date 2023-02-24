async function getBookList() {
    const response = await fetch("/getBookList")
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

if (window.localStorage.getItem("currentUser").length < 1 
|| window.localStorage.getItem("currentUserColor").length < 1) {
    window.location.href='/'
}
document.addEventListener("DOMContentLoaded", main);