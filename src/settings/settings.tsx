import { Tab, TabList, TabPanel, TabPanels, Tabs } from "@chakra-ui/react";
import { SettingsGeneral } from "./general";

export function Settings() {
    return (
        <Tabs>
            <TabList>
                <Tab>General</Tab>
            </TabList>
            <TabPanels>
                <TabPanel><SettingsGeneral /></TabPanel>
            </TabPanels>
        </Tabs>
    );
}