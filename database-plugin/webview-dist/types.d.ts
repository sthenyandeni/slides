export declare type ConfigKey = 'default_language';
export declare type Language = {
    /** The two character key for the language (e.g., EN, ZU, etc.) */
    key: string;
    /** The full English name of the language */
    language: string;
};
export interface Title {
    /** The two character key for the language (e.g., EN, ZU, etc.) */
    language: string;
    /** The title of the book in the specified language */
    title: string;
}
interface BookTitle extends Title {
}
export declare type Book = {
    /** The database primary key for the book, always unique. */
    id: number;
    /** The list of all the title translations for the book (see BookTitle) */
    titles: BookTitle[];
};
interface SongTitle extends Title {
}
export declare type SongHeader = {
    /** The database primary key for the song, always unique. */
    id: number;
    /** The sequence of parts in the song, should be parsed */
    sequence: string;
    /** The song number within the book. Can be undefined */
    number?: number;
    /** THe list of all the title translations for the book (see SongTitle) */
    titles: SongTitle[];
};
export declare type SongSection = {
    /** The database primary key for the song section, always unique */
    id: number;
    /** The section key for this section */
    section: string;
    /** The language of this section */
    language: string;
    /** The lyrics that are contained within this section */
    content: string;
};
export declare type Song = SongHeader & {
    sections: SongSection[];
};
export {};
