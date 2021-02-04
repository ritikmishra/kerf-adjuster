import React from "react";
import "./App.scss";
import {CustomNavbar} from "./components/navbar/navbar";
import "bootstrap/dist/css/bootstrap.min.css";
import {DxfEditor} from "./components/file-selector-pane/dxf-editor";


function App() {
    return (
        <>
            <CustomNavbar/>
            <DxfEditor/>
        </>
    );
}

export default App;
