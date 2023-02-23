async function selectNewUser() {
    window.localStorage.setItem("currentUser", "")
    window.localStorage.setItem("currentUserColor", "")
    window.location.href='/'
}

async function setupUserData (){
    let us = document.getElementById("userSection")
    us.innerHTML = window.localStorage.getItem("currentUser")
    us.style.backgroundColor = window.localStorage.getItem("currentUserColor")
}