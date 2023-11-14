// POST
const form = document.querySelector("#form");
const fileInput = document.querySelector("#file");
const actionSelect = document.querySelector("#action");

form.addEventListener("submit", (e) => {
    e.preventDefault();

    const file = fileInput.files[0];
    const action = actionSelect.value;
    const animeName = document.querySelector("#anime_name").value;
    const formData = new FormData();
    formData.append("file", file);
    formData.append("action", action);
    formData.append("anime_name", animeName);

    axios({
        method: "post",
        url: "http://localhost:8000/",
        data: formData,
        headers: { "Content-Type": "multipart/form-data" },
    })
        .then(response => console.log(response.data))
        .catch((e) => console.error(e));
});

// GET
document.getElementById('get-file').addEventListener('click', function() {
    const selectedAction = document.getElementById('action-get').value; // Obtener la acción seleccionada desde el selector

    axios({
        url: 'http://localhost:8000/api/' + selectedAction, // Concatenar la acción a la URL
        method: 'GET',
        responseType: 'json', // important
    })
        .then(function (response) {
            const responseContainer = document.createElement('pre');
            responseContainer.textContent = JSON.stringify(response.data, null, 2);
            document.getElementById('image-container').appendChild(responseContainer);
        });
});