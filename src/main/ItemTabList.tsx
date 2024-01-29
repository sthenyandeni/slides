import { Tab, TabList, TabPanel, TabPanels, Tabs } from "@chakra-ui/react";
import BookList from "./BookList";

export default function ItemTabList() {
    return (
        <Tabs>
            <TabList>
                <Tab>Books</Tab>
            </TabList>
            <TabPanels>
                <TabPanel height='auto'>
                    <BookList />
                </TabPanel>
            </TabPanels>
        </Tabs>
    )
}