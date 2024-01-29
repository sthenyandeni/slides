import { useContext, useEffect, useState } from "react";
import { AppContext } from "../App";
import { Song, SongSection } from "database-plugin/webview-dist/types";
import { getSong } from "database-plugin";
import { Box, Card, CardBody, CardHeader, Heading, Stack, Text } from "@chakra-ui/react";

export function Backstage() {
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
            <h1>No song selected</h1>
            <p>DEBUG: activeSong = {activeSong}</p>
        </>
    )

    const title = getDefaultTitle(song.titles);
    const sectionKeys = song.sequence.split('-');
    const sections = sectionKeys.map(key => song.sections.find(section => section.section === key)).filter(_ => _) as SongSection[];

    return (
        <>
            <h1>Backstage</h1>
            <h2>{song.number ? `${song.number}.` : ''} {title.title}</h2>
            <Stack>
            {
                sections.map((section, i) => (
                    <Card key={i}>
                        <CardHeader>
                            <Heading size="sm">{section.section}</Heading>
                        </CardHeader>
                        <CardBody>
                            <Box>
                                <Text fontSize="md">{section.content}</Text>
                            </Box>
                        </CardBody>
                    </Card>
                ))
            }
            </Stack>
        </>
    );
}