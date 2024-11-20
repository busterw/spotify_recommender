document.getElementById('recommendation-form').addEventListener('submit', async function (event) {
    event.preventDefault();

    const mood = document.getElementById('mood').value;
    const genre = document.getElementById('genre').value;

    const response = await fetch(`/recommendations?mood=${mood}&genre=${genre}`);
    const tracks = await response.json();

    const tracksList = document.getElementById('tracks');
    tracksList.innerHTML = '';

    tracks.forEach(track => {
        const trackDiv = document.createElement('div');
        trackDiv.classList.add('track');

        // Get album art or use a placeholder
        const albumArt = track.album.images[0]?.url || 'default-placeholder.png';

        // Add album art, track name, artists, and a link to Spotify
        trackDiv.innerHTML = `
            <img src="${albumArt}" alt="Album Art" class="album-art">
            <h3>
                <a href="${track.external_urls.spotify}" target="_blank">${track.name}</a>
            </h3>
            <p>${track.artists.map(artist => artist.name).join(', ')}</p>
        `;

        tracksList.appendChild(trackDiv);
    });
});
