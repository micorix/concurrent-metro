import * as tauri from '@tauri-apps/api'

export const IPC_COMMAND_NAME = {
    READ_CONFIG: 'read_config',
    START_THREADS: 'start_threads',
    STOP_ALL_THREADS: 'stop_all_threads'
} as const;

export const ipcCommand = {
    readConfig: (filePath: string) => tauri.core.invoke(IPC_COMMAND_NAME.READ_CONFIG, { filePath }),
    startThreads: () => tauri.core.invoke(IPC_COMMAND_NAME.START_THREADS),
    stopAllThreads: () => tauri.core.invoke(IPC_COMMAND_NAME.STOP_ALL_THREADS),
} as const;