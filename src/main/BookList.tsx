import { List, ListItem, Select, Stack } from "@chakra-ui/react";
import React, { useContext, useEffect, useState } from "react";
import { getAllBooks, getBookSongs } from "database-plugin";
import { Book, SongHeader } from "database-plugin/webview-dist/types";
import { AppContext } from "../App";
import { isNumeric } from "../utility";

/* 
 * List height for scrollability
 * 50vh --> 50% of the screen, which the size of a single grid block
 * 7em  --> The "approximate" size of all the elements on top of the list as an offset
 */
const CSS_LIST_HEIGHT = 'calc(50vh - 7em)';

export default function BookList() {
    const context = useContext(AppContext);
    const {getDefaultTitle} = context;
    const {activeSong, setActiveSong} = context;
    const {activeBook, setActiveBook} = context;
    const [books, setBooks] = useState<Book[]>([]);
    const [songs, setSongs] = useState<SongHeader[]>([]);

    useEffect(() => {
        getAllBooks().then(result => {
            setBooks(result);
            if (result) setActiveBook(result[0].id);
        }).catch(err => console.error(err));
    }, []);

    useEffect(() => {
        if (!activeBook) return;
        getBookSongs(activeBook).then(result => setSongs(result)).catch(err => console.error(err));
    }, [activeBook]);

    function changeBook(e: React.ChangeEvent<HTMLSelectElement>) {
        let value = e.target.value;
        if (!isNumeric(value))
            throw "There was an error changing the book";

        const bookId = Number(value);
        setActiveBook(bookId);
    }

    function songClick(id: number) {
        setActiveSong(id);
    }

    return (
        <Stack >
            <Select size="sm" onChange={changeBook}>
            {
                books.map((book, i) => {
                    const title = getDefaultTitle(book.titles);
                    if (!title) throw 'Default value not found';
                    return <option key={i} value={book.id}>{title.title}</option>
                })
            }
            </Select>
            <List spacing={0} overflowY="scroll" height={CSS_LIST_HEIGHT}>
            {
                songs.map((song, i) => {
                    console.log(song.titles);
                    const title = getDefaultTitle(song.titles);
                    return (
                    <ListItem
                        key={i}
                        onClick={songClick.bind(null, song.id)}
                        bg={song.id === activeSong ? 'lightgreen' : undefined}
                    >
                        {song.number !== undefined ? `${song.number}. ${title.title}` : title.title}
                    </ListItem>)
                })
            }
            </List>
        </Stack>
    );
}