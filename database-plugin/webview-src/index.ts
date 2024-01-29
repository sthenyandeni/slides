import { invoke } from '@tauri-apps/api/tauri'
import { Book, ConfigKey, Language, Song, SongHeader } from './types'

export async function getAllLangs(): Promise<Language[]> {
    const result = await invoke<Language[]>('plugin:database|get_all_langs');
    return result;
}

export async function getLang(key: string): Promise<Language | undefined> {
    const result = await invoke<Language | undefined>('plugin:database|get_lang', {key});
    return result;
}

export async function getAllBooks(): Promise<Book[]> {
    const result = await invoke<Book[]>('plugin:database|get_all_books');
    return result;
}

export async function getBookSongs(bookId: number): Promise<SongHeader[]> {
    const result = await invoke<SongHeader[]>('plugin:database|get_book_songs', {bookId});
    return result;
}

export async function getSong(songId: number): Promise<Song | undefined> {
    const result = await invoke<Song | undefined>('plugin:database|get_song', {songId});
    return result
}

export async function getConfig(key: ConfigKey): Promise<string | undefined> {
    const result = await invoke<string | undefined>('plugin:database|get_config', {key});
    return result;
}

export async function setConfig(key: ConfigKey, value: string) {
    await invoke('plugin:database|set_config', {key, value})
}