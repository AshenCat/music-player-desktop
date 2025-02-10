'use client';

import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import styles from './MusicPlayer.module.css';

interface Song {
    title: string;
}

const MusicPlayer: React.FC = () => {
    const [songs, setSongs] = useState<Song[]>([]);
    const [currentSong, setCurrentSong] = useState<Song | null>(null);
    const [isPlaying, setIsPlaying] = useState(false);
    const [volume, setVolume] = useState(100);

    useEffect(() => {
        async function fetchSong() {
            const songList: Song[] = await invoke('get_songs');
            setSongs(songList);
        }
        fetchSong();
    }, []);

    const playSong = async (song: Song) => {
        await invoke('play_song', { title: song.title });
        setCurrentSong(song);
        setIsPlaying(true);
    };

    const pauseSong = async () => {
        await invoke('pause_song');
        setIsPlaying(false);
    };

    const changeVolume = async (event: React.ChangeEvent<HTMLInputElement>) => {
        const newVolume: number = parseInt(event.target.value, 10);
        const normalizedVolume: number = newVolume / 100;
        setVolume(normalizedVolume);

        await invoke('set_volume', { vol: normalizedVolume });
    };

    return (
        <div className={styles.container}>
            <div className={styles.songList}>
                <h1>Music Player</h1>
                <ul>
                    {songs.map((song: Song) => (
                        <li key={song.title} onClick={() => playSong(song)}>
                            <span>{song.title}</span>
                            {currentSong?.title === song.title && isPlaying && (
                                <span>...</span>
                            )}
                        </li>
                    ))}
                </ul>
            </div>
            {currentSong && (
                <div className={styles.controls}>
                    <h2>Now Playing: {currentSong.title}</h2>
                    {isPlaying ? (
                        <button onClick={() => pauseSong()}>Pause</button>
                    ) : (
                        <button onClick={() => playSong(currentSong)}>
                            Play
                        </button>
                    )}

                    <input
                        type="range"
                        min="0"
                        max="100"
                        value={volume}
                        onChange={changeVolume}
                    />
                </div>
            )}
        </div>
    );
};

export default MusicPlayer;
