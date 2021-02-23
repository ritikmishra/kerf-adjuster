import React, { ChangeEvent, FC, ReactElement, useState } from "react";
import "./dxf-editor.scss";
import { DxfDisplayArea } from "../dxf-display-area/dxf-display-area";

interface FilePickerProps {
    onNewFile: (fileContents: Buffer) => unknown
}

export const FilePicker: FC<FilePickerProps> = ({ onNewFile }: FilePickerProps) => {
    const handleFileChange = async (event: ChangeEvent<HTMLInputElement>): Promise<void> => {
        const files = event.target.files;
        if (files && files.length > 0) {
            const file = files[0];
            const fileContents = await file.arrayBuffer();
            const fileBuffer = Buffer.alloc(fileContents.byteLength);
            fileBuffer.fill(new Uint8Array(fileContents));
            onNewFile(fileBuffer);
        } else {
            alert("You have to pick a file!");
        }
    };

    return (
        <label>
            Upload a file
            <input type="file" onChange={handleFileChange} accept={".dxf"} />
        </label>
    );
};

export const DxfEditor: FC = (): ReactElement => {

    const [file, setFile] = useState<Buffer | undefined>();

    return (
        <>
            <div className="selector-pane">
                <FilePicker onNewFile={setFile}></FilePicker>
            </div>
            {file ? <DxfDisplayArea fileBuffer={file} /> : <div>No File Selected</div>}
        </>
    );
};
