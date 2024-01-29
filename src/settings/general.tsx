import { FormControl, FormLabel, Select, Stack } from "@chakra-ui/react";
import { getAllLangs } from "database-plugin";
import { Language } from "database-plugin/webview-dist/types";
import { useState } from "react";

export function SettingsGeneral() {
    const [languages, setLanguages] = useState<Language[]>([]);

    useState(() => {
        getAllLangs().then(result => setLanguages(result)).catch(err => console.error(err));
    });

    return (
        <Stack>
            <FormControl>
                <FormLabel>Language</FormLabel>
                <Select>
                {
                    languages.map((lang, i) => (
                        <option key={i} value={lang.key}>{lang.language}</option>
                    ))
                }
                </Select>
            </FormControl>
        </Stack>
    );
}