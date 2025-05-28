import "./App.css";
import Grid from "./components/Grid.tsx";
import { open } from '@tauri-apps/plugin-dialog';
import {useEffect, useReducer, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {ipcCommand} from "./ipc.ts";
import displayStateReducer from "./displayStateReducer.ts";

function App() {
    const [points, setPoints] = useState([])
    const [displayState, dispatchDisplayStateAction] = useReducer(displayStateReducer, {
        dynamicPoints: new Map()
    })

    const onStartClick = async () => {
        ipcCommand.startThreads()
    }
    const onStopClick = async () => {
        ipcCommand.stopAllThreads()
    }

    const openFile = async () => {
        const file = await open({
            multiple: false,
            directory: false,
        });
        console.log(file);
        const config = await ipcCommand.readConfig(file)
        console.log(config)
    }

    useEffect(() => {
        listen('dispatch_display_state_action', (event) => {
            console.log(event );
            dispatchDisplayStateAction(event.payload)
        });
    }, [])

  return (
    <main className="container">
        <button onClick={onStartClick}>Start</button>
        <button onClick={onStopClick}>Stop</button>
        <button onClick={openFile}>Open</button>
      <Grid displayState={displayState} />
    </main>
  );
}

export default App;
