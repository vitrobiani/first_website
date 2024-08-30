function loadhtml(filename, id) {
    fetch(filename)
    .then(response => response.text())
    .then(data => {
        document.getElementById(id).innerHTML = data;
    })
    .catch(error => console.error('Error fetching the footer:', error));
}

let savedValue = 'empty.html';

function saveValue(value) {
    savedValue = value;
    localStorage.setItem('savedValue', value);
    loadhtmlSavedValue("proj");
}

function loadhtmlSavedValue(id) {
    let savedValue1 = localStorage.getItem('savedValue') || 'empty.html';
    console.log(savedValue1);

    fetch(savedValue1)
    .then(response => response.text())
    .then(data => {
        document.getElementById(id).innerHTML = data;
    })
    .catch(error => console.error('Error fetching the footer:', error));
}
