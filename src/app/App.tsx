import React, {FC} from "react";
import "./App.scss";
import {CustomNavbar} from "./components/navbar/navbar";
import "bootstrap/dist/css/bootstrap.min.css";
import {DxfEditor} from "./components/dxf-editor/dxf-editor";
import { PlaceholderOffsetUI } from "./components/placeholder-offset/placeholder-offset";


const App: FC = () => {
    return (
        <div className={"app-root"}>
            <CustomNavbar/>
            <div style={{flexGrow: 100, width: "100%", position: "relative"}}>
                {/* <DxfEditor/> */}
                <PlaceholderOffsetUI />
            </div>
        </div>
    );
};

export default App;
