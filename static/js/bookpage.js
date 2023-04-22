async function deleteBook() {
    if (!window.confirm("Are you sure you want to delete this book?\n(cannot be undone)")) {
        return
    }
    isbn = window.location.toString().split('/').pop()
    fetch("/api/deleteBook", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({isbn: String(isbn)})
    }).then(() => window.location.href = "/")
}

async function rate(stars) {
    isbn = window.location.toString().split('/').pop()
    await fetch(`/api/rateBook/${isbn}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({username: window.localStorage.getItem("currentUser"), stars: stars})
    })
    updateStars()
}

async function updateStars() {
    let isbn = window.location.toString().split('/').pop()
    const response = await fetch(`/api/book/${isbn}`)
    let book = await response.json()
    
    let sum = 0
    let my_rating = 0
    for (rating of book.ratings) {
        sum = sum + rating.stars
        if (rating.username == window.localStorage.getItem("currentUser")) {
            my_rating = rating.stars
        }
    }

    let msg
    if (sum > 0) {
        msg = `Average: ${sum / book.ratings.length} Stars`
    }
    else {
        msg = "No ratings yet."
    }
    

    document.getElementById('avgStars').innerHTML = msg
    out = ""
    for(let i = 1; i <= my_rating; i++) {
        out += `<img id=star${i} class="star" src="../static/images/star_filled.svg" onclick="rate(${i})" />`
    }
    for(let i = 1; i <= 5 - my_rating; i++) {
        out += `<img id=star${my_rating + i} class="star" src="../static/images/star.svg" onclick=rate(${my_rating + i}) />`
    }
    document.getElementById('stars').innerHTML = out
}

async function updateAvgStars() {
    

}

async function main() {
    setupUserData()
    updateStars()
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
    
    comment_section = ""
    for (comment of book.comments) {
        comment_section += "<p>" + comment.content + "<br> - " + comment.username + "</p>"
    }
    document.getElementById('comments').innerHTML = comment_section

}

document.addEventListener("DOMContentLoaded", main);