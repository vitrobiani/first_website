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
    console.log(value + " saved");
    savedValue = value;
    localStorage.setItem('savedValue', value);
}

function loadhtmlSavedValue(id) {
    let savedValue1 = localStorage.getItem('savedValue') || 'empty.html';
    console.log(savedValue1);
    console.log("omer");

    fetch(savedValue1)
    .then(response => response.text())
    .then(data => {
        document.getElementById(id).innerHTML = data;
    })
    .catch(error => console.error('Error fetching the footer:', error));
}
