async function getBookList(params) {
    let username = window.localStorage.getItem("currentUser")
    const response = await fetch(`/api/getBookList?username=${username}&${params}`)
    let books = await response.json()
    let content = ""
    for (const book of books) {
        let read = book.ratings.find(rating => rating.username == username) ? "" : "hidden"
        content += `<button class=\"bookListElement\" onclick=\"openBookPage(\'${book.isbn}\')\">
                        <div class="listDetails">
                            <img class="thumb" src=${book.imageLinks.smallThumbnail}/>
                            <div>
                                <div class="title">${book.title}</div>
                                <div class="author">${book.authors} - ${book.categories[0]}</div>
                            </div>
                        </div>
                        <img class="check" src="../static/images/check.svg" ${read}/>
                    </button>`
    }
    if (books.length == 0) {
        content = "<p>No books found.</p><p>Scan one to add to the library or broaden the filter.</p>"
    }
    document.getElementById("contentContainer").innerHTML = content
}

async function queryChanged() {
    let query = new URLSearchParams({
        filter: selVal(document.getElementById("filter")),
        sort: selVal(document.getElementById("sort")),
        direction: selVal(document.getElementById("direction"))
    })
    console.log(query.toString())
    getBookList(query.toString())
}

function selVal(sortItem) {
    return sortItem[sortItem.selectedIndex].value
}

async function main() {
    setupUserData()
    getBookList("")
}
if (!userIsSignedIn()) {
    window.location.href='/'
}
window.addEventListener( "pageshow", function ( event ) {
    var historyTraversal = event.persisted || 
                           ( typeof window.performance != "undefined" && 
                                window.performance.getEntriesByType("navigation") === 2 );
    if ( historyTraversal ) {
      // Handle page restore.
      window.location.reload();
    }
  });
document.addEventListener("DOMContentLoaded", main);