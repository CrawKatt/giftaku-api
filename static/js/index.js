// POST
const form = document.querySelector("#form");
const fileInput = document.querySelector("#file");
const actionSelect = document.querySelector("#action");
const apiUrl = 'https://giftaku-api-production.up.railway.app/api/';

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
        url: apiUrl,
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
        url: apiUrl + selectedAction, // Concatenar la acción a la URL
        method: 'GET',
        responseType: 'json', // important
    })
        .then(function (response) {
            // Limpiar el contenedor antes de agregar nueva información
            const imageContainer = document.getElementById('image-container');
            imageContainer.innerHTML = '';

            // Crear un elemento div para mostrar el JSON
            const jsonContainer = document.createElement('div');
            jsonContainer.classList.add('json-container');

            // Crear un elemento pre para conservar el espacio
            const jsonContent = document.createElement('pre');
            jsonContent.classList.add('json-content');
            jsonContent.textContent = JSON.stringify(response.data, null, 2);

            // Crear el contenedor de GIFs
            const gifContainer = document.getElementById('gif-container');
            const image = document.createElement('img');

            // Agregar la imagen al contenedor de GIFs
            gifContainer.appendChild(image);

            // Agregar el contenido JSON al contenedor
            jsonContainer.appendChild(jsonContent);

            // Agregar el contenedor JSON al contenedor de la imagen
            imageContainer.appendChild(jsonContainer);
        })
        .catch(function (error) {
            console.error("Error fetching data:", error);
        });
});

// VIEW GIF
document.getElementById('view-file').addEventListener('click', function() {
    // Obtener la acción seleccionada desde el selector
    const selectedAction = document.getElementById('action-get').value;

    // Obtener el nombre del archivo desde el contenedor de la imagen
    const imageContainer = document.getElementById('image-container');
    const jsonContent = imageContainer.querySelector('.json-content');
    const jsonData = JSON.parse(jsonContent.textContent);
    const fileName = jsonData.url.split('/').pop();

    axios({
        url: apiUrl + selectedAction + '/' + fileName, // Concatenar la acción y el nombre del archivo a la URL
        method: 'GET',
        responseType: 'blob', // important
    })
    .then(function (response) {
        // Crear un objeto URL para el GIF
        const gifUrl = URL.createObjectURL(response.data);

        // Crear el contenedor de GIFs
        const gifContainer = document.getElementById('gif-container');

        // Crear un elemento de imagen y establecer su atributo src a la URL del GIF
        const image = document.createElement('img');
        image.src = gifUrl;

        // Agregar la imagen al contenedor de la imagen
        gifContainer.appendChild(image);
    })
    .catch(function (error) {
        console.error("Error fetching data:", error);
    });
});

// Botón hamburguesa
document.getElementById('toggle-dashboard').addEventListener('click', function() {
    const dashboardContainer = document.querySelector('.container');
    dashboardContainer.classList.toggle('container-hidden');
});