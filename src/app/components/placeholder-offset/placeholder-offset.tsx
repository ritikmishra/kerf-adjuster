import React, { useState } from "react";
import { FilePicker } from "../dxf-editor/dxf-editor";
import "./placeholder-offset.scss";


// TODO: delete this testing logic
import("kerfadjusterlogic").then((mod) => {
    console.log("module successfully imported;");
    console.log("let's multiply two nums");
    console.log(mod.multiply_nums(1, 3));
    console.log(mod.offset_drawing(Buffer.from("not dxf"), 10));
});

export const PlaceholderOffsetUI: React.FC = () => {

    const [kerfThickness, setKerfThickness] = useState<number>(0);

    const handleNewFile = async (file: Buffer) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const { offset_drawing } = (await import("kerfadjusterlogic")) as any;
        const new_drawing = Buffer.from(offset_drawing(file, kerfThickness));
        const new_drawing_data_uri = `data:application/dxf;base64,${new_drawing.toString("base64")}`;

        // sort of a hack to automatically download the newly offset dxf
        // make an <a> element with the new DXF as a data URI
        const download_link = document.createElement("a");
        download_link.setAttribute("href", new_drawing_data_uri);
        
        // make it download the file when opened
        download_link.setAttribute("download", "offset.dxf");
        download_link.innerHTML = "Click this link to download the offset DXF";
        
        // automatically click the link to download the file
        document.body.appendChild(download_link);
        download_link.click();
        document.body.removeChild(download_link);
    };

    return (
        <div className="content">
            <p>
                This is a web application that is supposed to kerf adjust your DXF files for the purposes of laser cutting.

                This is a work in progress. Right now, it is basically unusable, and doesn&apos;t really work. Regardless, you 
                can try to put your DXF through it and you can see what comes out.
            </p>
            <div className="centered">
                <FilePicker onNewFile={handleNewFile} />
                <br/>
                <label>
                    Amount to offset curves by
                    <input type="number" value={kerfThickness} onChange={(event) => setKerfThickness(Number(event.target.value))}></input>
                </label>
            </div>
            <p>
                Approximate feature roadmap
                <ul>
                    <li>Show DXF to user, and compare original version to offset version</li>
                    <li>Explicit error messages if an entity in the DXF file is not supported</li>
                    <li>Allow user to change whether a contour should be offset inwards or outwards</li>
                </ul>
            </p>
        </div>
    );
};
