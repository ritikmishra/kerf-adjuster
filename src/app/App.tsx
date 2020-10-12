import React from "react";
import "./App.css";
import {FileViewingPane} from "./components/file-viewing-pane/file-viewing-pane";
import {CustomNavbar} from "./components/navbar/navbar";
import "bootstrap/dist/css/bootstrap.min.css";


function App() {
    return (
        <div>
            <CustomNavbar/>
            <FileViewingPane/>
        </div>
    );
}

export default App;
