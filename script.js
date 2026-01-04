async function loadPage(page, id) {
  return fetch(page)
    .then(response => response.text())
    .then(data => {
      document.getElementById(id).innerHTML = data;
    })
}

async function loadPageAddClass(page, id, id2) {
  fetch(page)
    .then(response => response.text())
    .then(data => {
      document.getElementById(id).innerHTML = data;
    })
    .then(_ => {
      document.getElementById(id2).classList.add("nav-home")
    })
}

async function loadPage(page, id) {
  return fetch(page)
    .then(response => response.text())
    .then(data => {
      document.getElementById(id).innerHTML = data;
    })
}
