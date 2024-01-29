import { Book, ConfigKey, Language, Song, SongHeader } from './types';
export declare function getAllLangs(): Promise<Language[]>;
export declare function getLang(key: string): Promise<Language | undefined>;
export declare function getAllBooks(): Promise<Book[]>;
export declare function getBookSongs(bookId: number): Promise<SongHeader[]>;
export declare function getSong(songId: number): Promise<Song | undefined>;
export declare function getConfig(key: ConfigKey): Promise<string | undefined>;
export declare function setConfig(key: ConfigKey, value: string): Promise<void>;
