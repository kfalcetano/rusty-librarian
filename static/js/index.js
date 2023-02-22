async function setCurrentUser(e) {
    window.localStorage.setItem("currentUser", e.innerHTML)
    window.localStorage.setItem("currentUserColor", e.style.backgroundColor)
    window.location.href = "/scanner"
}

async function main() {
    if (window.localStorage.getItem("currentUser").length > 0 
        && window.localStorage.getItem("currentUserColor").length > 0) {
            window.location.href='/scanner'
        }
    const response = await fetch("/getUsers")
    let users = await response.json()
    ubunch = document.getElementById('userBunch')
    
    let content = ""
    users.forEach(user => {
        content += `<button class=\"userTile\" style=\"background-color: ${user.color}\" onclick=\"setCurrentUser(this)\">${user.name}</button>`
    });
    ubunch.innerHTML = content
}

main()