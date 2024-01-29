import { useContext, useEffect, useState } from "react";
import { AppContext } from "../App";
import { Song } from "database-plugin/webview-dist/types";
import { getSong } from "database-plugin";
import { Tab, TabList, TabPanel, TabPanels, Tabs, Text } from "@chakra-ui/react";

export function BackstageSettings() {
    const {activeSong, getDefaultTitle} = useContext(AppContext);
    const [song, setSong] = useState<Song | undefined>();
    
    useEffect(() => {
        if (!activeSong) {
            setSong(undefined);
            return;
        }

        getSong(activeSong).then(result => setSong(result)).catch(err => console.error(err));
    }, [activeSong]);

    if (!song) return (
        <>
            <Text fontSize="xl">No song set</Text>
        </>
    );

    const title = getDefaultTitle(song.titles);

    return (
        <Tabs>
            <TabList>
                <Tab>Lyrics</Tab>
            </TabList>
            <TabPanels>
                <TabPanel></TabPanel>
            </TabPanels>
        </Tabs>
    )
}