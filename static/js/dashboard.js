async function main() {
    setupUserData()
}
if (window.localStorage.getItem("currentUser").length < 1 
|| window.localStorage.getItem("currentUserColor").length < 1) {
    window.location.href='/'
}
document.addEventListener("DOMContentLoaded", main);