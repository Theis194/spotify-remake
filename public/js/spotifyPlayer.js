const invoke = window.__TAURI__.invoke

async function initializeSpotifyPlayer() {
    const token = await invoke("get_auth_token");
    const player = new Spotify.Player({
        name: 'SpotifyBB',
        getOAuthToken: cb => {cb(token);},
        volume: 0.5
    });

    // Error handling
    player.addListener('initialization_error', ({ message }) => { console.error(message); });
    player.addListener('authentication_error', ({ message }) => { console.error(message); });
    player.addListener('account_error', ({ message }) => { console.error(message); });
    player.addListener('playback_error', ({ message }) => { console.error(message); });

    // Playback status updates
    player.addListener('player_state_changed', state => { console.log(state); });

    // Ready
    player.addListener('ready', async ({ device_id }) => {
        console.log('Ready with Device ID', device_id);
        window.spotifyDeviceId = device_id; // Store the device_id globally if needed

        await invoke("set_device_id", { device_id: device_id });
    });

    // Not Ready
    player.addListener('not_ready', ({ device_id }) => {
        console.log('Device ID has gone offline', device_id);
    });
    // Connect to the player!
    player.connect();
};

function play(spotify_uri, device_id, access_token) {
    if (spotify_uri === "") {
        fetch(`https://api.spotify.com/v1/me/player/play?device_id=${device_id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            },
        }).then(response => {
            if (response.ok) {
                console.log('Playback started');
            } else {
                console.error('Playback failed');
            }
        });
    } else {
        fetch(`https://api.spotify.com/v1/me/player/play?device_id=${device_id}`, {
            method: 'PUT',
            body: JSON.stringify({ uris: [spotify_uri] }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            },
        }).then(response => {
            if (response.ok) {
                console.log('Playback started');
            } else {
                console.error('Playback failed');
            }
        });
    }
}

function pause(device_id, access_token) {
    fetch(`https://api.spotify.com/v1/me/player/pause?device_id=${device_id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function next(device_id, access_token) {
    fetch(`https://api.spotify.com/v1/me/player/next?device_id=${device_id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function previous(device_id, access_token) {
    fetch(`https://api.spotify.com/v1/me/player/previous?device_id=${device_id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function shuffle(device_id, access_token, shuffle) {
    fetch(`https://api.spotify.com/v1/me/player/shuffle?device_id=${device_id}&state=${shuffle}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function repeat(device_id, access_token, state) {
    fetch(`https://api.spotify.com/v1/me/player/repeat?device_id=${device_id}&state=${state}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function volume(device_id, access_token, volume) {
    volume = Math.min(100, Math.max(0, volume));
    fetch(`https://api.spotify.com/v1/me/player/volume?device_id=${device_id}&volume_percent=${volume}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        },
    }).then(response => {
        if (response.ok) {
            console.log('Playback started');
        } else {
            console.error('Playback failed');
        }
    });
}

function get_device_id(){
    return window.spotifyDeviceId;
}

window.onSpotifyWebPlaybackSDKReady = initializeSpotifyPlayer;
window.playMusic = play; // Expose the play function globally
window.pauseMusic = pause;
window.nextMusic = next;
window.previousMusic = previous;
window.shuffleMusic = shuffle;
window.repeatMusic = repeat;
window.volumeMusic = volume;
window.getDeviceId = get_device_id;