async function selectNewUser() {
    window.localStorage.setItem("currentUser", "")
    window.localStorage.setItem("currentUserColor", "")
    window.location.href='/'
}

async function applyUserData() {
    let us = document.getElementById("userSection")
    us.innerHTML = window.localStorage.getItem("currentUser")
    us.style.backgroundColor = window.localStorage.getItem("currentUserColor")
}

async function openBookPage(isbn) {
    console.log(isbn)
}

async function setupUserData (){
    const response = await fetch("/api/getUsers")
    let users = await response.json()
    for (const user of users) {
        if (user.name == window.localStorage.getItem("currentUser")) {
            applyUserData()
            return
        }
    }
    alert("The user you selected has been deleted :(")
    selectNewUser()
}

function userIsSignedIn() {
    return (Boolean(window.localStorage.getItem("currentUser")) && Boolean(window.localStorage.getItem("currentUserColor")))
}