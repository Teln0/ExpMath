const electron = require("electron");
const engine = require("./exp-math-engine");

function createWindow () {
    const win = new electron.BrowserWindow({
        width: 1200,
        height: 720,
        title: "ExpMath Editor",
        webPreferences: {
            nodeIntegration: true
        }
    });

    win.webContents.openDevTools();
    win.loadFile('./web/index.html');
};

electron.ipcMain.handle("create_new_math_stack", (event, data) => {
    let math_stack = engine.create_new_math_stack();
    return math_stack;
});

electron.app.whenReady().then(createWindow);