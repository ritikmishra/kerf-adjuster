import React, { useState } from "react";
import { Button } from "react-bootstrap";
import { FilePicker } from "../dxf-editor/dxf-editor";
import "./placeholder-offset.scss";



export const PlaceholderOffsetUI: React.FC = () => {

    const [kerfThickness, setKerfThickness] = useState<number>(0);
    const [file, setFile] = useState<{ file: Buffer, fileName: string } | undefined>(undefined);

    const downloadFile = async () => {
        if (!file) {
            alert("Please select a file");
            return;
        }
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const { offset_drawing } = (await import("kerfadjusterlogic")) as any;
        const new_drawing = Buffer.from(offset_drawing(file, kerfThickness));
        const new_drawing_data_uri = `data:application/dxf;base64,${new_drawing.toString("base64")}`;

        // sort of a hack to automatically download the newly offset dxf
        // make an <a> element with the new DXF as a data URI
        const download_link = document.createElement("a");
        download_link.setAttribute("href", new_drawing_data_uri);

        // make it download the file when opened
        const newFileName = `${file.fileName.replace(/.dxf$/ig, "")}-offset.dxf`;
        download_link.setAttribute("download", newFileName);
        download_link.innerHTML = "Click this link to download the offset DXF";

        if (kerfThickness !== 0 || confirm("It looks like you wanted us to offset the DXF contours by 0 units. Was this intentional?")) {
            // automatically click the link to download the file
            document.body.appendChild(download_link);
            download_link.click();
            document.body.removeChild(download_link);
        }
    };

    return (
        <div className="content">
            <p>
                This is a web application that is supposed to kerf adjust your DXF files for the purposes of laser cutting.

                This is a work in progress. Right now, it is basically unusable, and doesn&apos;t really work. The two main 
                problems are that not all geometries can be offset, and not all geometries will be offset in the right direction.
                Regardless, you can try to put your DXF through it and you can see what comes out.
            </p>
            <div className="dxf-offset-form">
                <FilePicker onNewFile={(file, fileName) => setFile({ file, fileName })} />
                <label>
                    Amount to offset curves by:
                    <input type="number" value={kerfThickness} step="0.001" onChange={(event) => setKerfThickness(Number(event.target.value))}></input>
                </label>
                <Button variant="success" disabled={file === undefined} onClick={downloadFile}>Download offset DXF</Button>
            </div>
            <p>
                Approximate feature roadmap
                <ul>
                    <li>Show DXF to user, and compare original version to offset version</li>
                    <li>Testing on a variety of DXF files and entity types beyond line, circle, arc</li>
                    <li>Explicit error messages if an entity in the DXF file is not supported</li>
                    <li>Allow user to change whether a contour should be offset inwards or outwards</li>
                </ul>
            </p>
        </div>
    );
};
