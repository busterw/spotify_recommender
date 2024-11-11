document.getElementById('recommendation-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    const mood = document.getElementById('mood').value;
    const genre = document.getElementById('genre').value;
    const response = await fetch(`/recommendations?mood=${mood}&genre=${genre}`);
    const tracks = await response.json();
   
    const tracksList = document.getElementById('tracks');
    tracksList.innerHTML = '';
    tracks.forEach(track => {
        const listItem = document.createElement('li');
        listItem.textContent = `${track.name} by ${track.artists.map(artist => artist.name).join(', ')}`;
        tracksList.appendChild(listItem);
    });
});