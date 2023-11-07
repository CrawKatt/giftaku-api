//const axios = require('axios');

// POST
const form = document.querySelector("#form");
const fileInput = document.querySelector("#file");

form.addEventListener("submit", (e) => {
    e.preventDefault();

    const file = fileInput.files[0];
    axios({
        method: "post",
        url: "http://localhost:8000/",
        data: file,
        headers: { "Content-Type": file.type },
    })
        .then(response => console.log(response.data))
        .catch((e) => console.error(e));
});

// GET
document.getElementById('get-file').addEventListener('click', function() {
    const fileName = document.getElementById('file-name').value;

    axios({
        url: 'http://localhost:8000/api/' + fileName,
        method: 'GET',
        responseType: 'blob', // important
    })
        .then(function (response) {
            const url = window.URL.createObjectURL(new Blob([response.data]));
            const img = document.createElement('img');
            img.src = url;
            document.getElementById('image-container').appendChild(img);
        });
});