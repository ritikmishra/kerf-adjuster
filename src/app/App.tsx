import React from "react";
import "./App.scss";
import {FileViewingPane} from "./components/file-viewing-pane/file-viewing-pane";
import {CustomNavbar} from "./components/navbar/navbar";
import "bootstrap/dist/css/bootstrap.min.css";
import {FileContextProvider} from "./context/file-context";
import {FileSelectorPane} from "./components/file-selector-pane/file-selector-pane";


function App() {
    return (
        <FileContextProvider>
            <div>
                <div className={"pane-accessories"}>
                    <CustomNavbar/>
                    <FileSelectorPane/>
                </div>
                <FileViewingPane/>
            </div>
        </FileContextProvider>
    );
}

export default App;
