import React, {createContext, ReactElement, ReactNode, useState} from "react";

export interface FileContextValue {
    file?: Buffer;
    setFile?: React.Dispatch<React.SetStateAction<Buffer | undefined>>;
}

export const FileContext = createContext<FileContextValue>({});

export const FileContextProvider = ({children}: { children: ReactNode }): ReactElement => {
    console.log("running the file context provider function");
    const [file, setFile] = useState<Buffer>();
    // const numContextValue = useMemo((): FileContextValue => ({file, setFile}), [file]);
    return (
        <FileContext.Provider value={({file, setFile})}>
            {children}
        </FileContext.Provider>
    );
};