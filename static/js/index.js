function openModal(id) {
    document.getElementById(id).classList.add('open');
    document.body.classList.add('modal-open');
    document.getElementById("uname").focus()
}

function closeModal() {
    document.querySelector('.modal.open').classList.remove('open');
    document.body.classList.remove('modal-open');
    document.getElementById("uname").value = ""
    deselectSwatch()
}

async function addUser() {
    let name = document.getElementById("uname")
    if (name.value.length < 1) { 
        alert("Please enter a name")
        return 
    }
    let swatch = document.querySelector(".selected-swatch")
    if (!swatch){ 
        alert("Please select a color")
        return 
    }
    res = fetch("/addUser", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({name: String(name.value), color: String(swatch.style.backgroundColor)})
    })
    closeModal()
    const response = await res
    if (!response.ok) {
        alert(await response.text())
        name.select()
        return
    }
    await fetchUsers()
}

async function setCurrentUser(e) {
    window.localStorage.setItem("currentUser", e.innerHTML)
    window.localStorage.setItem("currentUserColor", e.style.backgroundColor)
    window.location.href = "/dashboard"
}

async function fetchUsers() {
    const response = await fetch("/getUsers")
    let users = await response.json()
    ubunch = document.getElementById('userBunch')
    let content = ""
    users.forEach(user => {
        content += `<button class=\"userTile\" style=\"background-color: ${user.color}\" onclick=\"setCurrentUser(this)\">${user.name}</button>`
    });
    ubunch.innerHTML = content
}

async function deselectSwatch() {
    let sel = document.querySelector(".selected-swatch")
    if(sel) {sel.classList.remove("selected-swatch")}
}

async function main() {
    document.addEventListener('click', event => {
        if (event.target.classList.contains('modal')) {
            closeModal();
        }
    });

    // color swatch on modals
    let swatches = document.getElementsByClassName("swatch")
    for (var i = 0; i < swatches.length; i++) {
        swatches[i].addEventListener("click", e => {
            deselectSwatch()
            e.target.classList.add("selected-swatch");
        })
        
    }
}

if (window.localStorage.getItem("currentUser").length > 0 
        && window.localStorage.getItem("currentUserColor").length > 0) {
            window.location.href='/dashboard'
    }
fetchUsers()

window.addEventListener('DOMContentLoaded', main)
