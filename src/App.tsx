import { Grid, GridItem } from "@chakra-ui/react";
import ItemTabList from "./main/ItemTabList";
import React, { useEffect, useState } from "react";
import { Backstage } from "./main/Backstage";
import { Title } from "database-plugin/webview-dist/types";
import { getConfig } from "database-plugin";
import { BackstageSettings } from "./main/BackstageSettings";

const ENGLISH = 'EN';

type AppContext = {
    defaultLanguage: string
    setDefaultLanguage: (_: string) => void,
    getDefaultTitle: <T extends Title>(titles: T[]) => T
    activeSong?: number
    setActiveSong: (_: number) => void
    activeBook?: number
    setActiveBook: (_: number) => void
}

export const AppContext = React.createContext<AppContext>({
    defaultLanguage: ENGLISH,
    setDefaultLanguage: _ => {},
    getDefaultTitle: _ => _[0],
    setActiveSong: _ => {},
    setActiveBook: _ => {}
})

function App() {
    const [defaultLanguage, setDefaultLanguage] = useState<string>(ENGLISH);
    const [activeSong, setActiveSong] = useState<number | undefined>();
    const [activeBook, setActiveBook] = useState<number | undefined>();

    useEffect(() => {
        getConfig('default_language').then(result => setDefaultLanguage(result ?? ENGLISH))
    })

    function getDefaultTitle<T extends Title>(titles: T[]): T {
        const title = titles.find(t => t.language === defaultLanguage);
        if (!title) return titles[0];
        return title;    
    }

    return (
        <AppContext.Provider value={{
            defaultLanguage,
            setDefaultLanguage,
            getDefaultTitle,
            activeSong,
            setActiveSong,
            activeBook,
            setActiveBook
        }}>
            <Grid
                height="100vh"
                minHeight={0}
                templateRows='repeat(2, 1fr)'
                templateColumns='repeat(3, 1fr)'
                gap={2}
            >
                <GridItem rowSpan={1} colSpan={1} overflow='hidden'><ItemTabList /></GridItem>
                <GridItem rowSpan={1} colSpan={1} overflow='scroll'><Backstage /></GridItem>
                <GridItem rowSpan={1} colSpan={1} overflow='scroll' bg='brown'></GridItem>
                <GridItem rowSpan={1} colSpan={1} overflow='scroll' bg='blue'></GridItem>
                <GridItem rowSpan={1} colSpan={1} overflow='scroll'><BackstageSettings /></GridItem>
                <GridItem rowSpan={1} colSpan={1} overflow='scroll' bg='blue'></GridItem>
            </Grid>
        </AppContext.Provider>
    );
}

export default App;
