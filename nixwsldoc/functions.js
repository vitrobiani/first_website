
// Toggle sidebar open/close
function toggleSidebar() {
    const sidebar = document.getElementById('sidebar');
    const mainContent = document.getElementById('main-content');
    sidebar.classList.toggle('active');
    mainContent.classList.toggle('shifted');

    var menuIcon = document.querySelector('.menu-icon');
    menuIcon.classList.toggle('change');
}

// Toggle the visibility of submenu items
function toggleSubMenu(subMenuId) {
    const subMenu = document.getElementById(subMenuId);
    subMenu.style.display = subMenu.style.display === 'none' ? 'block' : 'none';
}

function filterItems() {
    const query = document.getElementById('search-input').value.toLowerCase();
    const items = document.querySelectorAll('.subitem');

    items.forEach(item => {
        const text = item.textContent.toLowerCase();
        item.style.display = text.includes(query) ? '' : 'none';
    });
}

let nixDocSavedValue = 'empty.html';

function saveValue(value) {
    nixDocSavedValue = value;
    localStorage.setItem('nixDocSavedValue', value);
    loadhtmlSavedValue("main-content");
}

function loadhtmlSavedValue(id) {
    let savedValue1 = localStorage.getItem('nixDocSavedValue') || 'empty.html';
    console.log(savedValue1);

    fetch(savedValue1)
    .then(response => response.text())
    .then(data => {
        document.getElementById(id).innerHTML = data;
    })
    .catch(error => console.error('Error fetching the footer:', error));
}