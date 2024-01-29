import React from "react";
import ReactDOM from "react-dom/client";
import { ChakraProvider } from "@chakra-ui/react";
import App from "./App";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Settings } from "./settings/settings";

const router = createBrowserRouter([
    {
        path: '/',
        element: <App />
    },
    {
        path: '/settings',
        element: <Settings />
    }
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ChakraProvider>
            <RouterProvider router={router} />
        </ChakraProvider>
    </React.StrictMode>,
);
