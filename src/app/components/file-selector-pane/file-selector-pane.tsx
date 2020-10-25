import React, {ChangeEvent, ReactElement, useContext} from "react";
import "./file-selector-pane.scss";
import {FileContext} from "../../context/file-context";


export const FileSelectorPane = (): ReactElement => {
    const fileContext = useContext(FileContext);

    const handleFileChange = async (event: ChangeEvent<HTMLInputElement>): Promise<void> => {
        const files = event.target.files;
        if (files && files.length > 0) {
            const file = files[0];
            const fileContents = await file.arrayBuffer();
            const fileBuffer = Buffer.alloc(fileContents.byteLength);
            fileBuffer.fill(new Uint8Array(fileContents));

            if (fileContext.setFile) {
                fileContext.setFile(fileBuffer);
            }
        }
    };


    return (
        <div className="selector-pane">
            <label>
                Upload a file
                <input type="file" onChange={handleFileChange}/>
            </label>
        </div>
    );
};