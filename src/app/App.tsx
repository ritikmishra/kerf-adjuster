import React, {FC} from "react";
import "./App.scss";
import {CustomNavbar} from "./components/navbar/navbar";
import "bootstrap/dist/css/bootstrap.min.css";
import {DxfEditor} from "./components/dxf-editor/dxf-editor";


const App: FC = () => {
    return (
        <div className={"app-root"}>
            <CustomNavbar/>
            <div style={{flexGrow: 100, width: "100%", position: "relative"}}>
                <DxfEditor/>
            </div>
        </div>
    );
};

export default App;
